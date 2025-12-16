//! Item statistics and modifiers

use serde::{Deserialize, Serialize};

/// Item stat bonuses
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ItemStats {
    pub strength: i32,
    pub agility: i32,
    pub intelligence: i32,
    pub defense: i32,
    pub attack_power: i32,
    pub health: i32,
    pub mana: i32,
    pub critical_chance: f32, // Percentage (0.0 - 1.0)
    pub haste: f32,           // Percentage (0.0 - 1.0)
    pub movement_speed: f32,  // Percentage (0.0 - 1.0)
}

impl ItemStats {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_stat(&mut self, stat_type: &str, value: i32) {
        match stat_type {
            "strength" => self.strength += value,
            "agility" => self.agility += value,
            "intelligence" => self.intelligence += value,
            "defense" => self.defense += value,
            "attack_power" => self.attack_power += value,
            "health" => self.health += value,
            "mana" => self.mana += value,
            _ => {} // Unknown stat type, ignore
        }
    }

    pub fn add_percentage_stat(&mut self, stat_type: &str, percentage: f32) {
        match stat_type {
            "critical_chance" => self.critical_chance += percentage,
            "haste" => self.haste += percentage,
            "movement_speed" => self.movement_speed += percentage,
            _ => {} // Unknown stat type, ignore
        }
    }

    /// Combine two ItemStats (useful for equipment calculations)
    pub fn combine(&self, other: &ItemStats) -> ItemStats {
        ItemStats {
            strength: self.strength + other.strength,
            agility: self.agility + other.agility,
            intelligence: self.intelligence + other.intelligence,
            defense: self.defense + other.defense,
            attack_power: self.attack_power + other.attack_power,
            health: self.health + other.health,
            mana: self.mana + other.mana,
            critical_chance: self.critical_chance + other.critical_chance,
            haste: self.haste + other.haste,
            movement_speed: self.movement_speed + other.movement_speed,
        }
    }

    /// Check if stats are all zero (empty item)
    pub fn is_empty(&self) -> bool {
        self.strength == 0
            && self.agility == 0
            && self.intelligence == 0
            && self.defense == 0
            && self.attack_power == 0
            && self.health == 0
            && self.mana == 0
            && (self.critical_chance - 0.0).abs() < f32::EPSILON
            && (self.haste - 0.0).abs() < f32::EPSILON
            && (self.movement_speed - 0.0).abs() < f32::EPSILON
    }
}

/// Item requirements for equipping
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemRequirements {
    pub level: u32,
    pub class: Option<String>, // Specific class requirement
    pub strength: u32,
    pub agility: u32,
    pub intelligence: u32,
}

impl ItemRequirements {
    pub fn new(level: u32) -> Self {
        Self {
            level,
            class: None,
            strength: 0,
            agility: 0,
            intelligence: 0,
        }
    }

    pub fn requires_class(&mut self, class: String) {
        self.class = Some(class);
    }

    pub fn requires_stat(&mut self, stat: &str, value: u32) {
        match stat {
            "strength" => self.strength = value,
            "agility" => self.agility = value,
            "intelligence" => self.intelligence = value,
            _ => {}
        }
    }
}

/// Item durability system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemDurability {
    pub current: u32,
    pub maximum: u32,
}

impl ItemDurability {
    pub fn new(max_durability: u32) -> Self {
        Self {
            current: max_durability,
            maximum: max_durability,
        }
    }

    pub fn is_broken(&self) -> bool {
        self.current == 0
    }

    pub fn repair(&mut self) {
        self.current = self.maximum;
    }

    pub fn damage(&mut self, amount: u32) {
        self.current = self.current.saturating_sub(amount);
    }

    pub fn durability_percentage(&self) -> f32 {
        if self.maximum == 0 {
            0.0
        } else {
            (self.current as f32 / self.maximum as f32) * 100.0
        }
    }
}
