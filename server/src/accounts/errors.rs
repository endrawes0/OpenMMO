//! Account-related error types

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AccountError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Account not found")]
    AccountNotFound,

    #[error("Account already exists")]
    AccountExists,

    #[error("Invalid username: {reason}")]
    InvalidUsername { reason: String },

    #[error("Invalid email: {reason}")]
    InvalidEmail { reason: String },

    #[error("Invalid password: {reason}")]
    InvalidPassword { reason: String },

    #[error("Password hashing failed")]
    PasswordHashingFailed,

    #[error("Password verification failed")]
    PasswordVerificationFailed,

    #[error("Account is banned: {reason}")]
    AccountBanned { reason: String },

    #[error("Account is inactive")]
    AccountInactive,

    #[error("Character limit exceeded for account")]
    CharacterLimitExceeded,

    #[error("Character name already exists")]
    CharacterNameExists,

    #[error("Invalid character class: {class}")]
    InvalidCharacterClass { class: String },
}

pub type AccountResult<T> = Result<T, AccountError>;
