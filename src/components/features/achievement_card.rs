use crate::tauri_bridge::Achievement;
use leptos::prelude::*;

#[component]
pub fn AchievementCard(achievement: Achievement) -> impl IntoView {
    let is_unlocked = achievement.unlocked_at.is_some();

    view! {
        <div class=format!(
            "p-6 rounded-[2.5rem] transition-all duration-500 flex flex-col items-center text-center gap-4 relative overflow-hidden group {}",
            if is_unlocked {
                "glass border-gray-200 shadow-2xl shadow-black/5"
            } else {
                "glass border-white/5 opacity-50 grayscale hover:opacity-100 transition-all duration-700"
            }
        )>
            {if is_unlocked {
                view! {
                    <div class="absolute -top-12 -right-12 w-32 h-32 bg-gray-100 blur-3xl group-hover:bg-gray-200 transition-all duration-700"></div>
                }.into_any()
            } else {
                ().into_any()
            }}

            <div class=format!("text-5xl mb-2 transition-transform duration-500 group-hover:scale-110 {}", if is_unlocked { "drop-shadow-lg" } else { "" })>
                {achievement.icon}
            </div>

            <div class="space-y-2 relative z-10">
                <h3 class=format!("font-black text-lg tracking-tighter uppercase italic {}", if is_unlocked { "text-white" } else { "text-gray-400" })>
                    {achievement.title}
                </h3>
                <p class="text-[10px] text-gray-500 font-black uppercase tracking-widest leading-relaxed">
                    {achievement.description}
                </p>
            </div>

            <div class="mt-4 w-full">
                {if is_unlocked {
                    view! {
                        <div class="px-4 py-1.5 rounded-full bg-black">
                            <span class="text-[9px] text-white font-black uppercase tracking-[0.2em]">
                                "¡Desbloqueado!"
                            </span>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div class="px-4 py-1.5 rounded-full bg-white/5 border border-white/5">
                            <span class="text-[9px] text-gray-600 font-black uppercase tracking-[0.2em]">
                                "Bloqueado"
                            </span>
                        </div>
                    }.into_any()
                }}
            </div>
        </div>
    }
}
