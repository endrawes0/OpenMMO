//! Core simulation loop
//!
//! This module implements the main game simulation loop that runs
//! at 20 Hz and updates all game systems.

pub mod tick_loop;
pub mod movement_system;
pub mod combat_system;

pub use tick_loop::*;
pub use movement_system::*;
pub use combat_system::*;