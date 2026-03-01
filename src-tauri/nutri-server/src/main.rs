use axum::{
    routing::{delete, get, post},
    Router,
};
use directories::BaseDirs;
use nutri_core::state::AppState;
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

mod error;
mod handlers;
mod scheduler;

use handlers::{
    achievements, calendar, config, email, import_export, ingredients, nutrition, ollama, pantry,
    plans, preferences, shopping, statistics, sync, system, tags, water,
};

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Determine data directory
    let identifier = "com.emele.nutri-r";

    let data_dir = if let Ok(env_path) = std::env::var("NUTRI_DATA_DIR") {
        PathBuf::from(env_path)
    } else if let Some(base_dirs) = BaseDirs::new() {
        #[cfg(target_os = "windows")]
        {
            base_dirs.config_dir().join(identifier)
        }
        #[cfg(not(target_os = "windows"))]
        {
            if let Some(proj_dirs) = directories::ProjectDirs::from("com", "emele", "nutri-r") {
                proj_dirs.data_dir().to_path_buf()
            } else {
                PathBuf::from("data")
            }
        }
    } else {
        PathBuf::from("data")
    };

    tracing::info!("Using data directory: {:?}", data_dir);

    if !data_dir.exists() {
        tracing::warn!("Data directory does not exist: {:?}", data_dir);
    }

    let app_state = AppState::new(data_dir);
    let shared_state = Arc::new(app_state);

    // Start background scheduler
    let scheduler_state = shared_state.clone();
    tokio::spawn(async move {
        scheduler::start_scheduler(scheduler_state).await;
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(system::health_check))
        .route("/api/health", get(system::health_check))
        // Config
        .route(
            "/api/config",
            get(config::get_config).post(config::update_config),
        )
        // Plans
        .route("/api/plans", get(plans::list_plans))
        .route("/api/plans/favorites", get(plans::get_favorite_plans))
        .route("/api/plans/search", post(plans::search_plans))
        .route("/api/plans/generate", post(plans::generate_plan))
        .route("/api/plans/:id", get(plans::get_plan))
        .route("/api/plans/:id/favorite", post(plans::toggle_favorite))
        .route("/api/plans/:id/metadata", get(plans::get_metadata))
        .route("/api/plans/:id/rating", post(plans::set_rating))
        .route("/api/plans/:id/note", post(plans::set_note))
        .route("/api/plans/:id/variation", post(plans::generate_variation))
        // Nutrition
        .route(
            "/api/plans/:id/nutrition",
            get(nutrition::calculate_nutrition),
        )
        // Shopping List
        .route(
            "/api/shopping/:plan_id",
            get(shopping::get_shopping_list).post(shopping::generate_shopping_list),
        )
        .route("/api/shopping/:plan_id/toggle", post(shopping::toggle_item))
        // Calendar
        .route(
            "/api/calendar",
            get(calendar::get_calendar_range)
                .post(calendar::assign_plan)
                .delete(calendar::remove_entry),
        )
        .route("/api/calendar/weekly", post(calendar::assign_weekly_plan))
        // Water
        .route(
            "/api/water/:date",
            get(water::get_water_intake).post(water::update_water_intake),
        )
        .route("/api/water/history", get(water::get_water_history))
        // Statistics
        .route("/api/stats", get(statistics::get_statistics))
        .route("/api/stats/trends", get(statistics::get_ingredient_trends))
        // Ingredients
        .route(
            "/api/ingredients/excluded",
            get(ingredients::get_excluded_ingredients).post(ingredients::save_excluded_ingredients),
        )
        .route(
            "/api/ingredients/stats",
            get(ingredients::get_ingredient_stats),
        )
        .route(
            "/api/ingredients/toggle",
            post(ingredients::toggle_ingredient_exclusion),
        )
        // Tags
        .route("/api/tags", get(tags::get_all_tags).post(tags::create_tag))
        .route("/api/tags/:tag_id", delete(tags::delete_tag))
        .route(
            "/api/tags/:plan_id/:tag_id",
            post(tags::add_tag_to_plan).delete(tags::remove_tag_from_plan),
        )
        // Pantry
        .route(
            "/api/pantry",
            get(pantry::list_pantry_items)
                .post(pantry::add_pantry_item)
                .put(pantry::update_pantry_item),
        )
        .route("/api/pantry/:id", delete(pantry::delete_pantry_item))
        // Achievements
        .route("/api/achievements", get(achievements::get_achievements))
        // Preferences
        .route(
            "/api/preferences",
            get(preferences::get_preferences).post(preferences::save_preferences),
        )
        // Ollama
        .route("/api/ollama/models", get(ollama::list_models))
        // Sync & Backup
        .route("/api/sync", post(sync::perform_sync))
        .route("/api/sync/status", get(sync::get_sync_status))
        .route("/api/sync/pull", post(sync::pull_from_server))
        .route("/api/sync/push", post(sync::push_to_server))
        .route(
            "/api/sync/vault",
            get(sync::get_vault).post(sync::update_vault),
        )
        .route("/api/backup/export", get(import_export::export_data))
        .route("/api/backup/import", post(import_export::import_data))
        // Email
        .route("/api/email/send", post(email::send_plan_email))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(cors)
        .with_state(shared_state);

    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3001);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
