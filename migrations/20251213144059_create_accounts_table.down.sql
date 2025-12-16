DROP TRIGGER IF EXISTS update_accounts_updated_at ON accounts;
DROP INDEX IF EXISTS idx_accounts_last_login_at;
DROP INDEX IF EXISTS idx_accounts_created_at;
DROP INDEX IF EXISTS idx_accounts_email;
DROP INDEX IF EXISTS idx_accounts_username;
DROP TABLE IF EXISTS accounts;
DROP FUNCTION IF EXISTS update_updated_at_column();