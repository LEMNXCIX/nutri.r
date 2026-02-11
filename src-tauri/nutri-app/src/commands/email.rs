use nutri_core::state::AppState;
use tauri::State;

#[tauri::command]
pub async fn send_plan_email(
    state: State<'_, AppState>,
    plan_id: String,
    target_email: String,
) -> Result<(), String> {
    let plan_service = state.plan_service.lock().await;
    let email_service = state.email_service.lock().await;

    let plan = plan_service
        .get_plan_content(&plan_id)
        .map_err(|e| e.to_string())?;

    let subject = format!("Tu Plan Nutricional: {}", plan_id);
    let html_content = format!(
        "<h1>Plán Nutricional</h1><p>Hola,</p><p>Aquí tienes el plan solicitado:</p><pre>{}</pre>",
        plan
    );

    email_service
        .send_plan_email(&target_email, &subject, html_content)
        .await
        .map_err(|e| e.to_string())
}

