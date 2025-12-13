-- Create progression table
CREATE TABLE progression (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    character_id UUID NOT NULL,
    progression_type VARCHAR(50) NOT NULL,
    current_value BIGINT NOT NULL DEFAULT 0,
    max_value BIGINT NOT NULL,
    level INTEGER NOT NULL DEFAULT 1,
    experience_to_next_level BIGINT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Create indexes for performance
CREATE UNIQUE INDEX idx_progression_character_type ON progression(character_id, progression_type);
CREATE INDEX idx_progression_character_id ON progression(character_id);
CREATE INDEX idx_progression_type ON progression(progression_type);
CREATE INDEX idx_progression_level ON progression(level);

-- Add updated_at trigger
CREATE TRIGGER update_progression_updated_at
    BEFORE UPDATE ON progression
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- migrate:down
DROP TRIGGER IF EXISTS update_progression_updated_at ON progression;
DROP INDEX IF EXISTS idx_progression_level;
DROP INDEX IF EXISTS idx_progression_type;
DROP INDEX IF EXISTS idx_progression_character_id;
DROP INDEX IF EXISTS idx_progression_character_type;
DROP TABLE IF EXISTS progression;
