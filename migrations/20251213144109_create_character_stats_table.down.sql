DROP TRIGGER IF EXISTS update_character_stats_updated_at ON character_stats;
DROP INDEX IF EXISTS idx_character_stats_name;
DROP INDEX IF EXISTS idx_character_stats_character_id;
DROP INDEX IF EXISTS idx_character_stats_character_stat;
DROP TABLE IF EXISTS character_stats;