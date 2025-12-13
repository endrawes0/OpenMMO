mod db;
mod network;

use axum::{extract::{State, WebSocketUpgrade}, http::StatusCode, response::{Json, Response}, routing::get, Router};
use serde_json::json;
use tracing::{error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
struct AppState {
    db_pool: sqlx::PgPool,
    session_store: network::SessionStore,
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

    // Run database migrations using our db module
    info!("Running database migrations...");
    db::run_migrations(&db_pool)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to run database migrations: {}", e))?;

    info!("Database migrations completed successfully");

    // Test database connectivity
    db::check_connection(&db_pool)
        .await
        .map_err(|e| anyhow::anyhow!("Database connectivity test failed: {}", e))?;

    info!("Database connectivity verified");

    // Create application state
    let session_store = network::SessionStore::new();
    let state = AppState { db_pool, session_store };

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

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> Response {
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
