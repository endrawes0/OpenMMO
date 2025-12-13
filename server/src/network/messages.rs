// Manual implementation of Protobuf message types
// This will be replaced by generated code once protoc is available

use serde::{Deserialize, Serialize};

/// Message envelope for all communications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Envelope {
    pub sequence_id: u32,
    pub timestamp: u64,
    pub payload: Payload,
}

/// Payload variants
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Payload {
    HandshakeRequest(HandshakeRequest),
    HandshakeResponse(HandshakeResponse),
    AuthRequest(AuthRequest),
    AuthResponse(AuthResponse),
    Ping(Ping),
    Pong(Pong),
    Error(Error),
    Disconnect(Disconnect),
    WorldSnapshot(WorldSnapshot),
}

/// Handshake messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandshakeRequest {
    pub client_version: String,
    pub protocol_version: String,
    pub supported_features: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandshakeResponse {
    pub accepted: bool,
    pub server_version: String,
    pub protocol_version: String,
    pub server_features: u32,
    pub message: String,
}

/// Authentication messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthRequest {
    pub username: String,
    pub password_hash: String,
    pub character_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    pub success: bool,
    pub session_token: Option<String>,
    pub message: String,
    pub player_id: Option<u64>,
    pub character_id: Option<u64>,
}

/// Ping/Pong for connection health
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ping {
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pong {
    pub timestamp: u64,
}

/// Error messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Error {
    pub code: ErrorCode,
    pub message: String,
    pub details: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ErrorCode {
    UnknownError = 0,
    InvalidRequest = 1,
    AuthenticationFailed = 2,
    SessionExpired = 3,
    CharacterNotFound = 4,
    ServerFull = 5,
    ProtocolMismatch = 6,
    RateLimited = 7,
}

/// Disconnect notification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Disconnect {
    pub reason: DisconnectReason,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DisconnectReason {
    Unknown = 0,
    ClientRequest = 1,
    ServerShutdown = 2,
    Timeout = 3,
    Kicked = 4,
    Banned = 5,
}

/// Basic world snapshot for initial state sync
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldSnapshot {
    pub snapshot_id: u64,
    pub entities: Vec<Entity>,
    pub player_entity_id: u64,
    pub zone_name: String,
}

/// Basic entity representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub id: u64,
    pub entity_type: String,
    pub position: Vector3,
    pub rotation: Vector3,
    pub state: EntityState,
}

/// 3D vector for positions/rotations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// Entity state information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityState {
    pub movement_state: MovementState,
    pub health_percent: f32,
    pub display_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MovementState {
    Idle = 0,
    Walking = 1,
    Running = 2,
    Dead = 3,
}