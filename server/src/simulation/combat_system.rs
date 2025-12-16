//! Combat system for handling attacks and damage
//!
//! This module implements the combat mechanics including
//! attack validation, damage calculation, and death handling.

use crate::entities::{Entity, EntityId};
use crate::world::WorldState;

/// Combat action types
#[derive(Debug, Clone)]
pub enum CombatAction {
    AutoAttack {
        target_id: EntityId,
    },
    Ability {
        ability_id: u32,
        target_id: EntityId,
    },
}

/// Combat result after processing an action
#[derive(Debug, Clone)]
pub struct CombatResult {
    pub success: bool,
    pub damage_dealt: u32,
    pub target_killed: bool,
    pub error_message: Option<String>,
}

/// Combat system for processing combat actions
pub struct CombatSystem;

impl CombatSystem {
    /// Process a combat action
    pub fn process_combat_action(
        world_state: &mut WorldState,
        attacker_id: EntityId,
        action: CombatAction,
    ) -> CombatResult {
        // Get attacker's zone
        let zone_id = match world_state.get_player_zone_id(attacker_id) {
            Some(id) => id,
            None => {
                return CombatResult {
                    success: false,
                    damage_dealt: 0,
                    target_killed: false,
                    error_message: Some("Attacker not in any zone".to_string()),
                }
            }
        };

        let zone = match world_state.get_zone_mut(zone_id) {
            Some(z) => z,
            None => {
                return CombatResult {
                    success: false,
                    damage_dealt: 0,
                    target_killed: false,
                    error_message: Some("Zone not found".to_string()),
                }
            }
        };

        // Get attacker entity
        let attacker = match zone.entities.get_entity(attacker_id) {
            Some(e) => e,
            None => {
                return CombatResult {
                    success: false,
                    damage_dealt: 0,
                    target_killed: false,
                    error_message: Some("Attacker entity not found".to_string()),
                }
            }
        };

        // Validate attacker can attack
        if !attacker.can_attack() {
            return CombatResult {
                success: false,
                damage_dealt: 0,
                target_killed: false,
                error_message: Some("Attacker cannot attack".to_string()),
            };
        }

        let target_id = match action {
            CombatAction::AutoAttack { target_id } | CombatAction::Ability { target_id, .. } => {
                target_id
            }
        };

        // Get target entity
        let target = match zone.entities.get_entity(target_id) {
            Some(e) => e,
            None => {
                return CombatResult {
                    success: false,
                    damage_dealt: 0,
                    target_killed: false,
                    error_message: Some("Target entity not found".to_string()),
                }
            }
        };

        // Validate attack
        if let Err(error) = Self::validate_attack(attacker, target, &action) {
            return CombatResult {
                success: false,
                damage_dealt: 0,
                target_killed: false,
                error_message: Some(error),
            };
        }

        // Calculate and apply damage
        let damage = Self::calculate_damage(attacker, target, &action);
        let target_killed =
            Self::apply_damage(zone.entities.get_entity_mut(target_id).unwrap(), damage);

        CombatResult {
            success: true,
            damage_dealt: damage,
            target_killed,
            error_message: None,
        }
    }

    /// Validate if an attack can be performed
    fn validate_attack(
        attacker: &Entity,
        target: &Entity,
        action: &CombatAction,
    ) -> Result<(), String> {
        // Check if target is alive
        if !target.is_alive() {
            return Err("Target is already dead".to_string());
        }

        // Check range
        let attack_range = attacker.combat.as_ref().unwrap().attack_range;
        let distance = attacker.distance_to(target);

        if distance > attack_range {
            return Err(format!(
                "Target is out of range ({} > {})",
                distance, attack_range
            ));
        }

        // Check attack cooldown
        let combat = attacker.combat.as_ref().unwrap();
        let current_time = 0.0; // TODO: Get actual current time
        let time_since_last_attack = current_time - combat.last_attack_time;

        match action {
            CombatAction::AutoAttack { .. } => {
                let attack_cooldown = 1.0 / combat.attack_speed;
                if time_since_last_attack < attack_cooldown as f64 {
                    return Err("Attack is on cooldown".to_string());
                }
            }
            CombatAction::Ability { ability_id: _, .. } => {
                // TODO: Check ability cooldowns
                // For now, allow abilities
            }
        }

        // Check if target is valid (not attacking self, etc.)
        if attacker.id == target.id {
            return Err("Cannot attack self".to_string());
        }

        Ok(())
    }

    /// Calculate damage for an attack
    fn calculate_damage(attacker: &Entity, target: &Entity, action: &CombatAction) -> u32 {
        let attacker_combat = attacker.combat.as_ref().unwrap();
        let target_combat = target.combat.as_ref();

        let base_damage = match action {
            CombatAction::AutoAttack { .. } => attacker_combat.attack_power,
            CombatAction::Ability { ability_id: _, .. } => {
                // TODO: Look up ability damage from data
                // For now, use a simple formula
                attacker_combat.attack_power * 2
            }
        };

        // Apply defense reduction
        let defense = target_combat.map_or(0, |c| c.defense);
        let damage_reduction = (defense as f32 * 0.5).min(base_damage as f32 * 0.75);
        let final_damage = (base_damage as f32 - damage_reduction).max(1.0) as u32;

        final_damage
    }

    /// Apply damage to a target and return if it was killed
    fn apply_damage(target: &mut Entity, damage: u32) -> bool {
        if let Some(health) = &mut target.health {
            if health.current <= damage {
                health.current = 0;
                // TODO: Handle death (respawn, loot, etc.)
                true // Target was killed
            } else {
                health.current -= damage;
                false // Target survived
            }
        } else {
            false // Target has no health component
        }
    }

    /// Check if an entity can attack another entity
    pub fn can_attack_entity(attacker: &Entity, target: &Entity) -> bool {
        if !attacker.can_attack() || !target.is_alive() {
            return false;
        }

        let attack_range = attacker.combat.as_ref().unwrap().attack_range;
        let distance = attacker.distance_to(target);

        distance <= attack_range
    }

    /// Get entities that can be attacked by a given entity
    pub fn get_attackable_entities(
        world_state: &WorldState,
        attacker_id: EntityId,
    ) -> Vec<EntityId> {
        let zone = match world_state.get_player_zone(attacker_id) {
            Some(z) => z,
            None => return Vec::new(),
        };

        let attacker = match zone.entities.get_entity(attacker_id) {
            Some(e) => e,
            None => return Vec::new(),
        };

        zone.entities
            .get_all_entities()
            .into_iter()
            .filter(|target| Self::can_attack_entity(attacker, target))
            .map(|e| e.id)
            .collect()
    }
}
