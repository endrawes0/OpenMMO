pub mod messages;

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Session represents a connected client
#[derive(Debug, Clone)]
pub struct Session {
    pub id: Uuid,
    pub account_id: Option<Uuid>,
    pub player_id: Option<u64>, // For network protocol compatibility
    pub character_id: Option<u64>,
    pub authenticated: bool,
    pub connected_at: std::time::Instant,
}

/// Movement intent from a client
#[derive(Debug, Clone)]
pub struct MovementIntent {
    pub player_id: u64,
    pub target_x: f32,
    pub target_y: f32,
    pub target_z: f32,
    pub speed_modifier: f32,
}

/// Session store for managing connected clients
#[derive(Debug, Clone)]
pub struct SessionStore {
    sessions: Arc<RwLock<HashMap<Uuid, Session>>>,
}

impl SessionStore {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn create_session(&self) -> Uuid {
        let session_id = Uuid::new_v4();
        let session = Session {
            id: session_id,
            account_id: None,
            player_id: None,
            character_id: None,
            authenticated: false,
            connected_at: std::time::Instant::now(),
        };

        let mut sessions = self.sessions.write().await;
        sessions.insert(session_id, session);
        session_id
    }

    pub async fn get_session(&self, session_id: &Uuid) -> Option<Session> {
        let sessions = self.sessions.read().await;
        sessions.get(session_id).cloned()
    }

    pub async fn update_session(&self, session: Session) {
        let mut sessions = self.sessions.write().await;
        sessions.insert(session.id, session);
    }

    pub async fn remove_session(&self, session_id: &Uuid) {
        let mut sessions = self.sessions.write().await;
        sessions.remove(session_id);
    }

    pub async fn authenticate_session(&self, session_id: &Uuid, account_id: Uuid, player_id: u64, character_id: u64) {
        if let Some(mut session) = self.get_session(session_id).await {
            session.authenticated = true;
            session.account_id = Some(account_id);
            session.player_id = Some(player_id);
            session.character_id = Some(character_id);
            self.update_session(session).await;
        }
    }

    pub async fn get_active_sessions(&self) -> Vec<Session> {
        let sessions = self.sessions.read().await;
        sessions.values().cloned().collect()
    }
}

impl Default for SessionStore {
    fn default() -> Self {
        Self::new()
    }
}
