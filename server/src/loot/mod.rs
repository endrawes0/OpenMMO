//! Loot system for managing item drops and rewards

use crate::entities::EntityId;
use crate::items::{ItemId, ItemInstance};
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Loot table entry defining an item drop
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootEntry {
    pub item_id: ItemId,
    pub drop_chance: f32, // 0.0 to 1.0 (percentage)
    pub min_quantity: u32,
    pub max_quantity: u32,
    pub conditions: Vec<LootCondition>,
}

impl LootEntry {
    pub fn new(item_id: ItemId, drop_chance: f32) -> Self {
        Self {
            item_id,
            drop_chance,
            min_quantity: 1,
            max_quantity: 1,
            conditions: Vec::new(),
        }
    }

    pub fn with_quantity(mut self, min: u32, max: u32) -> Self {
        self.min_quantity = min;
        self.max_quantity = max;
        self
    }

    pub fn with_condition(mut self, condition: LootCondition) -> Self {
        self.conditions.push(condition);
        self
    }

    pub fn should_drop(&self, rng: &mut impl Rng, context: &LootContext) -> bool {
        // Check drop chance
        if rng.gen::<f32>() >= self.drop_chance {
            return false;
        }

        // Check conditions
        for condition in &self.conditions {
            if !condition.check(context) {
                return false;
            }
        }

        true
    }

    pub fn generate_quantity(&self, rng: &mut impl Rng) -> u32 {
        if self.min_quantity == self.max_quantity {
            self.min_quantity
        } else {
            rng.gen_range(self.min_quantity..=self.max_quantity)
        }
    }
}

/// Conditions for loot drops
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LootCondition {
    MinLevel(u32),       // Player must be at least this level
    MaxLevel(u32),       // Player must be at most this level
    Class(String),       // Player must be this class
    QuestCompleted(u32), // Player must have completed this quest
    QuestActive(u32),    // Player must have this quest active
    ItemOwned(ItemId),   // Player must own this item
    RareDrop,            // Special rare drop condition
}

impl LootCondition {
    pub fn check(&self, context: &LootContext) -> bool {
        match self {
            LootCondition::MinLevel(required) => context.player_level >= *required,
            LootCondition::MaxLevel(required) => context.player_level <= *required,
            LootCondition::Class(required) => context.player_class == *required,
            LootCondition::QuestCompleted(quest_id) => context.completed_quests.contains(quest_id),
            LootCondition::QuestActive(quest_id) => context.active_quests.contains(quest_id),
            LootCondition::ItemOwned(item_id) => context.inventory_items.contains(item_id),
            LootCondition::RareDrop => context.rare_drop_roll, // Special flag for rare drops
        }
    }
}

/// Context for loot generation
#[derive(Debug, Clone)]
pub struct LootContext {
    pub player_id: EntityId,
    pub player_level: u32,
    pub player_class: String,
    pub active_quests: Vec<u32>,
    pub completed_quests: Vec<u32>,
    pub inventory_items: Vec<ItemId>,
    pub rare_drop_roll: bool, // Whether rare drop condition was met
}

impl LootContext {
    pub fn new(player_id: EntityId, player_level: u32, player_class: String) -> Self {
        Self {
            player_id,
            player_level,
            player_class,
            active_quests: Vec::new(),
            completed_quests: Vec::new(),
            inventory_items: Vec::new(),
            rare_drop_roll: false,
        }
    }

    pub fn with_quests(mut self, active: Vec<u32>, completed: Vec<u32>) -> Self {
        self.active_quests = active;
        self.completed_quests = completed;
        self
    }

    pub fn with_inventory(mut self, items: Vec<ItemId>) -> Self {
        self.inventory_items = items;
        self
    }

    pub fn with_rare_drop(mut self, rare: bool) -> Self {
        self.rare_drop_roll = rare;
        self
    }
}

/// Loot table containing multiple loot entries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootTable {
    pub id: u32,
    pub name: String,
    pub entries: Vec<LootEntry>,
    pub guaranteed_drops: Vec<ItemId>, // Items that always drop
    pub gold_min: u32,
    pub gold_max: u32,
}

impl LootTable {
    pub fn new(id: u32, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            entries: Vec::new(),
            guaranteed_drops: Vec::new(),
            gold_min: 0,
            gold_max: 0,
        }
    }

    pub fn add_entry(mut self, entry: LootEntry) -> Self {
        self.entries.push(entry);
        self
    }

    pub fn add_guaranteed_drop(mut self, item_id: ItemId) -> Self {
        self.guaranteed_drops.push(item_id);
        self
    }

    pub fn with_gold(mut self, min: u32, max: u32) -> Self {
        self.gold_min = min;
        self.gold_max = max;
        self
    }

    /// Generate loot from this table
    pub fn generate_loot(&self, context: &LootContext) -> Vec<LootDrop> {
        let mut rng = rand::thread_rng();
        let mut drops = Vec::new();

        // Add guaranteed drops
        for &item_id in &self.guaranteed_drops {
            drops.push(LootDrop::Item(ItemInstance::new(item_id, 1)));
        }

        // Process loot entries
        for entry in &self.entries {
            if entry.should_drop(&mut rng, context) {
                let quantity = entry.generate_quantity(&mut rng);
                drops.push(LootDrop::Item(ItemInstance::new(entry.item_id, quantity)));
            }
        }

        // Generate gold
        if self.gold_max > 0 {
            let gold_amount = if self.gold_min == self.gold_max {
                self.gold_min
            } else {
                rng.gen_range(self.gold_min..=self.gold_max)
            };
            if gold_amount > 0 {
                drops.push(LootDrop::Gold(gold_amount));
            }
        }

        drops
    }
}

/// Individual loot drop result
#[derive(Debug, Clone)]
pub enum LootDrop {
    Item(ItemInstance),
    Gold(u32),
    Experience(u32),
}

/// Loot system for managing loot tables and generation
pub struct LootSystem {
    tables: HashMap<u32, LootTable>,
}

impl LootSystem {
    pub fn new() -> Self {
        Self {
            tables: HashMap::new(),
        }
    }

    pub fn register_table(&mut self, table: LootTable) {
        self.tables.insert(table.id, table);
    }

    pub fn get_table(&self, id: u32) -> Option<&LootTable> {
        self.tables.get(&id)
    }

    pub fn generate_loot(&self, table_id: u32, context: &LootContext) -> Option<Vec<LootDrop>> {
        self.tables
            .get(&table_id)
            .map(|table| table.generate_loot(context))
    }

    /// Load default loot tables
    pub fn load_defaults(&mut self) {
        // Goblin loot table
        let goblin_table = LootTable::new(1, "Goblin Loot")
            .add_entry(LootEntry::new(200, 0.3).with_quantity(1, 3)) // Health potions
            .add_entry(LootEntry::new(1, 0.1)) // Rusty sword
            .add_entry(LootEntry::new(100, 0.05)) // Cloth shirt
            .with_gold(5, 15);

        self.register_table(goblin_table);

        // Orc loot table
        let orc_table = LootTable::new(2, "Orc Loot")
            .add_entry(LootEntry::new(201, 0.2).with_quantity(1, 2)) // Mana potions
            .add_entry(LootEntry::new(2, 0.15)) // Iron axe
            .add_entry(LootEntry::new(100, 0.1)) // Cloth shirt
            .with_gold(10, 25);

        self.register_table(orc_table);

        // Wolf loot table
        let wolf_table = LootTable::new(3, "Wolf Loot")
            .add_entry(LootEntry::new(200, 0.25).with_quantity(1, 2)) // Health potions
            .with_gold(3, 8);

        self.register_table(wolf_table);
    }
}
