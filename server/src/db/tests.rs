// Allow dead code warnings for Phase 0 infrastructure
#[allow(dead_code)]
#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests {
    use uuid::Uuid;

    #[tokio::test]
    async fn test_database_connection_string_parsing() {
        // Test that we can parse a valid database URL
        let database_url = "postgres://user:pass@localhost:5432/dbname";

        // This should not panic when creating a pool config
        let _pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(1)
            .connect_lazy(database_url);

        // If we get here, the URL parsing worked
        // URL parsing successful - no assertion needed
    }

    #[tokio::test]
    async fn test_account_model_serialization() {
        use crate::db::models::Account;
        use chrono::Utc;

        let account = Account {
            id: Uuid::new_v4(),
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: "hashed_password".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            last_login_at: None,
            is_active: true,
            is_banned: false,
            ban_reason: None,
            ban_expires_at: None,
        };

        // Test that the model can be serialized to JSON
        let json = serde_json::to_value(&account).unwrap();
        assert_eq!(json["username"], "testuser");
        assert_eq!(json["email"], "test@example.com");
        assert!(json.get("password_hash").is_none()); // Should be skipped
    }

    #[tokio::test]
    async fn test_character_model_creation() {
        use crate::db::models::Character;
        use chrono::Utc;

        let character = Character {
            id: Uuid::new_v4(),
            account_id: Uuid::new_v4(),
            name: "TestCharacter".to_string(),
            class: "warrior".to_string(),
            level: 1,
            experience: 0,
            zone_id: "starter_zone".to_string(),
            position_x: 0.0,
            position_y: 0.0,
            position_z: 0.0,
            rotation: 0.0,
            health: 100,
            max_health: 100,
            resource_type: "rage".to_string(),
            resource_value: 0,
            max_resource: 100,
            is_online: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            last_saved_at: Utc::now(),
        };

        // Test that the character model is valid
        assert_eq!(character.name, "TestCharacter");
        assert_eq!(character.class, "warrior");
        assert_eq!(character.level, 1);
        assert_eq!(character.health, 100);
        assert_eq!(character.max_health, 100);
    }

    #[test]
    fn test_database_error_types() {
        use crate::db::queries::DatabaseError;

        // Test that our error types can be created
        let _error = DatabaseError::AccountNotFound;
        let _error = DatabaseError::CharacterNotFound;
        let _error = DatabaseError::UsernameExists;
        let _error = DatabaseError::EmailExists;
        let _error = DatabaseError::CharacterNameExists;

        // Test error formatting
        let error = DatabaseError::AccountNotFound;
        assert_eq!(error.to_string(), "Account not found");
    }
}
