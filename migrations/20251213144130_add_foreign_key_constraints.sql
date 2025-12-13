-- Add foreign key constraints between tables
-- This migration runs after all tables are created

-- Add foreign key from characters to accounts
ALTER TABLE characters
ADD CONSTRAINT fk_characters_account_id
FOREIGN KEY (account_id) REFERENCES accounts(id) ON DELETE CASCADE;

-- migrate:down
-- Remove foreign key constraints
ALTER TABLE characters DROP CONSTRAINT IF EXISTS fk_characters_account_id;