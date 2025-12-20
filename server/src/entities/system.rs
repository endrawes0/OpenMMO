//! Entity system management
//!
//! This module provides the EntityManager for creating, updating,
//! and managing all game entities.

use crate::entities::components::*;
use crate::entities::{Entity, EntityId, EntityType};
use std::collections::HashMap;

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
        self.entities
            .values()
            .filter(|e| {
                std::mem::discriminant(&e.entity_type) == std::mem::discriminant(&entity_type)
            })
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
        self.entities
            .values()
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
                Self::update_entity_basic(entity, delta_time);
            }
        }

        // Update AI separately to avoid borrow issues
        // TODO: Re-enable AI updates after fixing compilation
        // for entity_id in entity_ids {
        //     if let Some(entity) = self.entities.get_mut(&entity_id) {
        //         self.update_ai(entity, delta_time);
        //     }
        // }
    }

    /// Update basic entity properties (health, movement)
    fn update_entity_basic(entity: &mut Entity, delta_time: f64) {
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

                // Preserve velocity as provided by movement intents; no damping
            }
        }
    }

    // /// Update AI behavior for an entity
    // fn update_ai(&self, entity: &mut Entity, delta_time: f64) {
    //     // TODO: Implement AI updates
    // }

    /// Create a test player entity
    pub fn create_test_player(&mut self, name: String) -> EntityId {
        let id = self.generate_id();
        let mut player = Entity::new_player(id, name);
        player.position = Some(Position {
            x: 0.0,
            y: 2.0,
            z: 12.0,
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
            inventory.items.insert(2, 5); // Item ID 2, quantity 5
        }
        self.add_entity(npc);
        id
    }
}
