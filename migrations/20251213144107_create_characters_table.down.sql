DROP TRIGGER IF EXISTS update_characters_updated_at ON characters;
DROP INDEX IF EXISTS idx_characters_updated_at;
DROP INDEX IF EXISTS idx_characters_is_online;
DROP INDEX IF EXISTS idx_characters_zone_id;
DROP INDEX IF EXISTS idx_characters_level;
DROP INDEX IF EXISTS idx_characters_class;
DROP INDEX IF EXISTS idx_characters_name;
DROP INDEX IF EXISTS idx_characters_account_id;
DROP TABLE IF EXISTS characters;