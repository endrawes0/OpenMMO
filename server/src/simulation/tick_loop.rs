//! Main simulation tick loop
//!
//! This module implements the 20 Hz game simulation loop that
//! updates all game systems each tick.

use crate::entities::{Entity as GameEntity, EntityType};
use crate::network::messages::{self, Envelope, MovementState, Payload, Vector3, WorldSnapshot};
use crate::network::SessionStore;
use crate::simulation::movement_system::{MovementIntent as SimMovementIntent, MovementSystem};
use crate::simulation::CombatSystem;
use crate::world::WorldState;
use chrono::Utc;
use std::time::Duration;
use tokio::time::interval;
use tracing::{info, warn};

/// Target ticks per second for the simulation
const TARGET_TPS: f64 = 20.0;
const TICK_DURATION: Duration = Duration::from_micros((1_000_000.0 / TARGET_TPS) as u64);

/// Main simulation loop
pub struct SimulationLoop {
    world_state: std::sync::Arc<tokio::sync::RwLock<WorldState>>,
    session_store: SessionStore,
    running: bool,
}

impl SimulationLoop {
    pub fn new(
        world_state: std::sync::Arc<tokio::sync::RwLock<WorldState>>,
        session_store: SessionStore,
    ) -> Self {
        Self {
            world_state,
            session_store,
            running: false,
        }
    }

    /// Start the simulation loop
    pub async fn run(&mut self) {
        self.running = true;
        info!("Starting simulation loop");
        let mut timer = interval(TICK_DURATION);

        loop {
            if !self.running {
                break;
            }
            timer.tick().await;
            self.process_tick().await;
        }

        info!("Simulation loop stopped");
    }

    /// Stop the simulation loop
    pub fn stop(&mut self) {
        self.running = false;
    }

    async fn process_tick(&self) {
        {
            let mut world = self.world_state.write().await;
            world.update(TICK_DURATION.as_secs_f64());

            for intent in world.drain_movement_intents() {
                let sim_intent = SimMovementIntent {
                    player_id: intent.player_id,
                    target_x: intent.target_x,
                    target_y: intent.target_y,
                    target_z: intent.target_z,
                    speed_modifier: intent.speed_modifier,
                };
                let player_id = sim_intent.player_id;

                if let Err(err) = MovementSystem::process_movement_intent(&mut world, sim_intent) {
                    warn!(player_id, ?err, "Movement intent failed validation");
                }
            }

            for (attacker_id, action) in world.drain_combat_actions() {
                let result = CombatSystem::process_combat_action(&mut world, attacker_id, action);
                if !result.success {
                    warn!(
                        attacker = attacker_id,
                        error = ?result.error_message,
                        "Combat action failed"
                    );
                }
            }
        }

        self.broadcast_world_snapshots().await;
    }

    async fn broadcast_world_snapshots(&self) {
        let sessions = self.session_store.get_active_sessions().await;
        if sessions.is_empty() {
            return;
        }

        let mut snapshots = Vec::with_capacity(sessions.len());
        {
            let world = self.world_state.read().await;
            for session in &sessions {
                if let Some(snapshot) = build_world_snapshot(&world, session) {
                    snapshots.push((session.id, snapshot));
                }
            }
        }

        for (session_id, snapshot) in snapshots {
            let envelope = Envelope {
                sequence_id: snapshot.snapshot_id as u32,
                timestamp: Utc::now().timestamp_millis() as u64,
                payload: Payload::WorldSnapshot(snapshot),
            };

            if self
                .session_store
                .send_envelope(&session_id, envelope)
                .await
                .is_err()
            {
                warn!("Failed to send world snapshot to session {}", session_id);
            }
        }
    }

    /// Get reference to world state (async)
    pub async fn world_state(&self) -> tokio::sync::RwLockReadGuard<'_, WorldState> {
        self.world_state.read().await
    }

    /// Get mutable reference to world state (async)
    pub async fn world_state_mut(&self) -> tokio::sync::RwLockWriteGuard<'_, WorldState> {
        self.world_state.write().await
    }
}

fn build_world_snapshot(
    world: &WorldState,
    session: &crate::network::Session,
) -> Option<WorldSnapshot> {
    let player_id = session.player_id?;
    let zone_id = world.get_player_zone_id(player_id)?;
    let zone = world.get_zone(zone_id)?;

    let entities = zone
        .entities
        .get_all_entities()
        .into_iter()
        .filter_map(entity_to_wire)
        .collect::<Vec<_>>();

    let snapshot_id_i64 = Utc::now().timestamp_millis();
    let snapshot_id = if snapshot_id_i64.is_negative() {
        0
    } else {
        snapshot_id_i64 as u64
    };

    Some(WorldSnapshot {
        snapshot_id,
        entities,
        player_entity_id: player_id,
        zone_name: zone.name.clone(),
    })
}

fn entity_to_wire(entity: &GameEntity) -> Option<messages::Entity> {
    let position = entity.position.as_ref()?;

    let movement_state = determine_movement_state(entity);
    let health_percent = entity
        .health
        .as_ref()
        .map(|health| {
            if health.maximum == 0 {
                1.0
            } else {
                (health.current as f32 / health.maximum as f32).clamp(0.0, 1.0)
            }
        })
        .unwrap_or(1.0);

    Some(messages::Entity {
        id: entity.id,
        entity_type: entity_type_name(&entity.entity_type).to_string(),
        position: Vector3 {
            x: position.x,
            y: position.y,
            z: position.z,
        },
        rotation: Vector3 {
            x: 0.0,
            y: position.rotation,
            z: 0.0,
        },
        state: messages::EntityState {
            movement_state,
            health_percent,
            display_name: entity.name.clone(),
        },
    })
}

fn determine_movement_state(entity: &GameEntity) -> MovementState {
    if entity
        .health
        .as_ref()
        .map(|health| health.current == 0)
        .unwrap_or(false)
    {
        MovementState::Dead
    } else if entity
        .movement
        .as_ref()
        .map(|movement| movement.is_moving)
        .unwrap_or(false)
    {
        MovementState::Running
    } else {
        MovementState::Idle
    }
}

fn entity_type_name(entity_type: &EntityType) -> &'static str {
    match entity_type {
        EntityType::Player => "player",
        EntityType::Mob => "mob",
        EntityType::Npc => "npc",
        EntityType::WorldObject => "object",
    }
}
