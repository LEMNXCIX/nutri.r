use chrono::Datelike;
use nutri_core::repositories::ConfigRepository;
use nutri_core::state::AppState;
use std::sync::Arc;
use tokio_cron_scheduler::{Job, JobScheduler};
use tracing::{error, info};

pub async fn start_scheduler(state: Arc<AppState>) {
    let sched = JobScheduler::new()
        .await
        .expect("Failed to create scheduler");

    // Read initial config
    let config = state.config_repo.get().unwrap_or_default();

    if config.auto_generate_plan {
        info!(
            "Setting up auto-generation job with cron: {}",
            config.cron_expression
        );

        let state_clone = state.clone();
        let job = Job::new_async(config.cron_expression.as_str(), move |_uuid, _l| {
            let state = state_clone.clone();
            Box::pin(async move {
                info!("Cron triggered: Starting automatic plan generation");
                let service = state.plan_service.lock().await;
                match service.generate_plan().await {
                    Ok(id) => {
                        info!("Successfully generated plan automatically: {}", id);

                        // We need the weekly_structure from the newly generated plan index
                        let mut plan_opt = None;
                        let mut structured_plan = None;
                        if let Ok(index) = service.list_plans() {
                            plan_opt = index.into_iter().find(|p| p.id == id);
                        }
                        if let Ok(detail) = service.get_plan_detail(&id) {
                            structured_plan = detail.structured_plan;
                        }

                        // Drop plan_service lock before getting the calendar lock
                        drop(service);

                        let config = state.config_repo.get().unwrap_or_default();
                        let calendar_service = state.calendar_service.lock().await;

                        // Find next free week (starting from the *next* Monday relative to today)
                        let mut next_monday = chrono::Local::now().date_naive().succ_opt().unwrap();
                        while next_monday.weekday() != chrono::Weekday::Mon {
                            next_monday = next_monday.succ_opt().unwrap();
                        }

                        let mut found_free_week = false;
                        for _ in 0..10 {
                            // check up to 10 weeks ahead
                            let end_of_week = next_monday + chrono::Duration::days(6);
                            if let Ok(entries) = calendar_service
                                .get_range(&next_monday.to_string(), &end_of_week.to_string())
                            {
                                if entries.is_empty() {
                                    found_free_week = true;
                                    break;
                                }
                            }
                            next_monday = next_monday + chrono::Duration::weeks(1);
                        }

                        if found_free_week {
                            if let Some(plan) = plan_opt {
                                if let Err(e) = calendar_service.assign_weekly_plan_to_date(
                                    &next_monday.to_string(),
                                    &id,
                                    structured_plan,
                                    plan.weekly_structure,
                                    config.default_meal_type,
                                ) {
                                    error!("Failed to assign generated plan to week: {}", e);
                                } else {
                                    info!(
                                        "Assigned generated plan {} to week starting {}",
                                        id, next_monday
                                    );
                                }
                            }
                        } else {
                            error!("Could not find a free week to assign the generated plan");
                        }

                        state.trigger_sync().await;
                    }
                    Err(e) => error!("Failed to automatically generate plan: {}", e),
                }
            })
        })
        .expect("Failed to create job");

        sched
            .add(job)
            .await
            .expect("Failed to add job to scheduler");
    } else {
        info!("Auto-generation is disabled in configuration");
    }

    sched.start().await.expect("Failed to start scheduler");
}
