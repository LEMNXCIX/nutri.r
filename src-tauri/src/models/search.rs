use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SearchFilters {
    #[serde(default)]
    pub query: Option<String>,
    #[serde(default)]
    pub only_favorites: bool,
    #[serde(default)]
    pub min_rating: Option<u8>,
    #[serde(default)]
    pub protein_contains: Option<String>,
}
