// Allow dead code warnings for Phase 0 infrastructure
#[allow(dead_code)]
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[allow(dead_code)]
pub struct Account {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    #[allow(dead_code)]
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_login_at: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub is_banned: bool,
    pub ban_reason: Option<String>,
    pub ban_expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[allow(dead_code)]
pub struct Character {
    pub id: Uuid,
    pub account_id: Uuid,
    pub name: String,
    pub class: String,
    pub level: i32,
    pub experience: i64,
    pub zone_id: String,
    pub position_x: f64,
    pub position_y: f64,
    pub position_z: f64,
    pub rotation: f64,
    pub health: i32,
    pub max_health: i32,
    pub resource_type: String,
    pub resource_value: i32,
    pub max_resource: i32,
    pub is_online: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_saved_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[allow(dead_code)]
pub struct CharacterStats {
    pub id: Uuid,
    pub character_id: Uuid,
    pub stat_name: String,
    pub stat_value: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[allow(dead_code)]
pub struct InventoryItem {
    pub id: Uuid,
    pub character_id: Uuid,
    pub item_id: String,
    pub item_name: String,
    pub item_type: String,
    pub item_slot: Option<String>,
    pub quantity: i32,
    pub quality: String,
    pub item_level: i32,
    pub stats: Option<serde_json::Value>,
    pub is_bound: bool,
    pub is_equipped: bool,
    pub slot_position: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[allow(dead_code)]
pub struct EquippedItem {
    pub id: Uuid,
    pub character_id: Uuid,
    pub inventory_item_id: Uuid,
    pub equipment_slot: String,
    pub equipped_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[allow(dead_code)]
pub struct Progression {
    pub id: Uuid,
    pub character_id: Uuid,
    pub progression_type: String,
    pub current_value: i64,
    pub max_value: i64,
    pub level: i32,
    pub experience_to_next_level: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[allow(dead_code)]
pub struct QuestState {
    pub id: Uuid,
    pub character_id: Uuid,
    pub quest_id: String,
    pub quest_name: String,
    pub status: String,
    pub progress: serde_json::Value,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub turned_in_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
