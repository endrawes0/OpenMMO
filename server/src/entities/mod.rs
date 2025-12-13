//! Entity Component System for game entities
//!
//! This module implements an ECS pattern for managing game entities including
//! players, NPCs, mobs, and world objects.

pub mod components;
pub mod entities;
pub mod system;

pub use components::*;
pub use entities::*;
pub use system::*;