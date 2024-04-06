use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
pub struct BlockSettings {
    pub max_count: Option<i32>,
    pub max_damage_if_absent: Option<i32>,
    pub max_damage: Option<i32>,
    pub fireproof: Option<bool>
}