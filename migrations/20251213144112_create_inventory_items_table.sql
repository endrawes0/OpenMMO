-- Create inventory items table
CREATE TABLE inventory_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    character_id UUID NOT NULL,
    item_id VARCHAR(100) NOT NULL,
    item_name VARCHAR(100) NOT NULL,
    item_type VARCHAR(50) NOT NULL,
    item_slot VARCHAR(30),
    quantity INTEGER NOT NULL DEFAULT 1,
    quality VARCHAR(20) NOT NULL DEFAULT 'common',
    item_level INTEGER NOT NULL DEFAULT 1,
    stats JSONB,
    is_bound BOOLEAN NOT NULL DEFAULT false,
    is_equipped BOOLEAN NOT NULL DEFAULT false,
    slot_position INTEGER,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Create indexes for performance
CREATE INDEX idx_inventory_items_character_id ON inventory_items(character_id);
CREATE INDEX idx_inventory_items_item_id ON inventory_items(item_id);
CREATE INDEX idx_inventory_items_item_type ON inventory_items(item_type);
CREATE INDEX idx_inventory_items_is_equipped ON inventory_items(is_equipped);
CREATE INDEX idx_inventory_items_slot_position ON inventory_items(slot_position);

-- Add updated_at trigger
CREATE TRIGGER update_inventory_items_updated_at
    BEFORE UPDATE ON inventory_items
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- migrate:down
DROP TRIGGER IF EXISTS update_inventory_items_updated_at ON inventory_items;
DROP INDEX IF EXISTS idx_inventory_items_slot_position;
DROP INDEX IF EXISTS idx_inventory_items_is_equipped;
DROP INDEX IF EXISTS idx_inventory_items_item_type;
DROP INDEX IF EXISTS idx_inventory_items_item_id;
DROP INDEX IF EXISTS idx_inventory_items_character_id;
DROP TABLE IF EXISTS inventory_items;
