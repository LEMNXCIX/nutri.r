use crate::tauri_bridge::{generate_week, get_index, PlanIndex};
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::components::A;

#[component]
pub fn Home() -> impl IntoView {
    // Estado
    let (loading, set_loading) = signal(false);
    let (plans, set_plans) = signal::<Vec<PlanIndex>>(vec![]); // Aquí cargarías el index
    spawn_local(async move {
        let indexs = get_index().await;
        match indexs {
            Ok(index) => {
                set_plans.set(index);
            }
            Err(e) => {
                leptos::logging::error!("Error: {}", e);
            }
        }
    });
    // Acción
    // Acción
    let on_generate = move |_| {
        set_loading.set(true);
        spawn_local(async move {
            let result = generate_week().await;
            match result {
                Ok(_) => {
                    leptos::logging::log!("Plan generado!");
                    let indexs = get_index().await;
                    if let Ok(index) = indexs {
                        set_plans.set(index);
                    }
                }
                Err(e) => {
                    leptos::logging::error!("Error: {}", e);
                }
            };
            set_loading.set(false);
        });
    };
    view! {
        <button
            class="bg-green-600 hover:bg-green-500 px-4 py-2 rounded transition disabled:opacity-50"
            on:click=on_generate
            disabled=loading
        >
            {move || if loading.get() { "Generando..." } else { "Crear Nueva Semana" }}
        </button>

        // Aquí iría el componente <PlanList plans=plans />
        <hr style="border:none; margin:20px;" />
        <span> Planes: </span>
        <ul>
            {move || plans.get().into_iter().map(|plan| view! {
                <A href={format!("/plan/{}", plan.id)}><li>{format!("{} ({})", plan.fecha, plan.id)}</li></A>
            }).collect_view()}
        </ul>
    }
}
