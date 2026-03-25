pub mod achievement;
pub mod backup;
pub mod calendar;
pub mod config;
pub mod ingredient;
pub mod metadata;
pub mod nutrition;
pub mod ollama;
pub mod pantry;
pub mod plan;
pub mod preferences;
pub mod search;
pub mod shopping;
pub mod statistics;
pub mod tag;
pub mod water;

// Re-export commonly used types
pub use achievement::Achievement;
pub use backup::AppBackup;
pub use calendar::{CalendarEntry, MealType};
pub use config::AppConfig;
pub use ingredient::ExcludedIngredients;
pub use metadata::PlanMetadata;
pub use nutrition::{NutritionalInfo, PlanNutrition};
pub use ollama::{
    ChatMessage, OllamaChatRequest, OllamaChatResponse, OllamaListResponse, OllamaModelInfo,
};
pub use pantry::PantryItem;
pub use plan::{
    derive_plan_display_name, PlanDetail, PlanIndex, RecipeSuggestion, StructuredDay,
    StructuredPlan, StructuredRecipe, VariationType, WeeklyMealInfo,
};
pub use preferences::UIPreferences;
pub use search::SearchFilters;
pub use shopping::{ShoppingItem, ShoppingList};
pub use statistics::{IngredientTrend, MonthlyData, Statistics};
pub use tag::Tag;
pub use water::WaterRecord;
