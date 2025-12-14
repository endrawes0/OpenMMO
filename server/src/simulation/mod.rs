//! Core simulation loop
//!
//! This module implements the main game simulation loop that runs
//! at 20 Hz and updates all game systems.

pub mod combat_system;
pub mod movement_system;
pub mod tick_loop;

pub use combat_system::*;

pub use tick_loop::*;
