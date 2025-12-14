//! Item type definitions and categories

use serde::{Deserialize, Serialize};

/// Unique identifier for items
pub type ItemId = u32;

/// Item rarity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ItemRarity {
    Common = 1,
    Uncommon = 2,
    Rare = 3,
    Epic = 4,
    Legendary = 5,
}

impl ItemRarity {
    pub fn color_code(&self) -> &str {
        match self {
            ItemRarity::Common => "white",
            ItemRarity::Uncommon => "green",
            ItemRarity::Rare => "blue",
            ItemRarity::Epic => "purple",
            ItemRarity::Legendary => "orange",
        }
    }
}

/// Item binding rules
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ItemBinding {
    None,           // Can be traded freely
    BindOnPickup,   // Binds when picked up
    BindOnEquip,    // Binds when equipped
    Soulbound,      // Permanently bound to character
}

/// Equipment slots for items
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EquipmentSlot {
    Head,
    Neck,
    Shoulders,
    Chest,
    Waist,
    Legs,
    Feet,
    Wrists,
    Hands,
    Finger1,
    Finger2,
    Trinket1,
    Trinket2,
    MainHand,
    OffHand,
    Ranged,
}

impl EquipmentSlot {
    pub fn is_weapon_slot(&self) -> bool {
        matches!(self, EquipmentSlot::MainHand | EquipmentSlot::OffHand | EquipmentSlot::Ranged)
    }

    pub fn is_armor_slot(&self) -> bool {
        matches!(self,
            EquipmentSlot::Head | EquipmentSlot::Neck | EquipmentSlot::Shoulders |
            EquipmentSlot::Chest | EquipmentSlot::Waist | EquipmentSlot::Legs |
            EquipmentSlot::Feet | EquipmentSlot::Wrists | EquipmentSlot::Hands
        )
    }

    pub fn is_jewelry_slot(&self) -> bool {
        matches!(self,
            EquipmentSlot::Finger1 | EquipmentSlot::Finger2 |
            EquipmentSlot::Trinket1 | EquipmentSlot::Trinket2
        )
    }
}

/// Item categories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemCategory {
    Weapon { weapon_type: WeaponType, damage: u32, speed: f32 },
    Armor { armor_type: ArmorType, defense: u32 },
    Consumable { consumable_type: ConsumableType, effect: ConsumableEffect },
    Quest { quest_id: u32 },
    Miscellaneous,
}

/// Weapon types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WeaponType {
    Sword,
    Axe,
    Mace,
    Dagger,
    Staff,
    Bow,
    Crossbow,
    Wand,
}

/// Armor types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArmorType {
    Cloth,
    Leather,
    Mail,
    Plate,
}

/// Consumable types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsumableType {
    HealthPotion,
    ManaPotion,
    Food,
    Drink,
    Scroll,
    Elixir,
}

/// Consumable effects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsumableEffect {
    RestoreHealth { amount: u32 },
    RestoreMana { amount: u32 },
    RestoreBoth { health: u32, mana: u32 },
    Buff { stat_buff: StatBuff, duration: u32 },
    Teleport { zone_id: String, x: f32, y: f32, z: f32 },
}

/// Stat buffs for consumables
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatBuff {
    pub strength: i32,
    pub agility: i32,
    pub intelligence: i32,
    pub defense: i32,
    pub attack_power: i32,
}