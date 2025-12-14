//! Account service implementation

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

use regex::Regex;
use sqlx::PgPool;
use uuid::Uuid;

use crate::accounts::{AccountError, AccountResult};
use crate::db::models::{Account, Character};

/// Account service for managing user accounts and authentication
#[derive(Clone)]
pub struct AccountService {
    pool: PgPool,
}

impl AccountService {
    /// Create a new account service
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Register a new account
    pub async fn register(
        &self,
        username: String,
        email: String,
        password: String,
    ) -> AccountResult<Account> {
        // Validate input
        self.validate_username(&username)?;
        self.validate_email(&email)?;
        self.validate_password(&password)?;

        // Check if account already exists
        if self.account_exists(&username, &email).await? {
            return Err(AccountError::AccountExists);
        }

        // Hash password
        let password_hash = self.hash_password(&password)?;

        // Create account
        let account = sqlx::query_as!(
            Account,
            r#"
            INSERT INTO accounts (username, email, password_hash)
            VALUES ($1, $2, $3)
            RETURNING id, username, email, password_hash, created_at, updated_at,
                      last_login_at, is_active, is_banned, ban_reason, ban_expires_at
            "#,
            username,
            email,
            password_hash
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(account)
    }

    /// Authenticate an account (login)
    pub async fn authenticate(
        &self,
        username_or_email: &str,
        password: &str,
    ) -> AccountResult<Account> {
        // Find account
        let account = self.find_account(username_or_email).await?;

        // Check if account is active
        if !account.is_active {
            return Err(AccountError::AccountInactive);
        }

        // Check if account is banned
        if account.is_banned {
            let reason = account
                .ban_reason
                .unwrap_or_else(|| "No reason provided".to_string());
            return Err(AccountError::AccountBanned { reason });
        }

        // Verify password
        self.verify_password(password, &account.password_hash)?;

        // Update last login
        self.update_last_login(account.id).await?;

        Ok(account)
    }

    /// Find an account by username or email
    pub async fn find_account(&self, username_or_email: &str) -> AccountResult<Account> {
        let query = sqlx::query_as!(
            Account,
            r#"
            SELECT id, username, email, password_hash, created_at, updated_at,
                   last_login_at, is_active, is_banned, ban_reason, ban_expires_at
            FROM accounts
            WHERE username = $1 OR email = $1
            "#,
            username_or_email
        );
        let account: Option<Account> = query.fetch_optional(&self.pool).await?;
        let account = account.ok_or(AccountError::AccountNotFound)?;

        Ok(account)
    }

    /// Get account by ID
    pub async fn get_account(&self, account_id: Uuid) -> AccountResult<Account> {
        let query = sqlx::query_as!(
            Account,
            r#"
            SELECT id, username, email, password_hash, created_at, updated_at,
                   last_login_at, is_active, is_banned, ban_reason, ban_expires_at
            FROM accounts
            WHERE id = $1
            "#,
            account_id
        );
        let account: Option<Account> = query.fetch_optional(&self.pool).await?;
        let account = account.ok_or(AccountError::AccountNotFound)?;

        Ok(account)
    }

    /// Get all characters for an account
    pub async fn get_characters(&self, account_id: Uuid) -> AccountResult<Vec<Character>> {
        let characters = sqlx::query_as!(
            Character,
            r#"
            SELECT id, account_id, name, class, level, experience, zone_id,
                   position_x, position_y, position_z, rotation,
                   health, max_health, resource_type, resource_value, max_resource,
                   is_online, created_at, updated_at, last_saved_at
            FROM characters
            WHERE account_id = $1
            ORDER BY created_at
            "#,
            account_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(characters)
    }

    /// Create a new character for an account
    pub async fn create_character(
        &self,
        account_id: Uuid,
        name: String,
        class: String,
    ) -> AccountResult<Character> {
        // Validate character name
        self.validate_character_name(&name)?;

        // Validate character class
        self.validate_character_class(&class)?;

        // Check character limit (max 3 characters per account)
        let character_count = self.get_character_count(account_id).await?;
        if character_count >= 3 {
            return Err(AccountError::CharacterLimitExceeded);
        }

        // Check if name is already taken
        if self.character_name_exists(&name).await? {
            return Err(AccountError::CharacterNameExists);
        }

        // Get class-specific starting stats
        let (max_health, resource_type, max_resource) = self.get_class_starting_stats(&class)?;

        // Create character
        let character = sqlx::query_as!(
            Character,
            r#"
            INSERT INTO characters (account_id, name, class, health, max_health, resource_type, resource_value, max_resource)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, account_id, name, class, level, experience, zone_id,
                      position_x, position_y, position_z, rotation,
                      health, max_health, resource_type, resource_value, max_resource,
                      is_online, created_at, updated_at, last_saved_at
            "#,
            account_id,
            name,
            class,
            max_health,
            max_health,
            resource_type,
            max_resource,
            max_resource
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(character)
    }

    /// Delete a character
    pub async fn delete_character(
        &self,
        account_id: Uuid,
        character_id: Uuid,
    ) -> AccountResult<()> {
        // Verify character belongs to account
        let result: sqlx::postgres::PgQueryResult = sqlx::query!(
            r#"
            DELETE FROM characters
            WHERE id = $1 AND account_id = $2
            "#,
            character_id,
            account_id
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AccountError::AccountNotFound); // Character not found or doesn't belong to account
        }

        Ok(())
    }

    /// Update character online status
    pub async fn set_character_online(
        &self,
        character_id: Uuid,
        online: bool,
    ) -> AccountResult<()> {
        sqlx::query!(
            r#"
            UPDATE characters
            SET is_online = $1, last_saved_at = NOW()
            WHERE id = $2
            "#,
            online,
            character_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // Private helper methods

    fn validate_username(&self, username: &str) -> AccountResult<()> {
        if username.len() < 3 || username.len() > 50 {
            return Err(AccountError::InvalidUsername {
                reason: "Username must be between 3 and 50 characters".to_string(),
            });
        }

        let username_regex = Regex::new(r"^[a-zA-Z0-9_]+$").unwrap();
        if !username_regex.is_match(username) {
            return Err(AccountError::InvalidUsername {
                reason: "Username can only contain letters, numbers, and underscores".to_string(),
            });
        }

        Ok(())
    }

    fn validate_email(&self, email: &str) -> AccountResult<()> {
        if email.len() > 255 {
            return Err(AccountError::InvalidEmail {
                reason: "Email is too long".to_string(),
            });
        }

        let email_regex = Regex::new(r"^[^@]+@[^@]+\.[^@]+$").unwrap();
        if !email_regex.is_match(email) {
            return Err(AccountError::InvalidEmail {
                reason: "Invalid email format".to_string(),
            });
        }

        Ok(())
    }

    fn validate_password(&self, password: &str) -> AccountResult<()> {
        if password.len() < 8 {
            return Err(AccountError::InvalidPassword {
                reason: "Password must be at least 8 characters long".to_string(),
            });
        }

        Ok(())
    }

    fn validate_character_name(&self, name: &str) -> AccountResult<()> {
        if name.len() < 2 || name.len() > 50 {
            return Err(AccountError::InvalidUsername {
                reason: "Character name must be between 2 and 50 characters".to_string(),
            });
        }

        let name_regex = Regex::new(r"^[a-zA-Z0-9\s]+$").unwrap();
        if !name_regex.is_match(name) {
            return Err(AccountError::InvalidUsername {
                reason: "Character name can only contain letters, numbers, and spaces".to_string(),
            });
        }

        Ok(())
    }

    fn validate_character_class(&self, class: &str) -> AccountResult<()> {
        let valid_classes = ["warrior", "mage", "rogue"];
        if !valid_classes.contains(&class.to_lowercase().as_str()) {
            return Err(AccountError::InvalidCharacterClass {
                class: class.to_string(),
            });
        }

        Ok(())
    }

    async fn account_exists(&self, username: &str, email: &str) -> AccountResult<bool> {
        let count = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM accounts
            WHERE username = $1 OR email = $2
            "#,
            username,
            email
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(count.count.unwrap_or(0) > 0)
    }

    async fn character_name_exists(&self, name: &str) -> AccountResult<bool> {
        let count = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM characters
            WHERE name = $1
            "#,
            name
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(count.count.unwrap_or(0) > 0)
    }

    async fn get_character_count(&self, account_id: Uuid) -> AccountResult<i64> {
        let count = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM characters
            WHERE account_id = $1
            "#,
            account_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(count.count.unwrap_or(0))
    }

    fn get_class_starting_stats(&self, class: &str) -> AccountResult<(i32, String, i32)> {
        match class.to_lowercase().as_str() {
            "warrior" => Ok((120, "rage".to_string(), 100)),
            "mage" => Ok((80, "mana".to_string(), 150)),
            "rogue" => Ok((100, "energy".to_string(), 120)),
            _ => Err(AccountError::InvalidCharacterClass {
                class: class.to_string(),
            }),
        }
    }

    fn hash_password(&self, password: &str) -> AccountResult<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| AccountError::PasswordHashingFailed)?
            .to_string();

        Ok(password_hash)
    }

    fn verify_password(&self, password: &str, hash: &str) -> AccountResult<()> {
        let parsed_hash =
            PasswordHash::new(hash).map_err(|_| AccountError::PasswordVerificationFailed)?;

        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_err(|_| AccountError::PasswordVerificationFailed)?;

        Ok(())
    }

    async fn update_last_login(&self, account_id: Uuid) -> AccountResult<()> {
        sqlx::query!(
            r#"
            UPDATE accounts
            SET last_login_at = NOW()
            WHERE id = $1
            "#,
            account_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
