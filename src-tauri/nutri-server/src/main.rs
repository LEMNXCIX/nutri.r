use axum::{
    routing::{get, post},
    Router,
};
use nutri_core::state::AppState;
use std::{net::SocketAddr, path::PathBuf, sync::Arc};

mod error;
mod handlers;

use handlers::{plans, system};

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    // Initialize AppState
    // Use a 'data' directory in the current folder for server storage
    let data_dir = PathBuf::from("data");
    if !data_dir.exists() {
        std::fs::create_dir_all(&data_dir).expect("failed to create data dir");
    }
    
    let app_state = AppState::new(data_dir);
    // Since AppState new returns struct, we likely need to wrap it in Arc if State expects Arc?
    // nutri_core::state::AppState struct fields are Arcs.
    // But Axum State usually wraps the whole thing.
    // The handler expects State<Arc<AppState>>.
    // So I need `Arc::new(app_state)`.
    // Wait, AppState fields are Arcs. `Clone` on AppState is cheap.
    // But Mutexes inside are shared.
    // Let's check `AppState` definition.
    // It derives `Clone`.
    // So `State<AppState>` is also fine if I update handler signature.
    // BUT explicit Arc is often better for Axum state to be really sure it's shared.
    // I used `State<Arc<AppState>>` in handlers. So I must wrap in Arc.

    let shared_state = Arc::new(app_state);

    let app = Router::new()
        .route("/health", get(system::health_check))
        .route("/vault", get(handlers::sync::get_vault).post(handlers::sync::update_vault))
        .route("/api/plans", get(plans::list_plans))
        .route("/api/plans/generate", post(plans::generate_plan))
        .route("/api/plans/:id", get(plans::get_plan))
        .with_state(shared_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    tracing::info!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
