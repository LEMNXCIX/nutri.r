use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Statistics {
    pub total_plans: usize,
    pub favorite_plans: usize,
    pub recipes_count: usize,
    pub ingredients_count: usize,
    pub meal_distribution: HashMap<String, usize>,
    pub monthly_activity: Vec<MonthlyData>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MonthlyData {
    pub month: String,
    pub count: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IngredientTrend {
    pub name: String,
    pub count: usize,
}
