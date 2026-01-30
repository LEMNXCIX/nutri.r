use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct NutritionalInfo {
    pub calories: f32,
    pub protein: f32,       // g
    pub carbohydrates: f32, // g
    pub fat: f32,           // g
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlanNutrition {
    pub plan_id: String,
    pub total_calories: f32,
    pub total_protein: f32,
    pub total_carbs: f32,
    pub total_fat: f32,
    pub breakdown_by_item: std::collections::HashMap<String, NutritionalInfo>,
}
