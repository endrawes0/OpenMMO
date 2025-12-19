mod accounts;
mod db;
mod entities;
mod equipment;
mod inventory;
mod items;
mod loot;
mod network;
mod simulation;
mod world;

use crate::network::messages::Envelope;
use crate::simulation::tick_loop::build_world_snapshot;
use axum::{
    extract::{State, WebSocketUpgrade},
    http::StatusCode,
    response::{Json, Response},
    routing::get,
    Router,
};
use db::conversions::{CharacterWireView, ConversionError};
use serde_json::json;
use tokio::time::{interval, Duration};
use tracing::{error, info, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

#[derive(Clone)]
struct AppState {
    db_pool: sqlx::PgPool,
    session_store: network::SessionStore,
    world_state: std::sync::Arc<tokio::sync::RwLock<world::WorldState>>,
    account_service: accounts::AccountService,
}

async fn persist_active_positions(state: &AppState) {
    let sessions = state.session_store.get_active_sessions().await;
    let world = state.world_state.read().await;

    for session in sessions {
        if let (Some(player_id), Some(character_id)) = (session.player_id, session.character_id) {
            if let Some((x, y, z, rot)) = world.get_player_pose(player_id) {
                if let Err(e) = state
                    .account_service
                    .update_character_position(
                        character_id,
                        x as f64,
                        y as f64,
                        z as f64,
                        rot as f64,
                    )
                    .await
                {
                    warn!("Periodic save failed for session {}: {:?}", session.id, e);
                } else {
                    info!(
                        "Periodic save for character {} (session {}): ({:.2}, {:.2}, {:.2}) rot {:.2}",
                        character_id, session.id, x, y, z, rot
                    );
                }
            }
        }
    }
}

struct EnvLoadResult {
    path: Option<std::path::PathBuf>,
    warnings: Vec<String>,
}

fn load_env_file() -> anyhow::Result<EnvLoadResult> {
    let mut current_dir = std::env::current_dir()?;

    loop {
        let candidate = current_dir.join(".env");
        if candidate.exists() {
            let contents = std::fs::read_to_string(&candidate)?;
            let mut warnings = Vec::new();

            for (idx, raw_line) in contents.lines().enumerate() {
                let line = raw_line.trim();
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }

                let line = line.strip_prefix("export ").unwrap_or(line);
                let (key, value) = match line.split_once('=') {
                    Some(parts) => parts,
                    None => {
                        warnings.push(format!(
                            "Ignoring malformed line {}: '{}'",
                            idx + 1,
                            raw_line
                        ));
                        continue;
                    }
                };

                let key = key.trim();
                if key.is_empty() {
                    warnings.push(format!("Ignoring empty key on line {}", idx + 1));
                    continue;
                }

                if std::env::var_os(key).is_some() {
                    continue;
                }

                let value = value.trim().trim_matches('"').trim_matches('\'');
                std::env::set_var(key, value);
            }

            return Ok(EnvLoadResult {
                path: Some(candidate),
                warnings,
            });
        }

        if !current_dir.pop() {
            break;
        }
    }

    Ok(EnvLoadResult {
        path: None,
        warnings: Vec::new(),
    })
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let env_load = load_env_file()?;

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "openmmo=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    if let Some(path) = env_load.path {
        info!("Loaded environment from {}", path.display());
    } else {
        info!("No .env file found; relying on process environment");
    }

    for warning in env_load.warnings {
        warn!("{warning}");
    }

    // Load database URL from environment
    let database_url = std::env::var("DATABASE_URL")
        .map_err(|_| anyhow::anyhow!("DATABASE_URL environment variable must be set"))?;

    info!("Connecting to database...");

    // Create database connection pool using our db module
    let db_pool = db::create_pool(&database_url)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to connect to database: {}", e))?;

    info!("Successfully connected to database");

    // Test database connectivity
    db::check_connection(&db_pool)
        .await
        .map_err(|e| anyhow::anyhow!("Database connectivity test failed: {}", e))?;

    info!("Database connectivity verified");

    // Create world state
    let world_state = std::sync::Arc::new(tokio::sync::RwLock::new(world::WorldState::new()));
    info!(
        "World state initialized with {} zones",
        world_state.read().await.zone_count()
    );

    // Create application state
    let session_store = network::SessionStore::new();
    let account_service = accounts::AccountService::new(db_pool.clone());
    let state = AppState {
        db_pool,
        session_store,
        world_state: world_state.clone(),
        account_service,
    };

    // Start simulation loop in background
    let simulation_world_state = world_state.clone();
    let simulation_session_store = state.session_store.clone();
    tokio::spawn(async move {
        let mut simulation_loop =
            simulation::SimulationLoop::new(simulation_world_state, simulation_session_store);
        simulation_loop.run().await;
    });

    // Periodically persist active player positions
    let state_for_persist = state.clone();
    tokio::spawn(async move {
        let mut ticker = interval(Duration::from_secs(5));
        loop {
            ticker.tick().await;
            persist_active_positions(&state_for_persist).await;
        }
    });

    // Build our application with routes
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/health/db", get(database_health_check))
        .route("/ws", get(ws_handler))
        .with_state(state);

    // Run the server
    let server_host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let server_port: u16 = std::env::var("SERVER_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .unwrap_or(8080);

    let addr_str = format!("{}:{}", server_host, server_port);
    let addr: std::net::SocketAddr = addr_str
        .parse()
        .map_err(|e| anyhow::anyhow!("Invalid server address: {}", e))?;

    info!("OpenMMO server listening on {}", addr_str);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}

async fn database_health_check(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    match db::check_connection(&state.db_pool).await {
        Ok(_) => Ok(Json(json!({
            "status": "healthy",
            "database": "connected",
            "timestamp": chrono::Utc::now().to_rfc3339()
        }))),
        Err(e) => {
            error!("Database health check failed: {}", e);
            Err(StatusCode::SERVICE_UNAVAILABLE)
        }
    }
}

async fn ws_handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(socket: axum::extract::ws::WebSocket, state: AppState) {
    use axum::extract::ws::Message;
    use futures_util::{SinkExt, StreamExt};
    use network::messages::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    info!("New WebSocket connection established");

    // Create a session for this connection
    let session_id = state.session_store.create_session().await;
    info!("Created session: {}", session_id);

    let (mut ws_sender, mut ws_receiver) = socket.split();

    let (outgoing_tx, mut outgoing_rx) = tokio::sync::mpsc::unbounded_channel::<Envelope>();
    state
        .session_store
        .set_sender(&session_id, Some(outgoing_tx.clone()))
        .await;

    let send_task = tokio::spawn(async move {
        while let Some(envelope) = outgoing_rx.recv().await {
            if let Ok(json) = serde_json::to_string(&envelope) {
                if ws_sender.send(Message::Text(json)).await.is_err() {
                    break;
                }
            }
        }
    });

    // Send handshake response
    let handshake_response = Envelope {
        sequence_id: 1,
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64,
        payload: Payload::HandshakeResponse(HandshakeResponse {
            accepted: true,
            server_version: "0.1.0".to_string(),
            protocol_version: "1.0".to_string(),
            server_features: 0,
            message: "Welcome to OpenMMO!".to_string(),
        }),
    };

    if !send_session_envelope(&state, &session_id, handshake_response).await {
        return;
    }

    // Handle incoming messages
    while let Some(Ok(msg)) = ws_receiver.next().await {
        match msg {
            Message::Text(text) => {
                info!("Received message: {}", text);

                // Try to parse as Envelope
                if let Ok(envelope) = serde_json::from_str::<Envelope>(&text) {
                    match &envelope.payload {
                        Payload::Ping(ping) => {
                            // Respond with pong
                            let pong_response = Envelope {
                                sequence_id: envelope.sequence_id,
                                timestamp: SystemTime::now()
                                    .duration_since(UNIX_EPOCH)
                                    .unwrap()
                                    .as_millis() as u64,
                                payload: Payload::Pong(Pong {
                                    timestamp: ping.timestamp,
                                }),
                            };

                            if !send_session_envelope(&state, &session_id, pong_response).await {
                                break;
                            }
                        }
                        Payload::MovementIntent(movement) => {
                            // Queue movement intent for processing
                            if let Some(session) =
                                state.session_store.get_session(&session_id).await
                            {
                                let intent = network::MovementIntent {
                                    player_id: session.player_id.unwrap_or(0),
                                    target_x: movement.target_position.x,
                                    target_y: movement.target_position.y,
                                    target_z: movement.target_position.z,
                                    speed_modifier: movement.speed_modifier,
                                    stop_movement: movement.stop_movement,
                                    rotation_y: movement.rotation_y,
                                };

                                {
                                    let mut world = state.world_state.write().await;
                                    world.queue_movement_intent(intent);
                                }
                            }
                        }
                        Payload::CombatAction(combat) => {
                            // Queue combat action for processing
                            if let Some(session) =
                                state.session_store.get_session(&session_id).await
                            {
                                let action = match combat.action_type {
                                    network::messages::ActionType::AutoAttack => {
                                        crate::simulation::CombatAction::AutoAttack {
                                            target_id: combat.target_entity_id,
                                        }
                                    }
                                    network::messages::ActionType::Ability => {
                                        crate::simulation::CombatAction::Ability {
                                            ability_id: combat.ability_id,
                                            target_id: combat.target_entity_id,
                                        }
                                    }
                                };

                                {
                                    let mut world = state.world_state.write().await;
                                    world.queue_combat_action(
                                        session.player_id.unwrap_or(0),
                                        action,
                                    );
                                }
                            }
                        }
                        Payload::AuthRequest(auth) => {
                            // Handle authentication request
                            let auth_result = if auth.character_name.is_some() {
                                // Treat presence of character name as login attempt
                                state
                                    .account_service
                                    .authenticate(&auth.username, &auth.password_hash)
                                    .await
                            } else {
                                // Registration flow: try auth, then auto-register if needed
                                match state
                                    .account_service
                                    .authenticate(&auth.username, &auth.password_hash)
                                    .await
                                {
                                    Ok(account) => Ok(account),
                                    Err(_) => {
                                        state
                                            .account_service
                                            .register(
                                                auth.username.clone(),
                                                format!("{}@openmmo.local", auth.username),
                                                auth.password_hash.clone(),
                                            )
                                            .await
                                    }
                                }
                            };

                            let auth_response = match auth_result {
                                Ok(account) => {
                                    let player_id_u64 = match state
                                        .session_store
                                        .allocate_player_id(&session_id)
                                        .await
                                    {
                                        Some(id) => id,
                                        None => {
                                            error!(
                                                "Failed to allocate synthetic player id for session {}",
                                                session_id
                                            );
                                            let response = network::messages::AuthResponse {
                                                success: false,
                                                session_token: None,
                                                message: "Internal server error".to_string(),
                                                player_id: None,
                                                character_id: None,
                                            };

                                            let envelope = Envelope {
                                                sequence_id: envelope.sequence_id,
                                                timestamp: SystemTime::now()
                                                    .duration_since(UNIX_EPOCH)
                                                    .unwrap()
                                                    .as_millis()
                                                    as u64,
                                                payload: Payload::AuthResponse(response),
                                            };

                                            if !send_session_envelope(&state, &session_id, envelope)
                                                .await
                                            {
                                                break;
                                            }

                                            continue;
                                        }
                                    };
                                    state
                                        .session_store
                                        .authenticate_session(
                                            &session_id,
                                            account.id,
                                            player_id_u64,
                                            None,
                                        )
                                        .await;

                                    network::messages::AuthResponse {
                                        success: true,
                                        session_token: Some(session_id.to_string()),
                                        message: "Authentication successful".to_string(),
                                        player_id: Some(player_id_u64),
                                        character_id: None,
                                    }
                                }
                                Err(e) => network::messages::AuthResponse {
                                    success: false,
                                    session_token: None,
                                    message: format!("Authentication failed: {:?}", e),
                                    player_id: None,
                                    character_id: None,
                                },
                            };

                            let response = Envelope {
                                sequence_id: envelope.sequence_id,
                                timestamp: SystemTime::now()
                                    .duration_since(UNIX_EPOCH)
                                    .unwrap()
                                    .as_millis() as u64,
                                payload: Payload::AuthResponse(auth_response),
                            };

                            if !send_session_envelope(&state, &session_id, response).await {
                                break;
                            }
                        }
                        Payload::CharacterCreateRequest(create_req) => {
                            // Ensure session exists
                            let session = if let Some(s) =
                                state.session_store.get_session(&session_id).await
                            {
                                s
                            } else {
                                let error_response = network::messages::CharacterCreateResponse {
                                    success: false,
                                    character: None,
                                    error_message: Some("Session not found".to_string()),
                                };

                                let response = Envelope {
                                    sequence_id: envelope.sequence_id,
                                    timestamp: SystemTime::now()
                                        .duration_since(UNIX_EPOCH)
                                        .unwrap()
                                        .as_millis()
                                        as u64,
                                    payload: Payload::CharacterCreateResponse(error_response),
                                };

                                if !send_session_envelope(&state, &session_id, response).await {
                                    break;
                                }
                                continue;
                            };

                            let account_id = if let Some(id) = session.account_id {
                                id
                            } else {
                                let error_response = network::messages::CharacterCreateResponse {
                                    success: false,
                                    character: None,
                                    error_message: Some("Not authenticated".to_string()),
                                };

                                let response = Envelope {
                                    sequence_id: envelope.sequence_id,
                                    timestamp: SystemTime::now()
                                        .duration_since(UNIX_EPOCH)
                                        .unwrap()
                                        .as_millis()
                                        as u64,
                                    payload: Payload::CharacterCreateResponse(error_response),
                                };

                                if !send_session_envelope(&state, &session_id, response).await {
                                    break;
                                }
                                continue;
                            };

                            let create_result = state
                                .account_service
                                .create_character(
                                    account_id,
                                    create_req.name.clone(),
                                    create_req.class.clone(),
                                )
                                .await;

                            let create_response = match create_result {
                                Ok(character) => match state
                                    .session_store
                                    .map_character_id(&session_id, character.id)
                                    .await
                                {
                                    Some(synthetic_id) => {
                                        match build_character_info(
                                            &character,
                                            synthetic_id,
                                            character.is_online,
                                        ) {
                                            Ok(info) => {
                                                network::messages::CharacterCreateResponse {
                                                    success: true,
                                                    character: Some(info),
                                                    error_message: None,
                                                }
                                            }
                                            Err(err) => {
                                                error!(
                                                    "Invalid character data for session {}: {}",
                                                    session_id, err
                                                );
                                                network::messages::CharacterCreateResponse {
                                                    success: false,
                                                    character: None,
                                                    error_message: Some(
                                                        "Invalid character data".to_string(),
                                                    ),
                                                }
                                            }
                                        }
                                    }
                                    None => {
                                        error!(
                                            "Failed to map character id for session {}",
                                            session_id
                                        );
                                        network::messages::CharacterCreateResponse {
                                            success: false,
                                            character: None,
                                            error_message: Some(
                                                "Internal server error".to_string(),
                                            ),
                                        }
                                    }
                                },
                                Err(e) => network::messages::CharacterCreateResponse {
                                    success: false,
                                    character: None,
                                    error_message: Some(format!(
                                        "Character creation failed: {:?}",
                                        e
                                    )),
                                },
                            };

                            let response = Envelope {
                                sequence_id: envelope.sequence_id,
                                timestamp: SystemTime::now()
                                    .duration_since(UNIX_EPOCH)
                                    .unwrap()
                                    .as_millis() as u64,
                                payload: Payload::CharacterCreateResponse(create_response),
                            };

                            if !send_session_envelope(&state, &session_id, response).await {
                                break;
                            }
                        }
                        Payload::CharacterListRequest(_req) => {
                            let account_id = match state
                                .session_store
                                .get_session(&session_id)
                                .await
                            {
                                Some(session) => match session.account_id {
                                    Some(id) => id,
                                    None => {
                                        let error_response =
                                            network::messages::CharacterListResponse {
                                                characters: vec![],
                                            };

                                        let response = Envelope {
                                            sequence_id: envelope.sequence_id,
                                            timestamp: SystemTime::now()
                                                .duration_since(UNIX_EPOCH)
                                                .unwrap()
                                                .as_millis()
                                                as u64,
                                            payload: Payload::CharacterListResponse(error_response),
                                        };

                                        if !send_session_envelope(&state, &session_id, response)
                                            .await
                                        {
                                            break;
                                        }
                                        continue;
                                    }
                                },
                                None => {
                                    let error_response = network::messages::CharacterListResponse {
                                        characters: vec![],
                                    };

                                    let response = Envelope {
                                        sequence_id: envelope.sequence_id,
                                        timestamp: SystemTime::now()
                                            .duration_since(UNIX_EPOCH)
                                            .unwrap()
                                            .as_millis()
                                            as u64,
                                        payload: Payload::CharacterListResponse(error_response),
                                    };

                                    if !send_session_envelope(&state, &session_id, response).await {
                                        break;
                                    }
                                    continue;
                                }
                            };

                            let characters_result =
                                state.account_service.get_characters(account_id).await;

                            let character_list_response = match characters_result {
                                Ok(characters) => {
                                    let mut infos = Vec::with_capacity(characters.len());
                                    for character in characters {
                                        match state
                                            .session_store
                                            .map_character_id(&session_id, character.id)
                                            .await
                                        {
                                            Some(synthetic_id) => {
                                                match build_character_info(
                                                    &character,
                                                    synthetic_id,
                                                    character.is_online,
                                                ) {
                                                    Ok(info) => infos.push(info),
                                                    Err(err) => error!(
                                                        "Invalid character data for session {}: {}",
                                                        session_id, err
                                                    ),
                                                }
                                            }
                                            None => error!(
                                                "Failed to map character id for session {}",
                                                session_id
                                            ),
                                        }
                                    }

                                    network::messages::CharacterListResponse { characters: infos }
                                }
                                Err(e) => {
                                    error!("Failed to get characters: {:?}", e);
                                    network::messages::CharacterListResponse { characters: vec![] }
                                }
                            };

                            let response = Envelope {
                                sequence_id: envelope.sequence_id,
                                timestamp: SystemTime::now()
                                    .duration_since(UNIX_EPOCH)
                                    .unwrap()
                                    .as_millis() as u64,
                                payload: Payload::CharacterListResponse(character_list_response),
                            };

                            if !send_session_envelope(&state, &session_id, response).await {
                                break;
                            }
                        }
                        Payload::CharacterSelectRequest(select_req) => {
                            let account_id = match state
                                .session_store
                                .get_session(&session_id)
                                .await
                            {
                                Some(session) => match session.account_id {
                                    Some(id) => id,
                                    None => {
                                        let error_response =
                                            network::messages::CharacterSelectResponse {
                                                success: false,
                                                character: None,
                                                error_message: Some(
                                                    "Not authenticated".to_string(),
                                                ),
                                            };

                                        let response = Envelope {
                                            sequence_id: envelope.sequence_id,
                                            timestamp: SystemTime::now()
                                                .duration_since(UNIX_EPOCH)
                                                .unwrap()
                                                .as_millis()
                                                as u64,
                                            payload: Payload::CharacterSelectResponse(
                                                error_response,
                                            ),
                                        };

                                        if !send_session_envelope(&state, &session_id, response)
                                            .await
                                        {
                                            break;
                                        }
                                        continue;
                                    }
                                },
                                None => {
                                    let error_response =
                                        network::messages::CharacterSelectResponse {
                                            success: false,
                                            character: None,
                                            error_message: Some("Session not found".to_string()),
                                        };

                                    let response = Envelope {
                                        sequence_id: envelope.sequence_id,
                                        timestamp: SystemTime::now()
                                            .duration_since(UNIX_EPOCH)
                                            .unwrap()
                                            .as_millis()
                                            as u64,
                                        payload: Payload::CharacterSelectResponse(error_response),
                                    };

                                    if !send_session_envelope(&state, &session_id, response).await {
                                        break;
                                    }
                                    continue;
                                }
                            };

                            let target_character_uuid = match state
                                .session_store
                                .resolve_character_id(&session_id, select_req.character_id)
                                .await
                            {
                                Some(uuid) => uuid,
                                None => {
                                    let error_response =
                                        network::messages::CharacterSelectResponse {
                                            success: false,
                                            character: None,
                                            error_message: Some(
                                                "Unknown character selection".to_string(),
                                            ),
                                        };

                                    let response = Envelope {
                                        sequence_id: envelope.sequence_id,
                                        timestamp: SystemTime::now()
                                            .duration_since(UNIX_EPOCH)
                                            .unwrap()
                                            .as_millis()
                                            as u64,
                                        payload: Payload::CharacterSelectResponse(error_response),
                                    };

                                    if !send_session_envelope(&state, &session_id, response).await {
                                        break;
                                    }
                                    continue;
                                }
                            };

                            let characters_result =
                                state.account_service.get_characters(account_id).await;

                            let mut snapshot_to_send: Option<network::messages::WorldSnapshot> =
                                None;

                            let character_select_response = match characters_result {
                                Ok(characters) => {
                                    let selected = characters
                                        .into_iter()
                                        .find(|c| c.id == target_character_uuid);

                                    match selected {
                                        Some(character) => {
                                            let snapshot_character = character.clone();

                                            let spawn_pose = (
                                                snapshot_character.position_x as f32,
                                                snapshot_character.position_y as f32,
                                                snapshot_character.position_z as f32,
                                                snapshot_character.rotation as f32,
                                            );

                                            let entity_id = {
                                                let mut world = state.world_state.write().await;
                                                // Clear any stale copies of this character by name
                                                world.remove_player_by_name(
                                                    &snapshot_character.name,
                                                );
                                                info!(
                                                    "Spawning character {} in zone {} at ({:.2}, {:.2}, {:.2}) rot {:.2}",
                                                    snapshot_character.id,
                                                    snapshot_character.zone_id,
                                                    spawn_pose.0,
                                                    spawn_pose.1,
                                                    spawn_pose.2,
                                                    spawn_pose.3
                                                );
                                                world
                                                    .spawn_player_entity(
                                                        &snapshot_character.name,
                                                        &snapshot_character.zone_id,
                                                        (spawn_pose.0, spawn_pose.1, spawn_pose.2),
                                                        spawn_pose.3,
                                                        (
                                                            snapshot_character.health,
                                                            snapshot_character.max_health,
                                                        ),
                                                    )
                                                    .unwrap_or_else(|_| {
                                                        world
                                                            .spawn_player_entity(
                                                                &snapshot_character.name,
                                                                "1",
                                                                (
                                                                    spawn_pose.0,
                                                                    spawn_pose.1,
                                                                    spawn_pose.2,
                                                                ),
                                                                spawn_pose.3,
                                                                (
                                                                    snapshot_character.health,
                                                                    snapshot_character.max_health,
                                                                ),
                                                            )
                                                            .expect("Failed to spawn player entity")
                                                    })
                                            };

                                            state
                                                .session_store
                                                .authenticate_session(
                                                    &session_id,
                                                    account_id,
                                                    entity_id,
                                                    Some(character.id),
                                                )
                                                .await;

                                            // Persist spawn pose immediately so re-joins use latest position
                                            if let Err(e) = state
                                                .account_service
                                                .update_character_position(
                                                    character.id,
                                                    spawn_pose.0 as f64,
                                                    spawn_pose.1 as f64,
                                                    spawn_pose.2 as f64,
                                                    spawn_pose.3 as f64,
                                                )
                                                .await
                                            {
                                                warn!(
                                                    "Failed to persist spawn pose for character {}: {:?}",
                                                    character.id, e
                                                );
                                            }

                                            if let Err(e) = state
                                                .account_service
                                                .set_character_online(character.id, true)
                                                .await
                                            {
                                                error!(
                                                    "Failed to mark character online for session {}: {:?}",
                                                    session_id, e
                                                );
                                            }

                                            snapshot_to_send = {
                                                let world = state.world_state.read().await;
                                                if let Some(session) = state
                                                    .session_store
                                                    .get_session(&session_id)
                                                    .await
                                                {
                                                    build_world_snapshot(&world, &session)
                                                } else {
                                                    None
                                                }
                                            };

                                            match build_character_info(
                                                &character,
                                                select_req.character_id,
                                                true,
                                            ) {
                                                Ok(info) => {
                                                    network::messages::CharacterSelectResponse {
                                                        success: true,
                                                        character: Some(info),
                                                        error_message: None,
                                                    }
                                                }
                                                Err(err) => {
                                                    error!(
                                                        "Invalid character data for session {}: {}",
                                                        session_id, err
                                                    );
                                                    network::messages::CharacterSelectResponse {
                                                        success: false,
                                                        character: None,
                                                        error_message: Some(
                                                            "Invalid character data".to_string(),
                                                        ),
                                                    }
                                                }
                                            }
                                        }
                                        None => network::messages::CharacterSelectResponse {
                                            success: false,
                                            character: None,
                                            error_message: Some("Character not found".to_string()),
                                        },
                                    }
                                }
                                Err(e) => {
                                    error!("Failed to get characters for selection: {:?}", e);
                                    network::messages::CharacterSelectResponse {
                                        success: false,
                                        character: None,
                                        error_message: Some(
                                            "Failed to retrieve characters".to_string(),
                                        ),
                                    }
                                }
                            };

                            let response = Envelope {
                                sequence_id: envelope.sequence_id,
                                timestamp: SystemTime::now()
                                    .duration_since(UNIX_EPOCH)
                                    .unwrap()
                                    .as_millis() as u64,
                                payload: Payload::CharacterSelectResponse(
                                    character_select_response,
                                ),
                            };

                            if !send_session_envelope(&state, &session_id, response).await {
                                break;
                            }

                            if let Some(snapshot) = snapshot_to_send {
                                let snapshot_envelope = Envelope {
                                    sequence_id: envelope.sequence_id.wrapping_add(1),
                                    timestamp: SystemTime::now()
                                        .duration_since(UNIX_EPOCH)
                                        .unwrap()
                                        .as_millis()
                                        as u64,
                                    payload: Payload::WorldSnapshot(snapshot),
                                };

                                if !send_session_envelope(&state, &session_id, snapshot_envelope)
                                    .await
                                {
                                    break;
                                }
                            }
                        }
                        Payload::HandshakeRequest(_) => {
                            // Already handled handshake
                        }
                        _ => {
                            info!("Received unhandled message type");
                        }
                    }
                } else {
                    error!("Failed to parse message: {}", text);
                }
            }
            Message::Close(_) => {
                info!("WebSocket connection closed for session: {}", session_id);
                break;
            }
            _ => {}
        }
    }

    let session_snapshot = state.session_store.get_session(&session_id).await;

    state.session_store.set_sender(&session_id, None).await;
    drop(outgoing_tx);
    info!("Waiting for send task to finish for {}", session_id);
    match tokio::time::timeout(Duration::from_secs(2), send_task).await {
        Ok(join_res) => {
            if let Err(e) = join_res {
                warn!("Send task join error for {}: {:?}", session_id, e);
            } else {
                info!("Send task finished for {}", session_id);
            }
        }
        Err(_) => {
            warn!(
                "Send task did not finish in time for {}, aborting",
                session_id
            );
        }
    }

    info!("Beginning session cleanup for {}", session_id);

    if let Some(session) = &session_snapshot {
        info!(
            "Cleanup state for session {}: player_id {:?}, character_id {:?}",
            session_id, session.player_id, session.character_id
        );

        if let (Some(player_id), Some(character_id)) = (session.player_id, session.character_id) {
            let pose_and_name = {
                let world = state.world_state.read().await;
                (
                    world.get_player_pose(player_id),
                    world.get_player_name(player_id),
                )
            };

            if let Some((x, y, z, rot)) = pose_and_name.0 {
                info!(
                    "Persisting pose for session {} character {}: ({:.2}, {:.2}, {:.2}) rot {:.2}",
                    session_id, character_id, x, y, z, rot
                );

                if let Err(e) = state
                    .account_service
                    .update_character_position(
                        character_id,
                        x as f64,
                        y as f64,
                        z as f64,
                        rot as f64,
                    )
                    .await
                {
                    warn!(
                        "Failed to persist character position for session {}: {:?}",
                        session_id, e
                    );
                } else {
                    info!(
                        "Saved character {} position for session {}: ({:.2}, {:.2}, {:.2}) rot {:.2}",
                        character_id, session_id, x, y, z, rot
                    );
                }
            } else {
                let diagnostics = {
                    let world = state.world_state.read().await;
                    let zone_id = world.get_player_zone_id(player_id);
                    let has_entity = zone_id
                        .and_then(|zid| {
                            world
                                .get_zone(zid)
                                .and_then(|zone| zone.entities.get_entity(player_id))
                        })
                        .is_some();
                    (zone_id, has_entity)
                };

                warn!(
                    "No pose available to save for session {} (player_id {:?}), zone {:?}, entity_exists {}",
                    session_id, session.player_id, diagnostics.0, diagnostics.1
                );
            }

            if let Err(e) = state
                .account_service
                .set_character_online(character_id, false)
                .await
            {
                warn!(
                    "Failed to mark character offline for session {}: {:?}",
                    session_id, e
                );
            }

            let mut world = state.world_state.write().await;
            world.remove_player(player_id);
            // Also clear any duplicate stale entries by name
            if let Some(name) = pose_and_name.1 {
                world.remove_player_by_name(&name);
            }
        } else {
            warn!(
                "Session {} missing player_id or character_id during cleanup (player_id {:?}, character_id {:?})",
                session_id, session.player_id, session.character_id
            );
        }
    } else {
        warn!(
            "No session snapshot found during cleanup for {} (session already removed?)",
            session_id
        );
    }

    state.session_store.remove_session(&session_id).await;
    info!("Session cleaned up: {}", session_id);
}

async fn send_session_envelope(state: &AppState, session_id: &Uuid, envelope: Envelope) -> bool {
    match state
        .session_store
        .send_envelope(session_id, envelope)
        .await
    {
        Ok(_) => true,
        Err(err) => {
            error!("Failed to send envelope to {}: {:?}", session_id, err);
            false
        }
    }
}

fn build_character_info(
    character: &db::models::Character,
    synthetic_id: u64,
    is_online: bool,
) -> Result<network::messages::CharacterInfo, ConversionError> {
    let wire = CharacterWireView::try_from(character)?;
    Ok(network::messages::CharacterInfo {
        id: synthetic_id,
        name: character.name.clone(),
        class: character.class.clone(),
        level: wire.level,
        experience: wire.experience,
        zone_id: character.zone_id.clone(),
        health: wire.health,
        max_health: wire.max_health,
        resource_type: character.resource_type.clone(),
        resource_value: wire.resource_value,
        max_resource: wire.max_resource,
        is_online,
    })
}
