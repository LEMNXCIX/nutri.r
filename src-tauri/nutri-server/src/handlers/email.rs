use axum::{
    extract::{State},
    Json,
};
use nutri_core::state::AppState;
use std::sync::Arc;
use crate::error::ApiError;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendEmailRequest {
    pub plan_id: String,
    pub target_email: String,
}

pub async fn send_plan_email(
    State(state): State<Arc<AppState>>,
    Json(req): Json<SendEmailRequest>,
) -> Result<Json<()>, ApiError> {
    let service = state.email_service.lock().await;
    let plan_service = state.plan_service.lock().await;
    
    let content = plan_service.get_plan_content(&req.plan_id)?;
    let subject = format!("Tu Plan Nutricional - {}", req.plan_id);
    
    service.send_plan_email(&req.target_email, &subject, content).await?;
    
    Ok(Json(()))
}
