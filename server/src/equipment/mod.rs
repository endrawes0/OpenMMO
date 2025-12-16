//! Equipment system for managing equipped items

use crate::entities::EntityId;
use crate::items::{EquipmentSlot, ItemDefinition, ItemInstance, ItemRegistry, ItemStats};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Equipment system for managing character equipment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Equipment {
    pub slots: HashMap<EquipmentSlot, ItemInstance>,
    pub owner_id: EntityId,
}

impl Equipment {
    pub fn new(owner_id: EntityId) -> Self {
        Self {
            slots: HashMap::new(),
            owner_id,
        }
    }

    /// Equip an item
    pub fn equip_item(
        &mut self,
        item: ItemInstance,
        slot: EquipmentSlot,
        registry: &ItemRegistry,
    ) -> Result<(), EquipmentError> {
        let definition = registry
            .get_item(item.definition_id)
            .ok_or(EquipmentError::InvalidItem)?;

        // Validate item can be equipped in this slot
        self.validate_equipment_slot(definition, slot)?;

        // Check if slot is already occupied
        if let Some(_existing_item) = self.slots.remove(&slot) {
            // Return existing item (would go to inventory in full implementation)
            // For now, we'll just replace it
        }

        self.slots.insert(slot, item);
        Ok(())
    }

    /// Unequip an item from a slot
    pub fn unequip_item(&mut self, slot: EquipmentSlot) -> Option<ItemInstance> {
        self.slots.remove(&slot)
    }

    /// Get item equipped in a slot
    pub fn get_equipped_item(&self, slot: EquipmentSlot) -> Option<&ItemInstance> {
        self.slots.get(&slot)
    }

    /// Get all equipped items
    pub fn get_all_equipped(&self) -> Vec<(EquipmentSlot, &ItemInstance)> {
        self.slots
            .iter()
            .map(|(slot, item)| (*slot, item))
            .collect()
    }

    /// Calculate total stats from all equipped items
    pub fn calculate_total_stats(&self, registry: &ItemRegistry) -> ItemStats {
        let mut total_stats = ItemStats::default();

        for item in self.slots.values() {
            if let Some(definition) = registry.get_item(item.definition_id) {
                total_stats = total_stats.combine(&definition.stats);
            }
        }

        total_stats
    }

    /// Check if a slot is equipped
    pub fn is_slot_equipped(&self, slot: EquipmentSlot) -> bool {
        self.slots.contains_key(&slot)
    }

    /// Get equipment durability status
    pub fn get_durability_status(&self) -> Vec<(EquipmentSlot, f32)> {
        self.slots
            .iter()
            .filter_map(|(slot, item)| {
                item.durability
                    .as_ref()
                    .map(|dur| (*slot, dur.durability_percentage()))
            })
            .collect()
    }

    /// Repair all equipped items (would cost gold in full implementation)
    pub fn repair_all_equipment(&mut self) {
        for item in self.slots.values_mut() {
            if let Some(durability) = &mut item.durability {
                durability.repair();
            }
        }
    }

    /// Validate that an item can be equipped in a specific slot
    fn validate_equipment_slot(
        &self,
        definition: &ItemDefinition,
        slot: EquipmentSlot,
    ) -> Result<(), EquipmentError> {
        match &definition.category {
            crate::items::ItemCategory::Weapon { .. } => {
                if !slot.is_weapon_slot() {
                    return Err(EquipmentError::WrongSlotType);
                }
            }
            crate::items::ItemCategory::Armor { .. } => {
                if !slot.is_armor_slot() {
                    return Err(EquipmentError::WrongSlotType);
                }
            }
            _ => {
                return Err(EquipmentError::NotEquippable);
            }
        }

        // Check if item is broken
        if let Some(durability) = &definition.durability {
            if durability.is_broken() {
                return Err(EquipmentError::ItemBroken);
            }
        }

        Ok(())
    }

    /// Get weapon damage (for main hand weapon)
    pub fn get_weapon_damage(&self, registry: &ItemRegistry) -> Option<(u32, f32)> {
        self.get_equipped_item(EquipmentSlot::MainHand)
            .and_then(|item| registry.get_item(item.definition_id))
            .and_then(|def| match &def.category {
                crate::items::ItemCategory::Weapon { damage, speed, .. } => Some((*damage, *speed)),
                _ => None,
            })
    }

    /// Get armor defense value
    pub fn get_armor_value(&self, registry: &ItemRegistry) -> u32 {
        self.slots
            .values()
            .filter_map(|item| registry.get_item(item.definition_id))
            .filter_map(|def| match &def.category {
                crate::items::ItemCategory::Armor { defense, .. } => Some(*defense),
                _ => None,
            })
            .sum()
    }
}

/// Equipment operation errors
#[derive(Debug, thiserror::Error)]
pub enum EquipmentError {
    #[error("Invalid item definition")]
    InvalidItem,

    #[error("Item cannot be equipped")]
    NotEquippable,

    #[error("Wrong equipment slot type")]
    WrongSlotType,

    #[error("Item is broken and cannot be equipped")]
    ItemBroken,

    #[error("Slot is already occupied")]
    SlotOccupied,
}
