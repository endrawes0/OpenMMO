//! Item definitions and templates

use crate::items::{
    ItemBinding, ItemCategory, ItemDurability, ItemId, ItemRarity, ItemRequirements, ItemStats,
};
use serde::{Deserialize, Serialize};

/// Complete item definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemDefinition {
    pub id: ItemId,
    pub name: String,
    pub description: String,
    pub icon_path: String,
    pub rarity: ItemRarity,
    pub binding: ItemBinding,
    pub category: ItemCategory,
    pub stats: ItemStats,
    pub requirements: ItemRequirements,
    pub durability: Option<ItemDurability>,
    pub value: u32,      // Gold value
    pub stack_size: u32, // How many can stack in one slot
    pub is_sellable: bool,
    pub is_tradeable: bool,
}

impl ItemDefinition {
    pub fn new(id: ItemId, name: &str, rarity: ItemRarity) -> Self {
        Self {
            id,
            name: name.to_string(),
            description: String::new(),
            icon_path: format!("items/{}.png", id),
            rarity,
            binding: ItemBinding::None,
            category: ItemCategory::Miscellaneous,
            stats: ItemStats::new(),
            requirements: ItemRequirements::new(1),
            durability: None,
            value: 0,
            stack_size: 1,
            is_sellable: true,
            is_tradeable: true,
        }
    }

    pub fn with_description(mut self, description: &str) -> Self {
        self.description = description.to_string();
        self
    }

    pub fn with_binding(mut self, binding: ItemBinding) -> Self {
        self.binding = binding;
        self
    }

    pub fn with_category(mut self, category: ItemCategory) -> Self {
        self.category = category;
        self
    }

    pub fn with_stats(mut self, stats: ItemStats) -> Self {
        self.stats = stats;
        self
    }

    pub fn with_requirements(mut self, requirements: ItemRequirements) -> Self {
        self.requirements = requirements;
        self
    }

    pub fn with_durability(mut self, durability: ItemDurability) -> Self {
        self.durability = Some(durability);
        self
    }

    pub fn with_value(mut self, value: u32) -> Self {
        self.value = value;
        self
    }

    pub fn with_stack_size(mut self, stack_size: u32) -> Self {
        self.stack_size = stack_size;
        self
    }

    pub fn not_sellable(mut self) -> Self {
        self.is_sellable = false;
        self
    }

    pub fn not_tradeable(mut self) -> Self {
        self.is_tradeable = false;
        self
    }

    pub fn can_equip(
        &self,
        character_level: u32,
        character_class: &str,
        character_stats: &ItemStats,
    ) -> bool {
        // Check level requirement
        if character_level < self.requirements.level {
            return false;
        }

        // Check class requirement
        if let Some(required_class) = &self.requirements.class {
            if required_class != character_class {
                return false;
            }
        }

        // Check stat requirements
        if character_stats.strength < self.requirements.strength as i32
            || character_stats.agility < self.requirements.agility as i32
            || character_stats.intelligence < self.requirements.intelligence as i32
        {
            return false;
        }

        true
    }
}

/// Item instance (what players actually have in inventory)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemInstance {
    pub definition_id: ItemId,
    pub quantity: u32,
    pub durability: Option<ItemDurability>,
    pub is_bound: bool,
    pub creator: Option<String>, // For crafted items
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl ItemInstance {
    pub fn new(definition_id: ItemId, quantity: u32) -> Self {
        Self {
            definition_id,
            quantity,
            durability: None,
            is_bound: false,
            creator: None,
            created_at: chrono::Utc::now(),
        }
    }

    pub fn with_durability(mut self, durability: ItemDurability) -> Self {
        self.durability = Some(durability);
        self
    }

    pub fn bind(&mut self) {
        self.is_bound = true;
    }

    pub fn is_stackable(&self, other: &ItemInstance) -> bool {
        self.definition_id == other.definition_id
            && self.is_bound == other.is_bound
            && self.creator == other.creator
    }

    pub fn can_stack_more(&self, definition: &ItemDefinition) -> bool {
        self.quantity < definition.stack_size
    }

    pub fn stack_limit(&self, definition: &ItemDefinition) -> u32 {
        definition.stack_size - self.quantity
    }
}

/// Item registry for managing all item definitions
pub struct ItemRegistry {
    items: std::collections::HashMap<ItemId, ItemDefinition>,
}

impl ItemRegistry {
    pub fn new() -> Self {
        Self {
            items: std::collections::HashMap::new(),
        }
    }

    pub fn register_item(&mut self, item: ItemDefinition) {
        self.items.insert(item.id, item);
    }

    pub fn get_item(&self, id: ItemId) -> Option<&ItemDefinition> {
        self.items.get(&id)
    }

    pub fn get_all_items(&self) -> Vec<&ItemDefinition> {
        self.items.values().collect()
    }

    /// Load default item definitions
    pub fn load_defaults(&mut self) {
        // Weapons
        self.register_item(
            ItemDefinition::new(1, "Rusty Sword", ItemRarity::Common)
                .with_category(ItemCategory::Weapon {
                    weapon_type: crate::items::WeaponType::Sword,
                    damage: 15,
                    speed: 2.0,
                })
                .with_stats(ItemStats {
                    attack_power: 5,
                    ..Default::default()
                })
                .with_requirements(ItemRequirements::new(1))
                .with_durability(ItemDurability::new(50))
                .with_value(10),
        );

        self.register_item(
            ItemDefinition::new(2, "Iron Axe", ItemRarity::Uncommon)
                .with_category(ItemCategory::Weapon {
                    weapon_type: crate::items::WeaponType::Axe,
                    damage: 25,
                    speed: 2.5,
                })
                .with_stats(ItemStats {
                    attack_power: 8,
                    strength: 2,
                    ..Default::default()
                })
                .with_requirements(ItemRequirements::new(5))
                .with_durability(ItemDurability::new(75))
                .with_value(50),
        );

        // Armor
        self.register_item(
            ItemDefinition::new(100, "Cloth Shirt", ItemRarity::Common)
                .with_category(ItemCategory::Armor {
                    armor_type: crate::items::ArmorType::Cloth,
                    defense: 5,
                })
                .with_stats(ItemStats {
                    defense: 3,
                    ..Default::default()
                })
                .with_requirements(ItemRequirements::new(1))
                .with_durability(ItemDurability::new(30))
                .with_value(5),
        );

        // Consumables
        self.register_item(
            ItemDefinition::new(200, "Health Potion", ItemRarity::Common)
                .with_category(ItemCategory::Consumable {
                    consumable_type: crate::items::ConsumableType::HealthPotion,
                    effect: crate::items::ConsumableEffect::RestoreHealth { amount: 50 },
                })
                .with_value(25)
                .with_stack_size(20),
        );

        self.register_item(
            ItemDefinition::new(201, "Mana Potion", ItemRarity::Common)
                .with_category(ItemCategory::Consumable {
                    consumable_type: crate::items::ConsumableType::ManaPotion,
                    effect: crate::items::ConsumableEffect::RestoreMana { amount: 50 },
                })
                .with_value(25)
                .with_stack_size(20),
        );
    }
}
