//! Account management system
//!
//! This module provides account registration, login, and management functionality
//! with secure password hashing using Argon2.

pub mod errors;
pub mod service;

pub use errors::*;
pub use service::*;
