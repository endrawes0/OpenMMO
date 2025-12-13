//! Entity definitions and types
//!
//! This module defines the different types of entities in the game
//! and the Entity struct that composes components.

use std::collections::HashMap;
use crate::entities::components::*;

/// Entity archetype enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntityType {
    Player,
    Mob,
    Npc,
    WorldObject,
}

/// Main Entity struct that holds all components
#[derive(Debug, Clone)]
pub struct Entity {
    pub id: EntityId,
    pub entity_type: EntityType,
    pub name: String,

    // Core components (most entities have these)
    pub position: Option<Position>,
    pub health: Option<Health>,

    // Optional components
    pub movement: Option<Movement>,
    pub combat: Option<Combat>,
    pub abilities: Option<Abilities>,
    pub ai: Option<Ai>,
    pub social: Option<Social>,
    pub inventory: Option<Inventory>,
    pub equipment: Option<Equipment>,
    pub progression: Option<Progression>,
    pub quest_state: Option<QuestState>,
    pub appearance: Option<Appearance>,
    pub network_sync: Option<NetworkSync>,
}

impl Entity {
    /// Create a new player entity
    pub fn new_player(id: EntityId, name: String) -> Self {
        Self {
            id,
            entity_type: EntityType::Player,
            name,
            position: Some(Position {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                rotation: 0.0,
            }),
            health: Some(Health {
                current: 100,
                maximum: 100,
                regeneration_rate: 1.0,
            }),
            movement: Some(Movement {
                velocity_x: 0.0,
                velocity_y: 0.0,
                velocity_z: 0.0,
                speed: 5.0,
                max_speed: 8.0,
                is_moving: false,
            }),
            combat: Some(Combat {
                attack_power: 10,
                defense: 5,
                attack_range: 2.0,
                attack_speed: 1.0,
                last_attack_time: 0.0,
            }),
            abilities: Some(Abilities {
                ability_ids: vec![1, 2, 3], // Basic abilities
                cooldowns: HashMap::new(),
            }),
            ai: None, // Players don't have AI
            social: Some(Social {
                faction: Faction::Player,
                reputation: HashMap::new(),
            }),
            inventory: Some(Inventory {
                items: HashMap::new(),
                max_slots: 20,
            }),
            equipment: Some(Equipment {
                equipped_items: HashMap::new(),
            }),
            progression: Some(Progression {
                level: 1,
                experience: 0,
                experience_to_next: 100,
            }),
            quest_state: Some(QuestState {
                active_quests: HashMap::new(),
                completed_quests: Vec::new(),
            }),
            appearance: Some(Appearance {
                model_id: 1,
                scale: 1.0,
                color: (255, 255, 255),
            }),
            network_sync: Some(NetworkSync {
                last_sync_time: 0.0,
                sync_interval: 0.1, // Sync 10 times per second
                visible_to: Vec::new(),
            }),
        }
    }

    /// Create a new mob entity
    pub fn new_mob(id: EntityId, name: String, level: u32) -> Self {
        let base_health = 50 + (level * 20) as u32;
        let base_attack = 5 + (level * 2) as u32;

        Self {
            id,
            entity_type: EntityType::Mob,
            name,
            position: Some(Position {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                rotation: 0.0,
            }),
            health: Some(Health {
                current: base_health,
                maximum: base_health,
                regeneration_rate: 0.5,
            }),
            movement: Some(Movement {
                velocity_x: 0.0,
                velocity_y: 0.0,
                velocity_z: 0.0,
                speed: 3.0,
                max_speed: 6.0,
                is_moving: false,
            }),
            combat: Some(Combat {
                attack_power: base_attack,
                defense: base_attack / 2,
                attack_range: 1.5,
                attack_speed: 0.8,
                last_attack_time: 0.0,
            }),
            abilities: Some(Abilities {
                ability_ids: vec![100], // Basic mob attack
                cooldowns: HashMap::new(),
            }),
            ai: Some(Ai {
                state: AiState::Idle,
                aggro_range: 8.0,
                leash_range: 25.0,
                home_position: (0.0, 0.0, 0.0),
                last_state_change: 0.0,
            }),
            social: Some(Social {
                faction: Faction::Hostile,
                reputation: HashMap::new(),
            }),
            inventory: None, // Mobs don't have inventory
            equipment: None, // Mobs don't equip items
            progression: None, // Mobs don't level up
            quest_state: None, // Mobs don't have quests
            appearance: Some(Appearance {
                model_id: 100 + level, // Different models based on level
                scale: 1.0,
                color: (200, 100, 100), // Reddish tint for hostile mobs
            }),
            network_sync: Some(NetworkSync {
                last_sync_time: 0.0,
                sync_interval: 0.2, // Sync 5 times per second for mobs
                visible_to: Vec::new(),
            }),
        }
    }

    /// Create a new NPC entity
    pub fn new_npc(id: EntityId, name: String) -> Self {
        Self {
            id,
            entity_type: EntityType::Npc,
            name,
            position: Some(Position {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                rotation: 0.0,
            }),
            health: Some(Health {
                current: 1,
                maximum: 1,
                regeneration_rate: 0.0, // NPCs don't regenerate
            }),
            movement: None, // NPCs don't move
            combat: None, // NPCs don't fight
            abilities: None, // NPCs don't have abilities
            ai: None, // NPCs don't have AI
            social: Some(Social {
                faction: Faction::Friendly,
                reputation: HashMap::new(),
            }),
            inventory: Some(Inventory {
                items: HashMap::new(), // Can be populated with vendor items
                max_slots: 50,
            }),
            equipment: None,
            progression: None,
            quest_state: None,
            appearance: Some(Appearance {
                model_id: 200, // NPC model
                scale: 1.0,
                color: (255, 255, 255),
            }),
            network_sync: Some(NetworkSync {
                last_sync_time: 0.0,
                sync_interval: 1.0, // Sync once per second for static NPCs
                visible_to: Vec::new(),
            }),
        }
    }

    /// Create a new world object entity
    pub fn new_world_object(id: EntityId, name: String, object_type: u32) -> Self {
        Self {
            id,
            entity_type: EntityType::WorldObject,
            name,
            position: Some(Position {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                rotation: 0.0,
            }),
            health: None, // World objects may or may not have health
            movement: None, // World objects don't move
            combat: None, // World objects don't fight
            abilities: None,
            ai: None,
            social: Some(Social {
                faction: Faction::Neutral,
                reputation: HashMap::new(),
            }),
            inventory: None,
            equipment: None,
            progression: None,
            quest_state: None,
            appearance: Some(Appearance {
                model_id: object_type,
                scale: 1.0,
                color: (255, 255, 255),
            }),
            network_sync: Some(NetworkSync {
                last_sync_time: 0.0,
                sync_interval: 2.0, // Sync every 2 seconds for static objects
                visible_to: Vec::new(),
            }),
        }
    }

    /// Check if entity is alive (has health > 0)
    pub fn is_alive(&self) -> bool {
        self.health.as_ref().map_or(true, |h| h.current > 0)
    }

    /// Check if entity can move
    pub fn can_move(&self) -> bool {
        self.movement.is_some() && self.is_alive()
    }

    /// Check if entity can attack
    pub fn can_attack(&self) -> bool {
        self.combat.is_some() && self.is_alive()
    }

    /// Get distance to another entity
    pub fn distance_to(&self, other: &Entity) -> f32 {
        if let (Some(pos1), Some(pos2)) = (&self.position, &other.position) {
            let dx = pos1.x - pos2.x;
            let dy = pos1.y - pos2.y;
            let dz = pos1.z - pos2.z;
            (dx * dx + dy * dy + dz * dz).sqrt()
        } else {
            f32::INFINITY
        }
    }

    /// Check if entity is hostile toward another entity
    pub fn is_hostile_toward(&self, other: &Entity) -> bool {
        if let (Some(social1), Some(social2)) = (&self.social, &other.social) {
            matches!(social1.faction, Faction::Hostile) &&
            matches!(social2.faction, Faction::Player)
        } else {
            false
        }
    }
}