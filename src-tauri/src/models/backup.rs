use crate::models::{
    AppConfig, CalendarEntry, ExcludedIngredients, PantryItem, PlanDetail, PlanIndex, PlanMetadata,
    Tag, WaterRecord,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    #[serde(default)]
    pub water: HashMap<String, WaterRecord>,
}
