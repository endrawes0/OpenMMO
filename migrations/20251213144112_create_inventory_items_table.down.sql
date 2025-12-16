DROP TRIGGER IF EXISTS update_inventory_items_updated_at ON inventory_items;
DROP INDEX IF EXISTS idx_inventory_items_slot_position;
DROP INDEX IF EXISTS idx_inventory_items_is_equipped;
DROP INDEX IF EXISTS idx_inventory_items_item_type;
DROP INDEX IF EXISTS idx_inventory_items_item_id;
DROP INDEX IF EXISTS idx_inventory_items_character_id;
DROP TABLE IF EXISTS inventory_items;