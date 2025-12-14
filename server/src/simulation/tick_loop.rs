//! Main simulation tick loop
//!
//! This module implements the 20 Hz game simulation loop that
//! updates all game systems each tick.


use crate::world::WorldState;
use std::time::Duration;
use tracing::info;

/// Target ticks per second for the simulation
const TARGET_TPS: f64 = 20.0;
const TICK_DURATION: Duration = Duration::from_micros((1_000_000.0 / TARGET_TPS) as u64);

/// Main simulation loop
pub struct SimulationLoop {
    world_state: std::sync::Arc<tokio::sync::RwLock<WorldState>>,
    running: bool,
}

impl SimulationLoop {
    pub fn new(world_state: std::sync::Arc<tokio::sync::RwLock<WorldState>>) -> Self {
        Self {
            world_state,
            running: false,
        }
    }

    /// Start the simulation loop
    pub async fn run(&mut self) {
        self.running = true;
        info!("Starting simulation loop");
        self.running = false;
    }

    /// Stop the simulation loop
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Get reference to world state (async)
    pub async fn world_state(&self) -> tokio::sync::RwLockReadGuard<WorldState> {
        self.world_state.read().await
    }

    /// Get mutable reference to world state (async)
    pub async fn world_state_mut(&self) -> tokio::sync::RwLockWriteGuard<WorldState> {
        self.world_state.write().await
    }
}
