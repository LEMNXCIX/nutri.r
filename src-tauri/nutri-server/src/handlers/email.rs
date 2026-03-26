use crate::error::ApiError;
use axum::{extract::State, Json};
use nutri_core::services::build_plan_email;
use nutri_core::state::AppState;
use serde::Deserialize;
use std::sync::Arc;

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
    let plan_context = {
        let plan_service = state.plan_service.lock().await;
        plan_service.get_plan_email_context(&req.plan_id)?
    };

    let nutrition = {
        let nutrition_service = state.nutrition_service.lock().await;
        match nutrition_service.get_plan_nutrition(&req.plan_id).await {
            Ok(nutrition) => Some(nutrition),
            Err(error) => {
                log::warn!(
                    "Unable to enrich plan email '{}' with nutrition summary: {}",
                    req.plan_id,
                    error
                );
                None
            }
        }
    };

    let rendered = build_plan_email(&plan_context, nutrition.as_ref());
    let service = state.email_service.lock().await;

    service
        .send_plan_email(&req.target_email, &rendered.subject, rendered.html)
        .await?;

    Ok(Json(()))
}
