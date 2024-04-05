use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
pub struct ItemSettings {
    pub max_count: i32,
    pub max_damage_if_absent: i32,
    pub max_damage: i32,
    pub rarity: Rarity,
    pub fireproof: bool
}

#[derive(Serialize, Deserialize)]
pub enum Rarity {
    COMMON,
    UNCOMMON,
    RARE,
    EPiC
}