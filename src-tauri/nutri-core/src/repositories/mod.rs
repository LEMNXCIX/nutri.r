pub mod achievement_repository;
pub mod calendar_repository;
pub mod config_repository;
pub mod ingredient_repository;
pub mod metadata_repository;
pub mod pantry_repository;
pub mod plan_repository;
pub mod preferences_repository;
pub mod shopping_repository;
pub mod tag_repository;

// Re-export traits and implementations
pub use achievement_repository::{AchievementRepository, FileAchievementRepository};
pub use calendar_repository::{CalendarRepository, FileCalendarRepository};
pub use config_repository::{ConfigRepository, FileConfigRepository};
pub use ingredient_repository::{FileIngredientRepository, IngredientRepository};
pub use metadata_repository::{FileMetadataRepository, MetadataRepository};
pub use pantry_repository::{FilePantryRepository, PantryRepository};
pub use plan_repository::{FilePlanRepository, PlanRepository};
pub use preferences_repository::{FilePreferencesRepository, PreferencesRepository};
pub use shopping_repository::{FileShoppingListRepository, ShoppingListRepository};
pub use tag_repository::{FileTagRepository, TagRepository};
