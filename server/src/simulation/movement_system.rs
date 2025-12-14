//! Movement validation and processing system
//!
//! This module handles player movement intents, validates them,
//! and updates entity positions.


use crate::entities::{Entity, EntityId};
use crate::world::WorldState;

/// Movement intent from a client
#[derive(Debug, Clone)]
pub struct MovementIntent {
    pub player_id: EntityId,
    pub target_x: f32,
    pub target_y: f32,
    pub target_z: f32,
    pub speed_modifier: f32, // 1.0 = normal speed
}

/// Movement system for processing movement intents
pub struct MovementSystem;

impl MovementSystem {
    /// Process a movement intent
    pub fn process_movement_intent(
        world_state: &mut WorldState,
        intent: MovementIntent,
    ) -> Result<(), String> {
        // Get the player's zone
        let zone_id = world_state
            .get_player_zone_id(intent.player_id)
            .ok_or_else(|| format!("Player {} not in any zone", intent.player_id))?;

        let zone = world_state
            .get_zone_mut(zone_id)
            .ok_or_else(|| format!("Zone {} not found", zone_id))?;

        // Get the player entity
        let entity = zone
            .entities
            .get_entity_mut(intent.player_id)
            .ok_or_else(|| format!("Player entity {} not found", intent.player_id))?;

        // Validate movement
        Self::validate_movement(entity, &intent)?;

        // Apply movement
        Self::apply_movement(entity, intent);

        Ok(())
    }

    /// Validate a movement intent
    fn validate_movement(entity: &Entity, intent: &MovementIntent) -> Result<(), String> {
        // Check if entity can move
        if !entity.can_move() {
            return Err("Entity cannot move".to_string());
        }

        // Check if entity is alive
        if !entity.is_alive() {
            return Err("Entity is not alive".to_string());
        }

        let movement = entity.movement.as_ref().unwrap();
        let position = entity.position.as_ref().unwrap();

        // Check speed limits
        let dx = intent.target_x - position.x;
        let dy = intent.target_y - position.y;
        let dz = intent.target_z - position.z;
        let distance = (dx * dx + dy * dy + dz * dz).sqrt();

        let max_distance_per_tick = movement.max_speed * intent.speed_modifier / 20.0; // 20 TPS
        if distance > max_distance_per_tick {
            return Err(format!(
                "Movement distance {} exceeds maximum {} per tick",
                distance, max_distance_per_tick
            ));
        }

        // TODO: Add collision detection
        // TODO: Add terrain validation
        // TODO: Add zone boundary checks

        Ok(())
    }

    /// Apply validated movement to an entity
    fn apply_movement(entity: &mut Entity, intent: MovementIntent) {
        if let (Some(position), Some(movement)) = (&mut entity.position, &mut entity.movement) {
            // Calculate direction vector
            let dx = intent.target_x - position.x;
            let dy = intent.target_y - position.y;
            let dz = intent.target_z - position.z;
            let distance = (dx * dx + dy * dy + dz * dz).sqrt();

            if distance > 0.0 {
                // Normalize direction and apply speed
                let speed = movement.speed * intent.speed_modifier;
                movement.velocity_x = (dx / distance) * speed;
                movement.velocity_y = (dy / distance) * speed;
                movement.velocity_z = (dz / distance) * speed;
                movement.is_moving = true;

                // Update rotation to face movement direction
                position.rotation = dy.atan2(dx);
            }
        }
    }

    /// Stop movement for an entity
    pub fn stop_movement(world_state: &mut WorldState, entity_id: EntityId) -> Result<(), String> {
        let zone_id = world_state
            .get_player_zone_id(entity_id)
            .ok_or_else(|| format!("Entity {} not in any zone", entity_id))?;

        let zone = world_state
            .get_zone_mut(zone_id)
            .ok_or_else(|| format!("Zone {} not found", zone_id))?;

        let entity = zone
            .entities
            .get_entity_mut(entity_id)
            .ok_or_else(|| format!("Entity {} not found", entity_id))?;

        if let Some(movement) = &mut entity.movement {
            movement.velocity_x = 0.0;
            movement.velocity_y = 0.0;
            movement.velocity_z = 0.0;
            movement.is_moving = false;
        }

        Ok(())
    }

    /// Get current position of an entity
    pub fn get_entity_position(
        world_state: &WorldState,
        entity_id: EntityId,
    ) -> Option<(f32, f32, f32)> {
        let zone = world_state.get_player_zone(entity_id)?;
        let entity = zone.entities.get_entity(entity_id)?;
        entity.position.as_ref().map(|p| (p.x, p.y, p.z))
    }
}
