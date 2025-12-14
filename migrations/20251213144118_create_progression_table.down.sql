DROP TRIGGER IF EXISTS update_progression_updated_at ON progression;
DROP INDEX IF EXISTS idx_progression_level;
DROP INDEX IF EXISTS idx_progression_type;
DROP INDEX IF EXISTS idx_progression_character_id;
DROP INDEX IF EXISTS idx_progression_character_type;
DROP TABLE IF EXISTS progression;