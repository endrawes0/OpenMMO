//! Global world state management
//!
//! This module manages the overall game world state, including
//! all zones and cross-zone operations.

use std::collections::{HashMap, VecDeque};
use crate::world::Zone;
use crate::entities::EntityId;
use crate::network::MovementIntent;
use crate::simulation::CombatAction;

/// Manages the entire game world
pub struct WorldState {
    zones: HashMap<u32, Zone>,
    player_zone_map: HashMap<EntityId, u32>, // Player ID -> Zone ID
    movement_intents: VecDeque<MovementIntent>, // Queue of movement intents to process
    combat_actions: VecDeque<(EntityId, CombatAction)>, // Queue of (attacker_id, action) to process
}

impl WorldState {
    pub fn new() -> Self {
        let mut world = Self {
            zones: HashMap::new(),
            player_zone_map: HashMap::new(),
            movement_intents: VecDeque::new(),
            combat_actions: VecDeque::new(),
        };

        // Create starter zone
        let starter_zone = Zone::create_starter_zone();
        let zone_id = starter_zone.id;
        world.zones.insert(zone_id, starter_zone);

        world
    }

    /// Get a zone by ID
    pub fn get_zone(&self, zone_id: u32) -> Option<&Zone> {
        self.zones.get(&zone_id)
    }

    /// Get a mutable reference to a zone by ID
    pub fn get_zone_mut(&mut self, zone_id: u32) -> Option<&mut Zone> {
        self.zones.get_mut(&zone_id)
    }

    /// Get the zone a player is currently in
    pub fn get_player_zone(&self, player_id: EntityId) -> Option<&Zone> {
        self.player_zone_map.get(&player_id)
            .and_then(|zone_id| self.zones.get(zone_id))
    }

    /// Get the zone ID a player is currently in
    pub fn get_player_zone_id(&self, player_id: EntityId) -> Option<u32> {
        self.player_zone_map.get(&player_id).cloned()
    }

    /// Move a player to a different zone
    pub fn move_player_to_zone(&mut self, player_id: EntityId, new_zone_id: u32) -> Result<(), String> {
        // Remove from current zone
        if let Some(current_zone_id) = self.player_zone_map.get(&player_id).cloned() {
            if let Some(current_zone) = self.zones.get_mut(&current_zone_id) {
                current_zone.remove_player(player_id);
                // Note: Entity stays in zone's entity manager for now
                // In a full implementation, we'd move the entity data too
            }
        }

        // Add to new zone
        if let Some(new_zone) = self.zones.get_mut(&new_zone_id) {
            new_zone.add_player(player_id);
            self.player_zone_map.insert(player_id, new_zone_id);
            Ok(())
        } else {
            Err(format!("Zone {} does not exist", new_zone_id))
        }
    }

    /// Add a player to the starter zone
    pub fn add_player_to_starter_zone(&mut self, player_id: EntityId) {
        if let Some(starter_zone) = self.zones.get_mut(&1) {
            starter_zone.add_player(player_id);
            self.player_zone_map.insert(player_id, 1);
        }
    }

    /// Update all zones
    pub fn update(&mut self, delta_time: f64) {
        for zone in self.zones.values_mut() {
            zone.update(delta_time);
        }
    }

    /// Get all zones
    pub fn get_all_zones(&self) -> Vec<&Zone> {
        self.zones.values().collect()
    }

    /// Get zone count
    pub fn zone_count(&self) -> usize {
        self.zones.len()
    }

    /// Queue a movement intent for processing
    pub fn queue_movement_intent(&mut self, intent: MovementIntent) {
        self.movement_intents.push_back(intent);
    }

    /// Get and clear the movement intents queue
    pub fn drain_movement_intents(&mut self) -> VecDeque<MovementIntent> {
        std::mem::take(&mut self.movement_intents)
    }

    /// Queue a combat action for processing
    pub fn queue_combat_action(&mut self, attacker_id: EntityId, action: CombatAction) {
        self.combat_actions.push_back((attacker_id, action));
    }

    /// Get and clear the combat actions queue
    pub fn drain_combat_actions(&mut self) -> VecDeque<(EntityId, CombatAction)> {
        std::mem::take(&mut self.combat_actions)
    }
}