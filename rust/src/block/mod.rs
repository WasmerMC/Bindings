use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
pub struct BlockSettings {
    pub burnable: bool,
    pub collidable: bool,
    pub liquid: bool,
    pub luminance: i32,
    pub requires_tool: bool,
    pub resistance: f32,
    pub break_by_hand: bool,
    pub break_instantly: bool,
    pub map_color: MapColor
}

#[derive(Serialize, Deserialize)]
pub enum MapColor {
    BLACK = 0
}