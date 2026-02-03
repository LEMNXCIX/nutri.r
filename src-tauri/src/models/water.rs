use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WaterRecord {
    pub current: f32,
    pub target: f32,
    pub last_updated: String,
}
