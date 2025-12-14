//! ECS Components for game entities
//!
//! Components define the data and behavior aspects of entities.
//! Entities are composed of multiple components.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Unique identifier for entities
pub type EntityId = u64;

/// Position component for spatial location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub rotation: f32, // Yaw rotation in radians
}

/// Movement component for velocity and movement state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Movement {
    pub velocity_x: f32,
    pub velocity_y: f32,
    pub velocity_z: f32,
    pub speed: f32, // Current movement speed
    pub max_speed: f32, // Maximum allowed speed
    pub is_moving: bool,
}

/// Health and resource component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Health {
    pub current: u32,
    pub maximum: u32,
    pub regeneration_rate: f32, // HP per second
}

/// Combat component for attack/defense stats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Combat {
    pub attack_power: u32,
    pub defense: u32,
    pub attack_range: f32,
    pub attack_speed: f32, // Attacks per second
    pub last_attack_time: f64, // Timestamp of last attack
}

/// Ability component for entity abilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Abilities {
    pub ability_ids: Vec<u32>, // IDs of available abilities
    pub cooldowns: HashMap<u32, f64>, // Ability ID -> cooldown end time
}

/// AI component for NPC/mob behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AiState {
    Idle,
    Patrolling { waypoints: Vec<(f32, f32, f32)>, current_waypoint: usize },
    Chasing { target_id: EntityId },
    Attacking { target_id: EntityId },
    Fleeing { target_id: EntityId },
    Returning { home_position: (f32, f32, f32) },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ai {
    pub state: AiState,
    pub aggro_range: f32,
    pub leash_range: f32,
    pub home_position: (f32, f32, f32),
    pub last_state_change: f64,
}

/// Faction component for social relationships
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum Faction {
    Player,
    Neutral,
    Hostile,
    Friendly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Social {
    pub faction: Faction,
    pub reputation: HashMap<Faction, i32>, // Faction -> reputation value
}

/// Inventory component for items
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inventory {
    pub items: HashMap<u32, u32>, // Item ID -> quantity
    pub max_slots: u32,
}

/// Equipment component for equipped items
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum EquipmentSlot {
    Head,
    Chest,
    Legs,
    Feet,
    MainHand,
    OffHand,
    Accessory1,
    Accessory2,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Equipment {
    pub equipped_items: HashMap<EquipmentSlot, u32>, // Slot -> Item ID
}

/// Level and progression component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Progression {
    pub level: u32,
    pub experience: u32,
    pub experience_to_next: u32,
}

/// Quest state component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestState {
    pub active_quests: HashMap<u32, QuestProgress>, // Quest ID -> progress
    pub completed_quests: Vec<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestProgress {
    pub objectives: HashMap<String, u32>, // Objective name -> current count
    pub started_at: f64,
}

/// Visual appearance component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Appearance {
    pub model_id: u32,
    pub scale: f32,
    pub color: (u8, u8, u8), // RGB color tint
}

/// Network synchronization component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkSync {
    pub last_sync_time: f64,
    pub sync_interval: f64, // How often to sync this entity
    pub visible_to: Vec<EntityId>, // Which players can see this entity
}