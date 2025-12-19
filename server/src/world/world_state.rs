//! Global world state management
//!
//! This module manages the overall game world state, including
//! all zones and cross-zone operations.

use crate::entities::EntityId;
use crate::network::MovementIntent;
use crate::simulation::CombatAction;
use crate::world::Zone;
use std::collections::{HashMap, VecDeque};
use tracing::warn;

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

        // Create second zone
        let second_zone = Zone::create_second_zone();
        let zone_id = second_zone.id;
        world.zones.insert(zone_id, second_zone);

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
        self.player_zone_map
            .get(&player_id)
            .and_then(|zone_id| self.zones.get(zone_id))
    }

    /// Get the zone ID a player is currently in
    pub fn get_player_zone_id(&self, player_id: EntityId) -> Option<u32> {
        self.player_zone_map.get(&player_id).cloned()
    }

    /// Ensure a player has a zone mapping; if missing, try to infer it from zones.
    pub fn ensure_player_zone_mapping(&mut self, player_id: EntityId) -> Option<u32> {
        if let Some(zone_id) = self.get_player_zone_id(player_id) {
            return Some(zone_id);
        }

        for (zone_id, zone) in &self.zones {
            if zone.entities.get_entity(player_id).is_some() {
                self.player_zone_map.insert(player_id, *zone_id);
                return Some(*zone_id);
            }
        }

        None
    }

    /// Spawn or respawn a player entity in the requested zone at the given position
    pub fn spawn_player_entity(
        &mut self,
        name: &str,
        zone_label: &str,
        position: (f32, f32, f32),
        rotation: f32,
        health: (i32, i32),
    ) -> Result<EntityId, String> {
        let zone_id = self.resolve_zone_id(zone_label);
        let zone = self
            .get_zone_mut(zone_id)
            .ok_or_else(|| format!("Zone {} not found", zone_id))?;

        // Allocate a new entity ID and build a player entity
        let entity_id = zone.entities.generate_id();
        let mut player = crate::entities::Entity::new_player(entity_id, name.to_string());
        if let Some(pos) = &mut player.position {
            pos.x = position.0;
            pos.y = position.1;
            pos.z = position.2;
            pos.rotation = rotation;
        }
        if let Some(h) = &mut player.health {
            h.current = health.0.max(0) as u32;
            h.maximum = health.1.max(1) as u32;
        }

        zone.entities.add_entity(player);
        zone.add_player(entity_id);
        self.player_zone_map.insert(entity_id, zone_id);

        Ok(entity_id)
    }

    /// Resolve a zone identifier from either a numeric ID or name; defaults to starter zone.
    pub fn resolve_zone_id(&self, zone_label: &str) -> u32 {
        if let Ok(id) = zone_label.parse::<u32>() {
            if self.zones.contains_key(&id) {
                return id;
            }
        }
        let normalized = zone_label.replace('_', " ");
        for (id, zone) in &self.zones {
            if zone.name.eq_ignore_ascii_case(zone_label)
                || zone.name.eq_ignore_ascii_case(&normalized)
            {
                return *id;
            }
        }
        1
    }

    /// Move a player to a different zone
    pub fn move_player_to_zone(
        &mut self,
        player_id: EntityId,
        new_zone_id: u32,
    ) -> Result<(), String> {
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

        // Check for zone transitions
        self.check_zone_transitions();
    }

    /// Check for players at zone transition points and move them
    fn check_zone_transitions(&mut self) {
        let mut transitions = Vec::new();

        for (&zone_id, zone) in &self.zones {
            for &player_id in &zone.active_players.clone() {
                // Clone to avoid borrow issues
                if let Some(entity) = zone.entities.get_entity(player_id) {
                    if let Some(position) = &entity.position {
                        // Check transition from starter zone (1) to second zone (2)
                        if zone_id == 1 && position.x > 95.0 {
                            transitions.push((player_id, 2, (-95.0, position.y, position.z)));
                        }
                        // Check transition from second zone (2) to starter zone (1)
                        else if zone_id == 2 && position.x < -145.0 {
                            transitions.push((player_id, 1, (95.0, position.y, position.z)));
                        }
                    }
                }
            }
        }

        for (player_id, new_zone_id, new_position) in transitions {
            if let Err(e) =
                self.move_player_to_zone_with_position(player_id, new_zone_id, new_position)
            {
                warn!(
                    "Failed to move player {} to zone {}: {}",
                    player_id, new_zone_id, e
                );
            }
        }
    }

    /// Move a player to a different zone with specific position
    pub fn move_player_to_zone_with_position(
        &mut self,
        player_id: EntityId,
        new_zone_id: u32,
        position: (f32, f32, f32),
    ) -> Result<(), String> {
        // Remove from current zone
        if let Some(current_zone_id) = self.player_zone_map.get(&player_id).cloned() {
            if let Some(current_zone) = self.zones.get_mut(&current_zone_id) {
                current_zone.remove_player(player_id);
                // Move the entity to new position before moving zones
                if let Some(entity) = current_zone.entities.get_entity_mut(player_id) {
                    if let Some(pos) = &mut entity.position {
                        pos.x = position.0;
                        pos.y = position.1;
                        pos.z = position.2;
                    }
                }
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

    /// Remove a player from the world and clean up its entity
    pub fn remove_player(&mut self, player_id: EntityId) {
        if let Some(zone_id) = self.player_zone_map.remove(&player_id) {
            if let Some(zone) = self.zones.get_mut(&zone_id) {
                zone.remove_player(player_id);
                let _ = zone.entities.remove_entity(player_id);
            }
        }
    }

    /// Remove any player entities by display name to avoid stale duplicates
    pub fn remove_player_by_name(&mut self, name: &str) {
        let mut to_remove: Vec<EntityId> = Vec::new();

        for zone in self.zones.values() {
            for entity in zone.entities.get_all_entities() {
                if entity.name == name
                    && matches!(entity.entity_type, crate::entities::EntityType::Player)
                {
                    to_remove.push(entity.id);
                    self.player_zone_map.remove(&entity.id);
                }
            }
        }

        for id in to_remove {
            if let Some(zone_id) = self.player_zone_map.remove(&id) {
                if let Some(zone) = self.zones.get_mut(&zone_id) {
                    zone.remove_player(id);
                    let _ = zone.entities.remove_entity(id);
                }
            } else {
                // Best-effort removal even if map was already cleared
                for zone in self.zones.values_mut() {
                    zone.entities.remove_entity(id);
                    zone.remove_player(id);
                }
            }
        }
    }

    /// Get a player's current position and rotation if present
    pub fn get_player_pose(&self, player_id: EntityId) -> Option<(f32, f32, f32, f32)> {
        let zone_id = self.player_zone_map.get(&player_id)?;
        let zone = self.zones.get(zone_id)?;
        let entity = zone.entities.get_entity(player_id)?;
        let pos = entity.position.as_ref()?;
        Some((pos.x, pos.y, pos.z, pos.rotation))
    }

    pub fn get_player_name(&self, player_id: EntityId) -> Option<String> {
        let zone_id = self.player_zone_map.get(&player_id)?;
        let zone = self.zones.get(zone_id)?;
        let entity = zone.entities.get_entity(player_id)?;
        Some(entity.name.clone())
    }
}
