use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub enum MealType {
    Breakfast,
    #[default]
    Lunch,
    Dinner,
    Snack,
}

impl MealType {
    pub fn display_name(&self) -> &'static str {
        match self {
            MealType::Breakfast => "Desayuno",
            MealType::Lunch => "Almuerzo",
            MealType::Dinner => "Cena",
            MealType::Snack => "Snack",
        }
    }

    pub fn key(&self) -> &'static str {
        match self {
            MealType::Breakfast => "breakfast",
            MealType::Lunch => "lunch",
            MealType::Dinner => "dinner",
            MealType::Snack => "snack",
        }
    }

    pub fn from_label(value: &str) -> Option<Self> {
        match value.trim().to_lowercase().as_str() {
            "breakfast" | "desayuno" => Some(MealType::Breakfast),
            "lunch" | "almuerzo" => Some(MealType::Lunch),
            "dinner" | "cena" => Some(MealType::Dinner),
            "snack" | "merienda" | "colacion" | "colación" => Some(MealType::Snack),
            _ => None,
        }
    }
}

impl ToString for MealType {
    fn to_string(&self) -> String {
        self.display_name().to_string()
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
    #[serde(default)]
    pub assignment_id: Option<String>,
    #[serde(default)]
    pub plan_day_index: Option<u8>,
    #[serde(default)]
    pub recipe_id: Option<String>,
}
