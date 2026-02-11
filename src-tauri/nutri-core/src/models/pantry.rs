use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PantryItem {
    pub id: String,
    pub name: String,
    pub quantity: f32,
    pub unit: String,
    #[serde(alias = "expiration_date")]
    pub expiration_date: Option<String>, // ISO 8601 (YYYY-MM-DD)
    pub category: String,
}
