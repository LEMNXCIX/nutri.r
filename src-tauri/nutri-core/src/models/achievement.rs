use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AchievementType {
    PlanCreated,
    IngredientExcluded,
    ShoppingListGenerated,
    PlanRated,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Achievement {
    pub id: String,
    pub title: String,
    pub description: String,
    pub icon: String,
    pub unlocked_at: Option<String>,
    pub condition_type: AchievementType,
    pub condition_value: i32,
}
