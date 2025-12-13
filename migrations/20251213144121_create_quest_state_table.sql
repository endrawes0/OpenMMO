-- Create quest state table
CREATE TABLE quest_state (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    character_id UUID NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    quest_id VARCHAR(100) NOT NULL,
    quest_name VARCHAR(200) NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'inactive',
    progress JSONB NOT NULL DEFAULT '{}',
    started_at TIMESTAMP WITH TIME ZONE,
    completed_at TIMESTAMP WITH TIME ZONE,
    turned_in_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Create indexes for performance
CREATE UNIQUE INDEX idx_quest_state_character_quest ON quest_state(character_id, quest_id);
CREATE INDEX idx_quest_state_character_id ON quest_state(character_id);
CREATE INDEX idx_quest_state_quest_id ON quest_state(quest_id);
CREATE INDEX idx_quest_state_status ON quest_state(status);
CREATE INDEX idx_quest_state_completed_at ON quest_state(completed_at);

-- Add updated_at trigger
CREATE TRIGGER update_quest_state_updated_at
    BEFORE UPDATE ON quest_state
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- migrate:down
DROP TRIGGER IF EXISTS update_quest_state_updated_at ON quest_state;
DROP INDEX IF EXISTS idx_quest_state_completed_at;
DROP INDEX IF EXISTS idx_quest_state_status;
DROP INDEX IF EXISTS idx_quest_state_quest_id;
DROP INDEX IF EXISTS idx_quest_state_character_id;
DROP INDEX IF EXISTS idx_quest_state_character_quest;
DROP TABLE IF EXISTS quest_state;
DROP FUNCTION IF EXISTS update_updated_at_column();
