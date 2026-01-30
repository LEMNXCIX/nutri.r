use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlanMetadata {
    #[serde(alias = "plan_id")]
    pub plan_id: String,
    #[serde(alias = "is_favorite")]
    pub is_favorite: bool,
    pub rating: Option<u8>, // 1-5
    pub notes: String,
    pub tags: Vec<String>,
}
