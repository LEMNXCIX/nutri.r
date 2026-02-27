use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub enum MealType {
    Breakfast,
    #[default]
    Lunch,
    Dinner,
    Snack,
}

impl ToString for MealType {
    fn to_string(&self) -> String {
        match self {
            MealType::Breakfast => "Desayuno".to_string(),
            MealType::Lunch => "Almuerzo".to_string(),
            MealType::Dinner => "Cena".to_string(),
            MealType::Snack => "Snack".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CalendarEntry {
    pub date: String, // ISO date YYYY-MM-DD
    #[serde(alias = "meal_type")]
    pub meal_type: MealType,
    #[serde(alias = "plan_id")]
    pub plan_id: String,
}
