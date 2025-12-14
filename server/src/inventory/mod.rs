//! Inventory management system

use crate::entities::EntityId;
use crate::items::{ItemId, ItemInstance, ItemRegistry};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Inventory slot identifier
pub type SlotId = u32;

/// Inventory system for managing player items
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inventory {
    pub slots: HashMap<SlotId, ItemInstance>,
    pub max_slots: u32,
    pub owner_id: EntityId,
}

impl Inventory {
    pub fn new(owner_id: EntityId, max_slots: u32) -> Self {
        Self {
            slots: HashMap::new(),
            max_slots,
            owner_id,
        }
    }

    /// Add an item to the inventory
    pub fn add_item(
        &mut self,
        item: ItemInstance,
        registry: &ItemRegistry,
    ) -> Result<(), InventoryError> {
        let definition = registry
            .get_item(item.definition_id)
            .ok_or(InventoryError::InvalidItem)?;

        // Try to stack with existing items first
        if item.quantity > 0 {
            for (_slot_id, existing_item) in &mut self.slots {
                if existing_item.is_stackable(&item) && existing_item.can_stack_more(definition) {
                    let can_add = existing_item.stack_limit(definition).min(item.quantity);
                    existing_item.quantity += can_add;
                    let remaining = item.quantity - can_add;

                    if remaining > 0 {
                        // Still have items left, continue to find another slot
                        let mut remaining_item = item.clone();
                        remaining_item.quantity = remaining;
                        return self.add_item_to_new_slot(remaining_item, registry);
                    }
                    return Ok(());
                }
            }
        }

        // No stackable slot found, add to new slot
        self.add_item_to_new_slot(item, registry)
    }

    /// Add item to a new slot
    fn add_item_to_new_slot(
        &mut self,
        item: ItemInstance,
        registry: &ItemRegistry,
    ) -> Result<(), InventoryError> {
        let next_slot = self
            .find_empty_slot()
            .ok_or(InventoryError::InventoryFull)?;

        // Validate the item exists
        registry
            .get_item(item.definition_id)
            .ok_or(InventoryError::InvalidItem)?;

        self.slots.insert(next_slot, item);
        Ok(())
    }

    /// Remove items from inventory
    pub fn remove_item(
        &mut self,
        slot_id: SlotId,
        quantity: u32,
    ) -> Result<ItemInstance, InventoryError> {
        let item = self
            .slots
            .get_mut(&slot_id)
            .ok_or(InventoryError::SlotNotFound)?;

        if item.quantity < quantity {
            return Err(InventoryError::InsufficientQuantity);
        }

        let removed_item = if item.quantity == quantity {
            // Remove entire stack
            self.slots.remove(&slot_id).unwrap()
        } else {
            // Reduce stack size
            item.quantity -= quantity;
            let mut removed = item.clone();
            removed.quantity = quantity;
            removed
        };

        Ok(removed_item)
    }

    /// Move item between slots
    pub fn move_item(&mut self, from_slot: SlotId, to_slot: SlotId) -> Result<(), InventoryError> {
        if from_slot == to_slot {
            return Ok(()); // No-op
        }

        let from_item = self
            .slots
            .remove(&from_slot)
            .ok_or(InventoryError::SlotNotFound)?;

        if let Some(to_item) = self.slots.remove(&to_slot) {
            // Both slots have items, swap them
            self.slots.insert(from_slot, to_item);
            self.slots.insert(to_slot, from_item);
        } else {
            // Target slot is empty, just move
            self.slots.insert(to_slot, from_item);
        }

        Ok(())
    }

    /// Get item in a specific slot
    pub fn get_item(&self, slot_id: SlotId) -> Option<&ItemInstance> {
        self.slots.get(&slot_id)
    }

    /// Get all items in inventory
    pub fn get_all_items(&self) -> Vec<(SlotId, &ItemInstance)> {
        self.slots
            .iter()
            .map(|(slot, item)| (*slot, item))
            .collect()
    }

    /// Count total items of a specific type
    pub fn count_item(&self, item_id: ItemId) -> u32 {
        self.slots
            .values()
            .filter(|item| item.definition_id == item_id)
            .map(|item| item.quantity)
            .sum()
    }

    /// Check if inventory has enough of an item
    pub fn has_item(&self, item_id: ItemId, quantity: u32) -> bool {
        self.count_item(item_id) >= quantity
    }

    /// Find the first empty slot
    pub fn find_empty_slot(&self) -> Option<SlotId> {
        for slot in 0..self.max_slots {
            if !self.slots.contains_key(&slot) {
                return Some(slot);
            }
        }
        None
    }

    /// Get inventory capacity usage
    pub fn used_slots(&self) -> usize {
        self.slots.len()
    }

    /// Check if inventory is full
    pub fn is_full(&self) -> bool {
        self.slots.len() >= self.max_slots as usize
    }

    /// Get total value of all items
    pub fn total_value(&self, registry: &ItemRegistry) -> u32 {
        self.slots
            .values()
            .filter_map(|item| {
                registry
                    .get_item(item.definition_id)
                    .map(|def| def.value * item.quantity)
            })
            .sum()
    }

    /// Clear all items (for testing or character deletion)
    pub fn clear(&mut self) {
        self.slots.clear();
    }
}

/// Inventory operation errors
#[derive(Debug, thiserror::Error)]
pub enum InventoryError {
    #[error("Inventory is full")]
    InventoryFull,

    #[error("Invalid item definition")]
    InvalidItem,

    #[error("Slot not found")]
    SlotNotFound,

    #[error("Insufficient quantity")]
    InsufficientQuantity,

    #[error("Item cannot be stacked")]
    CannotStack,
}
