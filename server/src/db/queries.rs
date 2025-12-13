use super::models::*;
use sqlx::{PgPool, Row};
use uuid::Uuid;
use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Database query failed: {0}")]
    QueryFailed(#[from] sqlx::Error),
    #[error("Account not found")]
    AccountNotFound,
    #[error("Character not found")]
    CharacterNotFound,
    #[error("Username already exists")]
    UsernameExists,
    #[error("Email already exists")]
    EmailExists,
    #[error("Character name already exists")]
    CharacterNameExists,
}

pub struct AccountQueries;

impl AccountQueries {
    /// Create a new account
    pub async fn create_account(
        pool: &PgPool,
        username: &str,
        email: &str,
        password_hash: &str,
    ) -> Result<Account, DatabaseError> {
        let row = sqlx::query(
            r#"
            INSERT INTO accounts (username, email, password_hash)
            VALUES ($1, $2, $3)
            RETURNING id, username, email, password_hash, created_at, updated_at, 
                     last_login_at, is_active, is_banned, ban_reason, ban_expires_at
            "#,
        )
        .bind(username)
        .bind(email)
        .bind(password_hash)
        .fetch_one(pool)
        .await?;

        Ok(Account {
            id: row.get("id"),
            username: row.get("username"),
            email: row.get("email"),
            password_hash: row.get("password_hash"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            last_login_at: row.get("last_login_at"),
            is_active: row.get("is_active"),
            is_banned: row.get("is_banned"),
            ban_reason: row.get("ban_reason"),
            ban_expires_at: row.get("ban_expires_at"),
        })
    }

    /// Find account by username
    pub async fn find_by_username(pool: &PgPool, username: &str) -> Result<Option<Account>, DatabaseError> {
        let row = sqlx::query(
            "SELECT * FROM accounts WHERE username = $1 AND is_active = true"
        )
        .bind(username)
        .fetch_optional(pool)
        .await?;

        if let Some(row) = row {
            Ok(Some(Account {
                id: row.get("id"),
                username: row.get("username"),
                email: row.get("email"),
                password_hash: row.get("password_hash"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                last_login_at: row.get("last_login_at"),
                is_active: row.get("is_active"),
                is_banned: row.get("is_banned"),
                ban_reason: row.get("ban_reason"),
                ban_expires_at: row.get("ban_expires_at"),
            }))
        } else {
            Ok(None)
        }
    }

    /// Find account by ID
    pub async fn find_by_id(pool: &PgPool, account_id: Uuid) -> Result<Option<Account>, DatabaseError> {
        let row = sqlx::query(
            "SELECT * FROM accounts WHERE id = $1 AND is_active = true"
        )
        .bind(account_id)
        .fetch_optional(pool)
        .await?;

        if let Some(row) = row {
            Ok(Some(Account {
                id: row.get("id"),
                username: row.get("username"),
                email: row.get("email"),
                password_hash: row.get("password_hash"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                last_login_at: row.get("last_login_at"),
                is_active: row.get("is_active"),
                is_banned: row.get("is_banned"),
                ban_reason: row.get("ban_reason"),
                ban_expires_at: row.get("ban_expires_at"),
            }))
        } else {
            Ok(None)
        }
    }

    /// Update last login timestamp
    pub async fn update_last_login(pool: &PgPool, account_id: Uuid) -> Result<(), DatabaseError> {
        sqlx::query("UPDATE accounts SET last_login_at = NOW() WHERE id = $1")
            .bind(account_id)
            .execute(pool)
            .await?;

        Ok(())
    }
}

pub struct CharacterQueries;

impl CharacterQueries {
    /// Create a new character
    pub async fn create_character(
        pool: &PgPool,
        account_id: Uuid,
        name: &str,
        class: &str,
        health: i32,
        max_health: i32,
        resource_type: &str,
        resource_value: i32,
        max_resource: i32,
    ) -> Result<Character, DatabaseError> {
        let row = sqlx::query(
            r#"
            INSERT INTO characters (
                account_id, name, class, health, max_health, 
                resource_type, resource_value, max_resource
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING *
            "#,
        )
        .bind(account_id)
        .bind(name)
        .bind(class)
        .bind(health)
        .bind(max_health)
        .bind(resource_type)
        .bind(resource_value)
        .bind(max_resource)
        .fetch_one(pool)
        .await?;

        Ok(Character {
            id: row.get("id"),
            account_id: row.get("account_id"),
            name: row.get("name"),
            class: row.get("class"),
            level: row.get("level"),
            experience: row.get("experience"),
            zone_id: row.get("zone_id"),
            position_x: row.get("position_x"),
            position_y: row.get("position_y"),
            position_z: row.get("position_z"),
            rotation: row.get("rotation"),
            health: row.get("health"),
            max_health: row.get("max_health"),
            resource_type: row.get("resource_type"),
            resource_value: row.get("resource_value"),
            max_resource: row.get("max_resource"),
            is_online: row.get("is_online"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            last_saved_at: row.get("last_saved_at"),
        })
    }

    /// Get characters for an account
    pub async fn get_by_account_id(pool: &PgPool, account_id: Uuid) -> Result<Vec<Character>, DatabaseError> {
        let rows = sqlx::query(
            "SELECT * FROM characters WHERE account_id = $1 ORDER BY created_at"
        )
        .bind(account_id)
        .fetch_all(pool)
        .await?;

        let mut characters = Vec::new();
        for row in rows {
            characters.push(Character {
                id: row.get("id"),
                account_id: row.get("account_id"),
                name: row.get("name"),
                class: row.get("class"),
                level: row.get("level"),
                experience: row.get("experience"),
                zone_id: row.get("zone_id"),
                position_x: row.get("position_x"),
                position_y: row.get("position_y"),
                position_z: row.get("position_z"),
                rotation: row.get("rotation"),
                health: row.get("health"),
                max_health: row.get("max_health"),
                resource_type: row.get("resource_type"),
                resource_value: row.get("resource_value"),
                max_resource: row.get("max_resource"),
                is_online: row.get("is_online"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                last_saved_at: row.get("last_saved_at"),
            });
        }

        Ok(characters)
    }

    /// Find character by ID
    pub async fn find_by_id(pool: &PgPool, character_id: Uuid) -> Result<Option<Character>, DatabaseError> {
        let row = sqlx::query("SELECT * FROM characters WHERE id = $1")
            .bind(character_id)
            .fetch_optional(pool)
            .await?;

        if let Some(row) = row {
            Ok(Some(Character {
                id: row.get("id"),
                account_id: row.get("account_id"),
                name: row.get("name"),
                class: row.get("class"),
                level: row.get("level"),
                experience: row.get("experience"),
                zone_id: row.get("zone_id"),
                position_x: row.get("position_x"),
                position_y: row.get("position_y"),
                position_z: row.get("position_z"),
                rotation: row.get("rotation"),
                health: row.get("health"),
                max_health: row.get("max_health"),
                resource_type: row.get("resource_type"),
                resource_value: row.get("resource_value"),
                max_resource: row.get("max_resource"),
                is_online: row.get("is_online"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                last_saved_at: row.get("last_saved_at"),
            }))
        } else {
            Ok(None)
        }
    }

    /// Update character position
    pub async fn update_position(
        pool: &PgPool,
        character_id: Uuid,
        x: f64,
        y: f64,
        z: f64,
        rotation: f64,
    ) -> Result<(), DatabaseError> {
        sqlx::query(
            r#"
            UPDATE characters 
            SET position_x = $1, position_y = $2, position_z = $3, rotation = $4, updated_at = NOW()
            WHERE id = $5
            "#,
        )
        .bind(x)
        .bind(y)
        .bind(z)
        .bind(rotation)
        .bind(character_id)
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Update character online status
    pub async fn update_online_status(
        pool: &PgPool,
        character_id: Uuid,
        is_online: bool,
    ) -> Result<(), DatabaseError> {
        sqlx::query("UPDATE characters SET is_online = $1, updated_at = NOW() WHERE id = $2")
            .bind(is_online)
            .bind(character_id)
            .execute(pool)
            .await?;

        Ok(())
    }
}