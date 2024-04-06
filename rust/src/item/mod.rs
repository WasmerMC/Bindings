use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
pub struct ItemSettings {
    pub max_count: Option<i32>,
    pub max_damage_if_absent: Option<i32>,
    pub max_damage: Option<i32>,
    pub rarity: Option<Rarity>,
    pub fireproof: Option<bool>
}

#[derive(Serialize, Deserialize)]
pub enum Rarity {
    COMMON = 0,
    UNCOMMON = 1,
    RARE = 2,
    EPiC = 3
}