use nutri_core::{services::build_plan_email, state::AppState};
use tauri::State;

#[tauri::command]
pub async fn send_plan_email(
    state: State<'_, AppState>,
    plan_id: String,
    target_email: String,
) -> Result<(), String> {
    let plan_context = {
        let plan_service = state.plan_service.lock().await;
        plan_service
            .get_plan_email_context(&plan_id)
            .map_err(|e| e.to_string())?
    };

    let nutrition = {
        let nutrition_service = state.nutrition_service.lock().await;
        match nutrition_service.get_plan_nutrition(&plan_id).await {
            Ok(nutrition) => Some(nutrition),
            Err(error) => {
                log::warn!(
                    "Unable to enrich plan email '{}' with nutrition summary: {}",
                    plan_id,
                    error
                );
                None
            }
        }
    };

    let rendered = build_plan_email(&plan_context, nutrition.as_ref());
    let email_service = state.email_service.lock().await;

    email_service
        .send_plan_email(&target_email, &rendered.subject, rendered.html)
        .await
        .map_err(|e| e.to_string())
}
