DROP TRIGGER IF EXISTS update_quest_state_updated_at ON quest_state;
DROP INDEX IF EXISTS idx_quest_state_completed_at;
DROP INDEX IF EXISTS idx_quest_state_status;
DROP INDEX IF EXISTS idx_quest_state_quest_id;
DROP INDEX IF EXISTS idx_quest_state_character_id;
DROP INDEX IF EXISTS idx_quest_state_character_quest;
DROP TABLE IF EXISTS quest_state;