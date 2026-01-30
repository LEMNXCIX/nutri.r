use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UIPreferences {
    pub theme: String, // "light" or "dark"
    #[serde(alias = "primary_color")]
    pub primary_color: String,
}
