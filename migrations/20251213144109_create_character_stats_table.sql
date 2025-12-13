-- Create character stats table
CREATE TABLE character_stats (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    character_id UUID NOT NULL,
    stat_name VARCHAR(50) NOT NULL,
    stat_value INTEGER NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Create indexes for performance
CREATE UNIQUE INDEX idx_character_stats_character_stat ON character_stats(character_id, stat_name);
CREATE INDEX idx_character_stats_character_id ON character_stats(character_id);
CREATE INDEX idx_character_stats_name ON character_stats(stat_name);

-- Add updated_at trigger
CREATE TRIGGER update_character_stats_updated_at
    BEFORE UPDATE ON character_stats
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- migrate:down
DROP TRIGGER IF EXISTS update_character_stats_updated_at ON character_stats;
DROP INDEX IF EXISTS idx_character_stats_name;
DROP INDEX IF EXISTS idx_character_stats_character_id;
DROP INDEX IF EXISTS idx_character_stats_character_stat;
DROP TABLE IF EXISTS character_stats;
