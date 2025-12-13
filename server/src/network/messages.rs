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
    MovementIntent(MovementIntent),
    CombatAction(CombatAction),
    EntityUpdate(EntityUpdate),
    CharacterListRequest(CharacterListRequest),
    CharacterListResponse(CharacterListResponse),
    CharacterCreateRequest(CharacterCreateRequest),
    CharacterCreateResponse(CharacterCreateResponse),
    CharacterSelectRequest(CharacterSelectRequest),
    CharacterSelectResponse(CharacterSelectResponse),
    CharacterDeleteRequest(CharacterDeleteRequest),
    CharacterDeleteResponse(CharacterDeleteResponse),
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

/// Character management messages

/// Request character list for account
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterListRequest;

/// Response with character list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterListResponse {
    pub characters: Vec<CharacterInfo>,
}

/// Character information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterInfo {
    pub id: u64,
    pub name: String,
    pub class: String,
    pub level: u32,
    pub experience: u64,
    pub zone_id: String,
    pub health: u32,
    pub max_health: u32,
    pub resource_type: String,
    pub resource_value: u32,
    pub max_resource: u32,
    pub is_online: bool,
}

/// Create character request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterCreateRequest {
    pub name: String,
    pub class: String,
}

/// Create character response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterCreateResponse {
    pub success: bool,
    pub character: Option<CharacterInfo>,
    pub error_message: Option<String>,
}

/// Select character request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterSelectRequest {
    pub character_id: u64,
}

/// Select character response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterSelectResponse {
    pub success: bool,
    pub character: Option<CharacterInfo>,
    pub error_message: Option<String>,
}

/// Delete character request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterDeleteRequest {
    pub character_id: u64,
}

/// Delete character response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterDeleteResponse {
    pub success: bool,
    pub error_message: Option<String>,
}
