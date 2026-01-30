use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShoppingList {
    pub id: String,
    #[serde(alias = "plan_id")]
    pub plan_id: String,
    #[serde(alias = "created_at")]
    pub created_at: String,
    pub items: Vec<ShoppingItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShoppingItem {
    pub name: String,
    pub category: String,
    pub quantity: Option<String>,
    #[serde(default)]
    pub checked: bool,
}
