-- Create characters table
CREATE TABLE characters (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id UUID NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    name VARCHAR(50) UNIQUE NOT NULL,
    class VARCHAR(30) NOT NULL,
    level INTEGER NOT NULL DEFAULT 1,
    experience BIGINT NOT NULL DEFAULT 0,
    zone_id VARCHAR(50) NOT NULL DEFAULT 'starter_zone',
    position_x FLOAT NOT NULL DEFAULT 0.0,
    position_y FLOAT NOT NULL DEFAULT 0.0,
    position_z FLOAT NOT NULL DEFAULT 0.0,
    rotation FLOAT NOT NULL DEFAULT 0.0,
    health INTEGER NOT NULL,
    max_health INTEGER NOT NULL,
    resource_type VARCHAR(20) NOT NULL,
    resource_value INTEGER NOT NULL,
    max_resource INTEGER NOT NULL,
    is_online BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    last_saved_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Create indexes for performance
CREATE INDEX idx_characters_account_id ON characters(account_id);
CREATE INDEX idx_characters_name ON characters(name);
CREATE INDEX idx_characters_class ON characters(class);
CREATE INDEX idx_characters_level ON characters(level);
CREATE INDEX idx_characters_zone_id ON characters(zone_id);
CREATE INDEX idx_characters_is_online ON characters(is_online);
CREATE INDEX idx_characters_updated_at ON characters(updated_at);

-- Add updated_at trigger
CREATE TRIGGER update_characters_updated_at 
    BEFORE UPDATE ON characters 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
