use crate::repositories::{
    ConfigRepository, FileAchievementRepository, FileCalendarRepository, FileConfigRepository,
    FileIngredientRepository, FileMetadataRepository, FilePantryRepository, FilePlanRepository,
    FilePreferencesRepository, FileShoppingListRepository, FileTagRepository,
};
use crate::services::{
    AchievementService, CalendarService, EmailService, ImportExportService, IngredientService,
    MetadataService, NutritionService, PantryService, PlanService, SearchService,
    ShoppingListService, StatisticsService, TagService,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::{Mutex, Notify};

// Type aliases for our concrete implementations
pub type AppPlanService = PlanService<
    FilePlanRepository,
    FileConfigRepository,
    FileIngredientRepository,
    FilePantryRepository,
    FileMetadataRepository,
>;
pub type AppIngredientService = IngredientService<FilePlanRepository, FileIngredientRepository>;
pub type AppMetadataService = MetadataService<FileMetadataRepository>;
pub type AppShoppingListService =
    ShoppingListService<FileShoppingListRepository, FilePlanRepository, FileConfigRepository>;
pub type AppCalendarService = CalendarService<FileCalendarRepository>;
pub type AppStatisticsService =
    StatisticsService<FilePlanRepository, FileMetadataRepository, FileCalendarRepository>;
pub type AppNutritionService =
    NutritionService<FilePlanRepository, FileConfigRepository, FileShoppingListRepository>;
pub type AppSearchService = SearchService<FilePlanRepository, FileMetadataRepository>;
pub type AppTagService = TagService<FileTagRepository, FileMetadataRepository>;
pub type AppPantryService = PantryService<FilePantryRepository>;
pub type AppImportExportService = ImportExportService<
    FilePlanRepository,
    FileConfigRepository,
    FileIngredientRepository,
    FileMetadataRepository,
    FileCalendarRepository,
    FileTagRepository,
    FilePantryRepository,
>;

pub type AppAchievementService = AchievementService<
    FileAchievementRepository,
    FilePlanRepository,
    FileShoppingListRepository,
    FileIngredientRepository,
    FileMetadataRepository,
>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SyncStatus {
    Idle,
    Syncing,
    Success,
    Error(String),
}

#[derive(Clone)]
pub struct AppState {
    pub plan_service: Arc<Mutex<AppPlanService>>,
    pub ingredient_service: Arc<Mutex<AppIngredientService>>,
    pub metadata_service: Arc<Mutex<AppMetadataService>>,
    pub shopping_service: Arc<Mutex<AppShoppingListService>>,
    pub calendar_service: Arc<Mutex<AppCalendarService>>,
    pub nutrition_service: Arc<Mutex<AppNutritionService>>,
    pub search_service: Arc<Mutex<AppSearchService>>,
    pub tag_service: Arc<Mutex<AppTagService>>,
    pub pantry_service: Arc<Mutex<AppPantryService>>,
    pub import_export_service: Arc<Mutex<AppImportExportService>>,
    pub email_service: Arc<Mutex<EmailService>>,
    pub achievement_service: Arc<Mutex<AppAchievementService>>,
    pub statistics_service: Arc<AppStatisticsService>,
    pub config_repo: Arc<FileConfigRepository>,
    pub preferences_repo: Arc<FilePreferencesRepository>,
    pub data_dir: PathBuf,
    pub sync_trigger: Arc<Notify>,
    pub last_sync_status: Arc<Mutex<SyncStatus>>,
    pub last_modified: Arc<Mutex<Option<String>>>,
}

impl AppState {
    pub fn new(data_dir: PathBuf) -> Self {
        let config_path = data_dir.join("config.json");
        let preferences_path = data_dir.join("preferences.json");
        let excluded_path = data_dir.join("excluded_ingredients.json");

        let plan_repo = FilePlanRepository::new(data_dir.clone());
        let config_repo = FileConfigRepository::new(config_path.clone());
        let preferences_repo = FilePreferencesRepository::new(preferences_path.clone());
        let achievement_repo = FileAchievementRepository::new(data_dir.join("achievements.json"));
        let ingredient_repo = FileIngredientRepository::new(excluded_path.clone());
        let metadata_repo = FileMetadataRepository::new(data_dir.clone());
        let shopping_repo = FileShoppingListRepository::new(data_dir.clone());
        let calendar_repo = FileCalendarRepository::new(data_dir.clone());

        let pantry_repo = FilePantryRepository::new(data_dir.clone());

        let plan_service =
            PlanService::new(
                plan_repo,
                config_repo.clone(),
                ingredient_repo,
                pantry_repo,
                FileMetadataRepository::new(data_dir.clone()),
            );
        let ingredient_service = IngredientService::new(
            FilePlanRepository::new(data_dir.clone()),
            FileIngredientRepository::new(excluded_path.clone()),
        );
        let metadata_service = MetadataService::new(metadata_repo);
        let shopping_service = ShoppingListService::new(
            shopping_repo,
            FilePlanRepository::new(data_dir.clone()),
            FileConfigRepository::new(config_path.clone()),
        );
        let calendar_service = CalendarService::new(calendar_repo);
        let statistics_service = StatisticsService::new(
            FilePlanRepository::new(data_dir.clone()),
            FileMetadataRepository::new(data_dir.clone()),
            FileCalendarRepository::new(data_dir.clone()),
        );
        let nutrition_service = NutritionService::new(
            FilePlanRepository::new(data_dir.clone()),
            FileConfigRepository::new(config_path.clone()),
            FileShoppingListRepository::new(data_dir.clone()),
            data_dir.clone(),
        );
        let search_service = SearchService::new(
            FilePlanRepository::new(data_dir.clone()),
            FileMetadataRepository::new(data_dir.clone()),
        );
        let tag_service = TagService::new(
            FileTagRepository::new(data_dir.clone()),
            FileMetadataRepository::new(data_dir.clone()),
        );
        let pantry_service = PantryService::new(FilePantryRepository::new(data_dir.clone()));
        let import_export_service = ImportExportService::new(
            FilePlanRepository::new(data_dir.clone()),
            FileConfigRepository::new(config_path.clone()),
            FileIngredientRepository::new(excluded_path.clone()),
            FileMetadataRepository::new(data_dir.clone()),
            FileCalendarRepository::new(data_dir.clone()),
            FileTagRepository::new(data_dir.clone()),
            FilePantryRepository::new(data_dir.clone()),
            data_dir.clone(),
        );

        let email_service = EmailService::new(config_repo.get().unwrap_or_default());

        let achievement_service = AchievementService::new(
            achievement_repo,
            FilePlanRepository::new(data_dir.clone()),
            FileShoppingListRepository::new(data_dir.clone()),
            FileIngredientRepository::new(excluded_path.clone()),
            FileMetadataRepository::new(data_dir.clone()),
        );

        Self {
            plan_service: Arc::new(Mutex::new(plan_service)),
            ingredient_service: Arc::new(Mutex::new(ingredient_service)),
            metadata_service: Arc::new(Mutex::new(metadata_service)),
            shopping_service: Arc::new(Mutex::new(shopping_service)),
            calendar_service: Arc::new(Mutex::new(calendar_service)),
            nutrition_service: Arc::new(Mutex::new(nutrition_service)),
            search_service: Arc::new(Mutex::new(search_service)),
            tag_service: Arc::new(Mutex::new(tag_service)),
            pantry_service: Arc::new(Mutex::new(pantry_service)),
            import_export_service: Arc::new(Mutex::new(import_export_service)),
            email_service: Arc::new(Mutex::new(email_service)),
            achievement_service: Arc::new(Mutex::new(achievement_service)),
            statistics_service: Arc::new(statistics_service),
            config_repo: Arc::new(config_repo),
            preferences_repo: Arc::new(preferences_repo),
            data_dir: data_dir.clone(),
            sync_trigger: Arc::new(Notify::new()),
            last_sync_status: Arc::new(Mutex::new(SyncStatus::Idle)),
            last_modified: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn trigger_sync(&self) {
        let mut last_mod = self.last_modified.lock().await;
        *last_mod = Some(chrono::Utc::now().to_rfc3339());
        self.sync_trigger.notify_one();
    }
}
