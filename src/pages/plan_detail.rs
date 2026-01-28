use crate::tauri_bridge::get_plan_content;
use leptos::logging::log;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

#[component]
pub fn PlanDetail() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|params| params.get("id").unwrap_or_default());
    let plan_resource = LocalResource::new(move || {
        let id = id();
        async move {
            let result = get_plan_content(&id).await;
            match result {
                Ok(content) => {
                    let html = markdown::to_html(&content);
                    Some(html)
                }
                Err(err) => {
                    log!("Error fetching plan content: {}", err);
                    None
                }
            }
        }
    });

    log!("PlanDetail id: {}", id());

    view! {
        <div >
            <h2 >Plan Detail {id()}</h2>
            <span>Content:</span>
            <article
                class="prose prose-invert"
                prop:innerHTML=move || plan_resource.get().flatten().unwrap_or_else(|| "Loading...".to_string())
            />
        </div>
    }
}
