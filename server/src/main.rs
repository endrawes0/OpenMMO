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

use axum::{
    extract::{State, WebSocketUpgrade},
    http::StatusCode,
    response::{Json, Response},
    routing::get,
    Router,
};
use serde_json::json;
use tracing::{error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

#[derive(Clone)]
struct AppState {
    db_pool: sqlx::PgPool,
    session_store: network::SessionStore,
    world_state: std::sync::Arc<tokio::sync::RwLock<world::WorldState>>,
    account_service: accounts::AccountService,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "openmmo=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

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
    tokio::spawn(async move {
        let mut simulation_loop = simulation::SimulationLoop::new(simulation_world_state);
        simulation_loop.run().await;
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

async fn handle_socket(mut socket: axum::extract::ws::WebSocket, state: AppState) {
    use axum::extract::ws::Message;
    use futures_util::StreamExt;
    use network::messages::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    info!("New WebSocket connection established");

    // Create a session for this connection
    let session_id = state.session_store.create_session().await;
    info!("Created session: {}", session_id);

    // Create a test player entity for this session
    let player_id = {
        let mut world = state.world_state.write().await;
        let zone = world.get_zone_mut(1).unwrap(); // Starter zone
        let player_entity_id = zone
            .entities
            .create_test_player(format!("Player_{}", session_id));
        world.add_player_to_starter_zone(player_entity_id);
        player_entity_id
    };

    // Update session with player ID
    state
        .session_store
        .authenticate_session(&session_id, Uuid::from_u128(1), player_id, 0)
        .await; // Hardcode account_id = 1, character_id = 0

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

    if let Ok(json) = serde_json::to_string(&handshake_response) {
        if socket.send(Message::Text(json)).await.is_err() {
            info!("Failed to send handshake response");
            return;
        }
    }

    // Handle incoming messages
    while let Some(Ok(msg)) = socket.next().await {
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

                            if let Ok(json) = serde_json::to_string(&pong_response) {
                                if socket.send(Message::Text(json)).await.is_err() {
                                    break;
                                }
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
                                // This is a login request (character_name is provided for login)
                                state.account_service.authenticate(&auth.username, &auth.password_hash).await
                            } else {
                                // This is a registration request (no character_name means register)
                                // For registration, we need an email, but the client doesn't provide one
                                // For MVP, we'll treat this as login and auto-create account if it doesn't exist
                                match state.account_service.authenticate(&auth.username, &auth.password_hash).await {
                                    Ok(account) => Ok(account),
                                    Err(_) => {
                                        // Try to register the account
                                        state.account_service.register(
                                            auth.username.clone(),
                                            format!("{}@openmmo.local", auth.username), // Auto-generate email
                                            auth.password_hash.clone(),
                                        ).await
                                    }
                                }
                            };

                            let auth_response = match auth_result {
                                Ok(account) => {
                                    // Update session with account info
                                    let player_id_u64 = account.id.as_u128() as u64;
                                    state.session_store.authenticate_session(&session_id, account.id, player_id_u64, 0).await;

                                    network::messages::AuthResponse {
                                        success: true,
                                        session_token: Some(session_id.to_string()),
                                        message: "Authentication successful".to_string(),
                                        player_id: Some(player_id_u64),
                                        character_id: None, // Will be set when character is selected
                                    }
                                }
                                Err(e) => {
                                    network::messages::AuthResponse {
                                        success: false,
                                        session_token: None,
                                        message: format!("Authentication failed: {:?}", e),
                                        player_id: None,
                                        character_id: None,
                                    }
                                }
                            };

                            let response = Envelope {
                                sequence_id: envelope.sequence_id,
                                timestamp: SystemTime::now()
                                    .duration_since(UNIX_EPOCH)
                                    .unwrap()
                                    .as_millis() as u64,
                                payload: Payload::AuthResponse(auth_response),
                            };

                            if let Ok(json) = serde_json::to_string(&response) {
                                if socket.send(Message::Text(json)).await.is_err() {
                                    break;
                                }
                            }
                        }
                        Payload::CharacterCreateRequest(create_req) => {
                            // Handle character creation request
                            // Check if session exists and is authenticated
                            let session = if let Some(s) = state.session_store.get_session(&session_id).await {
                                s
                            } else {
                                // Send error response - session not found
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
                                        .as_millis() as u64,
                                    payload: Payload::CharacterCreateResponse(error_response),
                                };

                                if let Ok(json) = serde_json::to_string(&response) {
                                    let _ = socket.send(Message::Text(json)).await;
                                }
                                continue;
                            };

                            let account_id = if let Some(id) = session.account_id {
                                id
                            } else {
                                // Send error response - not authenticated
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
                                        .as_millis() as u64,
                                    payload: Payload::CharacterCreateResponse(error_response),
                                };

                                if let Ok(json) = serde_json::to_string(&response) {
                                    let _ = socket.send(Message::Text(json)).await;
                                }
                                continue;
                            };

                            let create_result = state.account_service.create_character(
                                account_id,
                                create_req.name.clone(),
                                create_req.class.clone(),
                            ).await;

                            let create_response = match create_result {
                                Ok(character) => {
                                    network::messages::CharacterCreateResponse {
                                        success: true,
                                        character: Some(network::messages::CharacterInfo {
                                            id: character.id.as_u128() as u64,
                                            name: character.name,
                                            class: character.class,
                                            level: character.level as u32,
                                            experience: character.experience as u64,
                                            zone_id: character.zone_id,
                                            health: character.health as u32,
                                            max_health: character.max_health as u32,
                                            resource_type: character.resource_type,
                                            resource_value: character.resource_value as u32,
                                            max_resource: character.max_resource as u32,
                                            is_online: character.is_online,
                                        }),
                                        error_message: None,
                                    }
                                }
                                Err(e) => {
                                    network::messages::CharacterCreateResponse {
                                        success: false,
                                        character: None,
                                        error_message: Some(format!("Character creation failed: {:?}", e)),
                                    }
                                }
                            };

                            let response = Envelope {
                                sequence_id: envelope.sequence_id,
                                timestamp: SystemTime::now()
                                    .duration_since(UNIX_EPOCH)
                                    .unwrap()
                                    .as_millis() as u64,
                                payload: Payload::CharacterCreateResponse(create_response),
                            };

                            if let Ok(json) = serde_json::to_string(&response) {
                                if socket.send(Message::Text(json)).await.is_err() {
                                    break;
                                }
                            }
                        }
                        Payload::CharacterListRequest(_) => {
                            // Handle character list request
                            // Get account_id from session
                            let account_id = match state.session_store.get_session(&session_id).await {
                                Some(session) => match session.account_id {
                                    Some(id) => id,
                                    None => {
                                        // Send error response - not authenticated
                                        let error_response = network::messages::CharacterListResponse {
                                            characters: vec![],
                                        };

                                        let response = Envelope {
                                            sequence_id: envelope.sequence_id,
                                            timestamp: SystemTime::now()
                                                .duration_since(UNIX_EPOCH)
                                                .unwrap()
                                                .as_millis() as u64,
                                            payload: Payload::CharacterListResponse(error_response),
                                        };

                                        if let Ok(json) = serde_json::to_string(&response) {
                                            let _ = socket.send(Message::Text(json)).await;
                                        }
                                        continue;
                                    }
                                },
                                None => {
                                    // Send error response - session not found
                                    let error_response = network::messages::CharacterListResponse {
                                        characters: vec![],
                                    };

                                    let response = Envelope {
                                        sequence_id: envelope.sequence_id,
                                        timestamp: SystemTime::now()
                                            .duration_since(UNIX_EPOCH)
                                            .unwrap()
                                            .as_millis() as u64,
                                        payload: Payload::CharacterListResponse(error_response),
                                    };

                                    if let Ok(json) = serde_json::to_string(&response) {
                                        let _ = socket.send(Message::Text(json)).await;
                                    }
                                    continue;
                                }
                            };

                            let characters_result = state.account_service.get_characters(account_id).await;

                            let character_list_response = match characters_result {
                                Ok(characters) => {
                                    let character_infos: Vec<network::messages::CharacterInfo> = characters.into_iter().map(|c| {
                                        network::messages::CharacterInfo {
                                            id: c.id.as_u128() as u64,
                                            name: c.name,
                                            class: c.class,
                                            level: c.level as u32,
                                            experience: c.experience as u64,
                                            zone_id: c.zone_id,
                                            health: c.health as u32,
                                            max_health: c.max_health as u32,
                                            resource_type: c.resource_type,
                                            resource_value: c.resource_value as u32,
                                            max_resource: c.max_resource as u32,
                                            is_online: c.is_online,
                                        }
                                    }).collect();

                                    network::messages::CharacterListResponse {
                                        characters: character_infos,
                                    }
                                }
                                Err(e) => {
                                    error!("Failed to get characters: {:?}", e);
                                    network::messages::CharacterListResponse {
                                        characters: vec![],
                                    }
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

                            if let Ok(json) = serde_json::to_string(&response) {
                                if socket.send(Message::Text(json)).await.is_err() {
                                    break;
                                }
                            }
                        }
                        Payload::CharacterSelectRequest(select_req) => {
                            // Handle character selection request
                            // Get account_id from session
                            let account_id = match state.session_store.get_session(&session_id).await {
                                Some(session) => match session.account_id {
                                    Some(id) => id,
                                    None => {
                                        // Send error response - not authenticated
                                        let error_response = network::messages::CharacterSelectResponse {
                                            success: false,
                                            character: None,
                                            error_message: Some("Not authenticated".to_string()),
                                        };

                                        let response = Envelope {
                                            sequence_id: envelope.sequence_id,
                                            timestamp: SystemTime::now()
                                                .duration_since(UNIX_EPOCH)
                                                .unwrap()
                                                .as_millis() as u64,
                                            payload: Payload::CharacterSelectResponse(error_response),
                                        };

                                        if let Ok(json) = serde_json::to_string(&response) {
                                            let _ = socket.send(Message::Text(json)).await;
                                        }
                                        continue;
                                    }
                                },
                                None => {
                                    // Send error response - session not found
                                    let error_response = network::messages::CharacterSelectResponse {
                                        success: false,
                                        character: None,
                                        error_message: Some("Session not found".to_string()),
                                    };

                                    let response = Envelope {
                                        sequence_id: envelope.sequence_id,
                                        timestamp: SystemTime::now()
                                            .duration_since(UNIX_EPOCH)
                                            .unwrap()
                                            .as_millis() as u64,
                                        payload: Payload::CharacterSelectResponse(error_response),
                                    };

                                    if let Ok(json) = serde_json::to_string(&response) {
                                        let _ = socket.send(Message::Text(json)).await;
                                    }
                                    continue;
                                }
                            };

                            // Get all characters and find the selected one
                            let characters_result = state.account_service.get_characters(account_id).await;

                            let character_select_response = match characters_result {
                                Ok(characters) => {
                                    // Find the character with the requested ID
                                    let selected_character = characters.into_iter().find(|c| c.id.as_u128() as u64 == select_req.character_id);

                                    match selected_character {
                                        Some(character) => {
                                            // Update session with selected character
                                            state.session_store.authenticate_session(&session_id, account_id, character.id.as_u128() as u64, character.id.as_u128() as u64).await;

                                            network::messages::CharacterSelectResponse {
                                                success: true,
                                                character: Some(network::messages::CharacterInfo {
                                                    id: character.id.as_u128() as u64,
                                                    name: character.name,
                                                    class: character.class,
                                                    level: character.level as u32,
                                                    experience: character.experience as u64,
                                                    zone_id: character.zone_id,
                                                    health: character.health as u32,
                                                    max_health: character.max_health as u32,
                                                    resource_type: character.resource_type,
                                                    resource_value: character.resource_value as u32,
                                                    max_resource: character.max_resource as u32,
                                                    is_online: character.is_online,
                                                }),
                                                error_message: None,
                                            }
                                        }
                                        None => {
                                            // Character not found
                                            network::messages::CharacterSelectResponse {
                                                success: false,
                                                character: None,
                                                error_message: Some("Character not found".to_string()),
                                            }
                                        }
                                    }
                                }
                                Err(e) => {
                                    error!("Failed to get characters for selection: {:?}", e);
                                    network::messages::CharacterSelectResponse {
                                        success: false,
                                        character: None,
                                        error_message: Some("Failed to retrieve characters".to_string()),
                                    }
                                }
                            };

                            let response = Envelope {
                                sequence_id: envelope.sequence_id,
                                timestamp: SystemTime::now()
                                    .duration_since(UNIX_EPOCH)
                                    .unwrap()
                                    .as_millis() as u64,
                                payload: Payload::CharacterSelectResponse(character_select_response),
                            };

                            if let Ok(json) = serde_json::to_string(&response) {
                                if socket.send(Message::Text(json)).await.is_err() {
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
                state.session_store.remove_session(&session_id).await;
                break;
            }
            _ => {}
        }
    }

    // Clean up session on disconnect
    state.session_store.remove_session(&session_id).await;
    info!("Session cleaned up: {}", session_id);
}
