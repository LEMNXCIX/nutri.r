use crate::components::features::AchievementCard;
use crate::components::ui::Loading;
use crate::tauri_bridge::{get_achievements, Achievement};
use leptos::prelude::*;
use leptos::task::spawn_local;

#[component]
pub fn Achievements() -> impl IntoView {
    let (achievements, set_achievements) = signal(Vec::<Achievement>::new());
    let (loading, set_loading) = signal(true);

    spawn_local(async move {
        match get_achievements().await {
            Ok(data) => {
                set_achievements.set(data);
                set_loading.set(false);
            }
            Err(e) => {
                log::error!("Error fetching achievements: {}", e);
                set_loading.set(false);
            }
        }
    });

    view! {
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-10 animate-in fade-in duration-700">
            <header class="mb-12 text-center">
                <span class="inline-block px-4 py-1.5 rounded-full bg-yellow-500/10 text-yellow-400 text-[10px] font-black uppercase tracking-[0.2em] mb-4">
                    "Rewards & Milestones"
                </span>
                <h2 class="text-5xl font-black text-white tracking-tighter mb-4 uppercase italic premium-gradient-text">
                    "Logros y Trofeos"
                </h2>
                <div class="h-1.5 w-24 bg-yellow-500 mx-auto rounded-full mb-6"></div>
                <p class="text-gray-400 font-medium max-w-2xl mx-auto uppercase tracking-widest text-[10px] leading-relaxed">
                    "Completa acciones en la aplicación para desbloquear recompensas exclusivas."
                </p>
            </header>

            {move || if loading.get() {
                view! { <div class="flex justify-center py-20"><Loading /></div> }.into_any()
            } else {
                let list = achievements.get();
                if list.is_empty() {
                    view! {
                        <div class="glass rounded-[3rem] p-20 text-center border-white/5 opacity-50">
                            <p class="text-gray-500 font-black uppercase tracking-widest text-sm">"No hay logros disponibles todavía"</p>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                            {list.into_iter().map(|a| view! { <AchievementCard achievement=a /> }).collect::<Vec<_>>()}
                        </div>
                    }.into_any()
                }
            }}
        </div>
    }
}
