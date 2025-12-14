//! Entity system management
//!
//! This module provides the EntityManager for creating, updating,
//! and managing all game entities.

use std::collections::HashMap;
use crate::entities::{Entity, EntityId, EntityType};
use crate::entities::components::*;

/// Manages all entities in the game world
pub struct EntityManager {
    entities: HashMap<EntityId, Entity>,
    next_id: EntityId,
}

impl EntityManager {
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
            next_id: 1,
        }
    }

    /// Generate a new unique entity ID
    pub fn generate_id(&mut self) -> EntityId {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    /// Add an entity to the manager
    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.insert(entity.id, entity);
    }

    /// Remove an entity from the manager
    pub fn remove_entity(&mut self, id: EntityId) -> Option<Entity> {
        self.entities.remove(&id)
    }

    /// Get an entity by ID
    pub fn get_entity(&self, id: EntityId) -> Option<&Entity> {
        self.entities.get(&id)
    }

    /// Get a mutable reference to an entity by ID
    pub fn get_entity_mut(&mut self, id: EntityId) -> Option<&mut Entity> {
        self.entities.get_mut(&id)
    }

    /// Get all entities
    pub fn get_all_entities(&self) -> Vec<&Entity> {
        self.entities.values().collect()
    }

    /// Get entities by type
    pub fn get_entities_by_type(&self, entity_type: EntityType) -> Vec<&Entity> {
        self.entities.values()
            .filter(|e| std::mem::discriminant(&e.entity_type) == std::mem::discriminant(&entity_type))
            .collect()
    }

    /// Get player entities
    pub fn get_players(&self) -> Vec<&Entity> {
        self.get_entities_by_type(EntityType::Player)
    }

    /// Get mob entities
    pub fn get_mobs(&self) -> Vec<&Entity> {
        self.get_entities_by_type(EntityType::Mob)
    }

    /// Get entities within a certain range of a position
    pub fn get_entities_in_range(&self, center: &(f32, f32, f32), range: f32) -> Vec<&Entity> {
        self.entities.values()
            .filter(|entity| {
                if let Some(pos) = &entity.position {
                    let dx = pos.x - center.0;
                    let dy = pos.y - center.1;
                    let dz = pos.z - center.2;
                    let distance_squared = dx * dx + dy * dy + dz * dz;
                    distance_squared <= range * range
                } else {
                    false
                }
            })
            .collect()
    }

    /// Update all entities (called every tick)
    pub fn update_entities(&mut self, delta_time: f64) {
        let entity_ids: Vec<EntityId> = self.entities.keys().cloned().collect();

        for entity_id in entity_ids {
            if let Some(entity) = self.entities.get_mut(&entity_id) {
                self.update_entity(entity, delta_time);
            }
        }
    }

    /// Update a single entity
    fn update_entity(&self, entity: &mut Entity, delta_time: f64) {
        // Update health regeneration
        if let Some(health) = &mut entity.health {
            if health.current < health.maximum {
                let regen_amount = (health.regeneration_rate * delta_time as f32) as u32;
                health.current = (health.current + regen_amount).min(health.maximum);
            }
        }

        // Update movement
        if let (Some(position), Some(movement)) = (&mut entity.position, &mut entity.movement) {
            if movement.is_moving {
                // Apply velocity to position
                position.x += movement.velocity_x * delta_time as f32;
                position.y += movement.velocity_y * delta_time as f32;
                position.z += movement.velocity_z * delta_time as f32;

                // Apply friction/damping
                let damping = 0.9;
                movement.velocity_x *= damping;
                movement.velocity_y *= damping;
                movement.velocity_z *= damping;

                // Stop if velocity is very low
                if movement.velocity_x.abs() < 0.01 &&
                   movement.velocity_y.abs() < 0.01 &&
                   movement.velocity_z.abs() < 0.01 {
                    movement.is_moving = false;
                    movement.velocity_x = 0.0;
                    movement.velocity_y = 0.0;
                    movement.velocity_z = 0.0;
                }
            }
        }

        // Update AI for mobs
        if let Some(ai) = &mut entity.ai {
            self.update_ai(entity, ai, delta_time);
        }
    }

    /// Update AI behavior for an entity
    fn update_ai(&self, entity: &mut Entity, ai: &mut Ai, delta_time: f64) {
        match &mut ai.state {
            AiState::Idle => {
                // Check for players in aggro range
                if let Some(position) = &entity.position {
                    let nearby_players = self.get_entities_in_range(
                        &(position.x, position.y, position.z),
                        ai.aggro_range
                    ).into_iter()
                    .filter(|e| matches!(e.entity_type, EntityType::Player))
                    .collect::<Vec<_>>();

                    if let Some(target) = nearby_players.first() {
                        ai.state = AiState::Chasing { target_id: target.id };
                        ai.last_state_change = delta_time;
                    }
                }
            }
            AiState::Chasing { target_id } => {
                if let Some(target) = self.get_entity(*target_id) {
                    if let (Some(entity_pos), Some(target_pos)) = (&entity.position, &target.position) {
                        let distance = entity.distance_to(target);

                        // If target is too far, return home
                        let home_distance = {
                            let dx = entity_pos.x - ai.home_position.0;
                            let dy = entity_pos.y - ai.home_position.1;
                            let dz = entity_pos.z - ai.home_position.2;
                            (dx * dx + dy * dy + dz * dz).sqrt()
                        };

                        if home_distance > ai.leash_range {
                            ai.state = AiState::Returning {
                                home_position: ai.home_position
                            };
                            ai.last_state_change = delta_time;
                        }
                        // If close enough to attack, switch to attacking
                        else if distance <= entity.combat.as_ref().map_or(1.5, |c| c.attack_range) {
                            ai.state = AiState::Attacking { target_id: *target_id };
                            ai.last_state_change = delta_time;
                        }
                        // Otherwise, move toward target
                        else if let Some(movement) = &mut entity.movement {
                            let dx = target_pos.x - entity_pos.x;
                            let dy = target_pos.y - entity_pos.y;
                            let dz = target_pos.z - entity_pos.z;
                            let length = (dx * dx + dy * dy + dz * dz).sqrt();

                            if length > 0.0 {
                                movement.velocity_x = (dx / length) * movement.speed;
                                movement.velocity_y = (dy / length) * movement.speed;
                                movement.velocity_z = (dz / length) * movement.speed;
                                movement.is_moving = true;
                            }
                        }
                    } else {
                        // Target not found, return to idle
                        ai.state = AiState::Idle;
                        ai.last_state_change = delta_time;
                    }
                } else {
                    // Target not found, return to idle
                    ai.state = AiState::Idle;
                    ai.last_state_change = delta_time;
                }
            }
            AiState::Attacking { target_id } => {
                if let Some(target) = self.get_entity(*target_id) {
                    let distance = entity.distance_to(target);

                    // If target moved away, chase again
                    if distance > entity.combat.as_ref().map_or(2.0, |c| c.attack_range * 1.5) {
                        ai.state = AiState::Chasing { target_id: *target_id };
                        ai.last_state_change = delta_time;
                    }
                    // Otherwise, perform attack logic (handled in combat system)
                } else {
                    // Target not found, return to idle
                    ai.state = AiState::Idle;
                    ai.last_state_change = delta_time;
                }
            }
            AiState::Returning { home_position } => {
                if let Some(position) = &entity.position {
                    let dx = home_position.0 - position.x;
                    let dy = home_position.1 - position.y;
                    let dz = home_position.2 - position.z;
                    let distance = (dx * dx + dy * dy + dz * dz).sqrt();

                    if distance < 1.0 {
                        // Reached home, return to idle
                        ai.state = AiState::Idle;
                        ai.last_state_change = delta_time;
                    } else if let Some(movement) = &mut entity.movement {
                        // Move toward home
                        movement.velocity_x = (dx / distance) * movement.speed;
                        movement.velocity_y = (dy / distance) * movement.speed;
                        movement.velocity_z = (dz / distance) * movement.speed;
                        movement.is_moving = true;
                    }
                }
            }
            AiState::Patrolling { waypoints, current_waypoint } => {
                // TODO: Implement patrolling behavior
                // For now, just stay idle
                ai.state = AiState::Idle;
                ai.last_state_change = delta_time;
            }
            AiState::Fleeing { .. } => {
                // TODO: Implement fleeing behavior
                ai.state = AiState::Idle;
                ai.last_state_change = delta_time;
            }
        }
    }

    /// Create a test player entity
    pub fn create_test_player(&mut self, name: String) -> EntityId {
        let id = self.generate_id();
        let mut player = Entity::new_player(id, name);
        player.position = Some(Position {
            x: 10.0,
            y: 0.0,
            z: 10.0,
            rotation: 0.0,
        });
        self.add_entity(player);
        id
    }

    /// Create a test mob entity
    pub fn create_test_mob(&mut self, name: String, x: f32, z: f32, level: u32) -> EntityId {
        let id = self.generate_id();
        let mut mob = Entity::new_mob(id, name, level);
        mob.position = Some(Position {
            x,
            y: 0.0,
            z,
            rotation: 0.0,
        });
        if let Some(ai) = &mut mob.ai {
            ai.home_position = (x, 0.0, z);
        }
        self.add_entity(mob);
        id
    }

    /// Create a test vendor NPC
    pub fn create_test_vendor(&mut self, name: String, x: f32, z: f32) -> EntityId {
        let id = self.generate_id();
        let mut npc = Entity::new_npc(id, name);
        npc.position = Some(Position {
            x,
            y: 0.0,
            z,
            rotation: 0.0,
        });
        // Add some items to inventory for selling
        if let Some(inventory) = &mut npc.inventory {
            inventory.items.insert(1, 10); // Item ID 1, quantity 10
            inventory.items.insert(2, 5);  // Item ID 2, quantity 5
        }
        self.add_entity(npc);
        id
    }
}