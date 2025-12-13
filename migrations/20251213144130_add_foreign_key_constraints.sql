-- Add foreign key constraints between tables
-- This migration runs after all tables are created

-- Add foreign key from characters to accounts
ALTER TABLE characters
ADD CONSTRAINT fk_characters_account_id
FOREIGN KEY (account_id) REFERENCES accounts(id) ON DELETE CASCADE;

-- Add foreign key from character_stats to characters
ALTER TABLE character_stats
ADD CONSTRAINT fk_character_stats_character_id
FOREIGN KEY (character_id) REFERENCES characters(id) ON DELETE CASCADE;

-- Add foreign key from inventory_items to characters
ALTER TABLE inventory_items
ADD CONSTRAINT fk_inventory_items_character_id
FOREIGN KEY (character_id) REFERENCES characters(id) ON DELETE CASCADE;

-- Add foreign key from equipped_items to characters
ALTER TABLE equipped_items
ADD CONSTRAINT fk_equipped_items_character_id
FOREIGN KEY (character_id) REFERENCES characters(id) ON DELETE CASCADE;

-- Add foreign key from equipped_items to inventory_items
ALTER TABLE equipped_items
ADD CONSTRAINT fk_equipped_items_inventory_item_id
FOREIGN KEY (inventory_item_id) REFERENCES inventory_items(id) ON DELETE CASCADE;

-- Add foreign key from progression to characters
ALTER TABLE progression
ADD CONSTRAINT fk_progression_character_id
FOREIGN KEY (character_id) REFERENCES characters(id) ON DELETE CASCADE;

-- Add foreign key from quest_state to characters
ALTER TABLE quest_state
ADD CONSTRAINT fk_quest_state_character_id
FOREIGN KEY (character_id) REFERENCES characters(id) ON DELETE CASCADE;

-- migrate:down
-- Remove foreign key constraints (in reverse order)
ALTER TABLE quest_state DROP CONSTRAINT IF EXISTS fk_quest_state_character_id;
ALTER TABLE progression DROP CONSTRAINT IF EXISTS fk_progression_character_id;
ALTER TABLE equipped_items DROP CONSTRAINT IF EXISTS fk_equipped_items_inventory_item_id;
ALTER TABLE equipped_items DROP CONSTRAINT IF EXISTS fk_equipped_items_character_id;
ALTER TABLE inventory_items DROP CONSTRAINT IF EXISTS fk_inventory_items_character_id;
ALTER TABLE character_stats DROP CONSTRAINT IF EXISTS fk_character_stats_character_id;
ALTER TABLE characters DROP CONSTRAINT IF EXISTS fk_characters_account_id;