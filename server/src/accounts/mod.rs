//! Account management system
//!
//! This module provides account registration, login, and management functionality
//! with secure password hashing using Argon2.

pub mod service;
pub mod errors;

pub use service::*;
pub use errors::*;