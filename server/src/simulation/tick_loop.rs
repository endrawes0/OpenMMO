//! Main simulation tick loop
//!
//! This module implements the 20 Hz game simulation loop that
//! updates all game systems each tick.

use std::time::{Duration, Instant};
use tokio::time;
use tracing::{info, warn};
use crate::world::WorldState;

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
        info!("Starting simulation loop at {} TPS", TARGET_TPS);

        let mut last_tick = Instant::now();
        let mut tick_count = 0u64;

        while self.running {
            let tick_start = Instant::now();

            // Calculate delta time
            let delta_time = last_tick.elapsed().as_secs_f64();
            last_tick = tick_start;

            // Update world state
            {
                let mut world = self.world_state.write().await;
                world.update(delta_time);
            }

            // TODO: Process movement intents
            // TODO: Process combat actions
            // TODO: Update AI
            // TODO: Handle network sync

            tick_count += 1;

            // Log TPS every 200 ticks (10 seconds at 20 TPS)
            if tick_count % 200 == 0 {
                let elapsed = tick_start.elapsed();
                let tps = 200.0 / elapsed.as_secs_f64();
                info!("Simulation TPS: {:.1}", tps);
            }

            // Sleep to maintain target tick rate
            let elapsed = tick_start.elapsed();
            if elapsed < TICK_DURATION {
                time::sleep(TICK_DURATION - elapsed).await;
            } else {
                warn!("Simulation tick took too long: {:?}", elapsed);
            }
        }

        info!("Simulation loop stopped");
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