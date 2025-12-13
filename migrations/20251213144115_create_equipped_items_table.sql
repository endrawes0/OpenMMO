-- Create equipped items table
CREATE TABLE equipped_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    character_id UUID NOT NULL,
    inventory_item_id UUID NOT NULL,
    equipment_slot VARCHAR(30) NOT NULL,
    equipped_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Create indexes for performance
CREATE UNIQUE INDEX idx_equipped_items_character_slot ON equipped_items(character_id, equipment_slot);
CREATE INDEX idx_equipped_items_character_id ON equipped_items(character_id);
CREATE INDEX idx_equipped_items_inventory_item_id ON equipped_items(inventory_item_id);
CREATE INDEX idx_equipped_items_equipment_slot ON equipped_items(equipment_slot);

-- migrate:down
DROP INDEX IF EXISTS idx_equipped_items_equipment_slot;
DROP INDEX IF EXISTS idx_equipped_items_inventory_item_id;
DROP INDEX IF EXISTS idx_equipped_items_character_id;
DROP INDEX IF EXISTS idx_equipped_items_character_slot;
DROP TABLE IF EXISTS equipped_items;
