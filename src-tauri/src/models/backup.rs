use crate::models::{
    AppConfig, CalendarEntry, ExcludedIngredients, PantryItem, PlanDetail, PlanIndex, PlanMetadata,
    Tag,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct AppBackup {
    pub version: String,
    pub last_updated: String,
    pub config: AppConfig,
    pub plans: Vec<PlanIndex>,
    pub plan_details: Vec<PlanDetail>,
    pub metadata: Vec<PlanMetadata>,
    pub tags: Vec<Tag>,
    pub calendar: Vec<CalendarEntry>,
    pub pantry: Vec<PantryItem>,
    pub excluded_ingredients: ExcludedIngredients,
}
