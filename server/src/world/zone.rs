//! Zone management for the game world
//!
//! Zones represent distinct areas of the game world with their own
//! entities, boundaries, and rules.

use std::collections::HashSet;
use crate::entities::{EntityManager, EntityId};

/// Represents a game zone/area
pub struct Zone {
    pub id: u32,
    pub name: String,
    pub bounds: ZoneBounds,
    pub entities: EntityManager,
    pub active_players: HashSet<EntityId>,
}

#[derive(Debug, Clone)]
pub struct ZoneBounds {
    pub min_x: f32,
    pub max_x: f32,
    pub min_y: f32,
    pub max_y: f32,
    pub min_z: f32,
    pub max_z: f32,
}

impl Zone {
    pub fn new(id: u32, name: String, bounds: ZoneBounds) -> Self {
        Self {
            id,
            name,
            bounds,
            entities: EntityManager::new(),
            active_players: HashSet::new(),
        }
    }

    /// Check if a position is within this zone's bounds
    pub fn contains_position(&self, x: f32, y: f32, z: f32) -> bool {
        x >= self.bounds.min_x && x <= self.bounds.max_x &&
        y >= self.bounds.min_y && y <= self.bounds.max_y &&
        z >= self.bounds.min_z && z <= self.bounds.max_z
    }

    /// Add a player to this zone
    pub fn add_player(&mut self, player_id: EntityId) {
        self.active_players.insert(player_id);
    }

    /// Remove a player from this zone
    pub fn remove_player(&mut self, player_id: EntityId) {
        self.active_players.remove(&player_id);
    }

    /// Get all players in this zone
    pub fn get_players(&self) -> Vec<EntityId> {
        self.active_players.iter().cloned().collect()
    }

    /// Update all entities in this zone
    pub fn update(&mut self, delta_time: f64) {
        self.entities.update_entities(delta_time);
    }

    /// Create starter zone with some test entities
    pub fn create_starter_zone() -> Self {
        let mut zone = Self::new(
            1,
            "Starter Zone".to_string(),
            ZoneBounds {
                min_x: -100.0,
                max_x: 100.0,
                min_y: -10.0,
                max_y: 50.0,
                min_z: -100.0,
                max_z: 100.0,
            },
        );

        // Create some test mobs
        zone.entities.create_test_mob("Goblin".to_string(), 15.0, 15.0);
        zone.entities.create_test_mob("Orc".to_string(), -15.0, 15.0);
        zone.entities.create_test_mob("Wolf".to_string(), 0.0, 25.0);

        zone
    }
}