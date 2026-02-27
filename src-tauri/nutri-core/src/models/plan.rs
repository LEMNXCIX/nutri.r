use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WeeklyMealInfo {
    #[serde(alias = "day_index")]
    pub day_index: u8,
    #[serde(alias = "meal_type")]
    pub meal_type: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlanIndex {
    pub id: String,
    #[serde(alias = "date", alias = "created_at")]
    pub fecha: String,
    #[serde(alias = "proteins", alias = "protein_list")]
    pub proteinas: Vec<String>,
    #[serde(alias = "sent", alias = "is_sent")]
    pub enviado: bool,
    #[serde(default, alias = "is_favorite")]
    pub is_favorite: bool,
    #[serde(default)]
    pub rating: Option<u8>,
    #[serde(default)]
    pub weekly_structure: Option<Vec<WeeklyMealInfo>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlanDetail {
    pub id: String,
    pub markdown_content: String,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum VariationType {
    Vegan,
    Keto,
    GlutenFree,
    LowCarb,
    HighProtein,
}

impl std::fmt::Display for VariationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VariationType::Vegan => write!(f, "Vegano"),
            VariationType::Keto => write!(f, "Keto"),
            VariationType::GlutenFree => write!(f, "Sin Gluten"),
            VariationType::LowCarb => write!(f, "Bajo en Carbohidratos"),
            VariationType::HighProtein => write!(f, "Alto en Proteínas"),
        }
    }
}
