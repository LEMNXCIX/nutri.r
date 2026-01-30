use crate::tauri_bridge::{create_tag, get_all_tags, Tag};
use leptos::prelude::*;
use leptos::task::spawn_local;

#[component]
pub fn TagSelector(
    on_select: Callback<Tag>,
    existing_tag_ids: Signal<Vec<String>>,
) -> impl IntoView {
    let (available_tags, set_available_tags) = signal::<Vec<Tag>>(vec![]);
    let (show_dropdown, set_show_dropdown) = signal(false);
    let (new_tag_name, set_new_tag_name) = signal(String::new());

    let fetch_tags = move || {
        spawn_local(async move {
            if let Ok(tags) = get_all_tags().await {
                set_available_tags.set(tags);
            }
        });
    };

    // Initial fetch
    fetch_tags();

    let filtered_tags = move || {
        let current_ids = existing_tag_ids.get();
        available_tags
            .get()
            .into_iter()
            .filter(|t| !current_ids.contains(&t.id))
            .collect::<Vec<_>>()
    };

    let on_create = move |_| {
        let name = new_tag_name.get();
        if name.is_empty() {
            return;
        }

        spawn_local(async move {
            let colors = vec![
                "#60a5fa", "#34d399", "#f87171", "#fbbf24", "#a78bfa", "#f472b6",
            ];
            let hash = name.chars().map(|c| c as usize).sum::<usize>();
            let color = colors[hash % colors.len()].to_string();

            if let Ok(tag) = create_tag(name, color).await {
                on_select.run(tag);
                set_new_tag_name.set(String::new());
                set_show_dropdown.set(false);
                fetch_tags();
            }
        });
    };

    view! {
        <div class="relative inline-block">
            <button
                class="flex items-center gap-2 px-4 py-2 rounded-xl text-[10px] font-black uppercase tracking-widest bg-white/5 border border-white/10 text-gray-400 hover:text-white hover:border-blue-500/50 hover:bg-blue-500/5 transition-all active:scale-95"
                on:click=move |_| set_show_dropdown.update(|v| *v = !*v)
            >
                <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
                </svg>
                "ETIQUETA"
            </button>

            {move || if show_dropdown.get() {
                view! {
                    <div class="absolute top-full left-0 mt-3 w-64 glass rounded-[2rem] border-white/10 shadow-3xl z-50 p-4 animate-in fade-in slide-in-from-top-2 duration-300 overflow-hidden">
                        <div class="absolute inset-0 bg-blue-500/5 blur-3xl pointer-events-none"></div>
                        <div class="relative space-y-4">
                            <div class="max-h-48 overflow-y-auto space-y-1 custom-scrollbar pr-1">
                                {move || {
                                    let tags = filtered_tags();
                                    if tags.is_empty() {
                                        view! {
                                            <p class="text-[9px] font-black uppercase tracking-widest text-gray-600 text-center py-4">"No hay más etiquetas"</p>
                                        }.into_any()
                                    } else {
                                        tags.into_iter().map(|tag| {
                                            let t = tag.clone();
                                            view! {
                                                <button
                                                    class="w-full text-left px-3 py-2.5 rounded-xl hover:bg-white/10 transition-all flex items-center gap-3 group active:scale-95"
                                                    on:click=move |_| {
                                                        on_select.run(t.clone());
                                                        set_show_dropdown.set(false);
                                                    }
                                                >
                                                    <div class="w-1.5 h-1.5 rounded-full shadow-[0_0_8px_rgba(255,255,255,0.2)]" style=format!("background-color: {}", tag.color)></div>
                                                    <span class="text-[10px] font-black text-gray-400 group-hover:text-white uppercase tracking-widest">{tag.name}</span>
                                                </button>
                                            }
                                        }).collect_view().into_any()
                                    }
                                }}
                            </div>

                            <div class="pt-4 border-t border-white/5">
                                <div class="flex flex-col gap-3">
                                    {view! {
                                        <input
                                            type="text"
                                            placeholder="Nueva etiqueta..."
                                            class="glass-light w-full border border-white/5 rounded-xl px-4 py-2.5 text-[10px] font-black uppercase tracking-widest text-white outline-none focus:ring-1 focus:ring-blue-500/50"
                                            prop:value=move || new_tag_name.get()
                                            on:input=move |ev| set_new_tag_name.set(event_target_value(&ev))
                                            on:keydown=move |ev| if ev.key() == "Enter" { on_create(()); }
                                        />
                                    }.into_any()}
                                    {view! {
                                        <button
                                            class="w-full bg-blue-600 hover:bg-blue-500 text-white text-[10px] font-black uppercase tracking-[0.2em] py-3 rounded-xl shadow-lg shadow-blue-500/20 transition-all active:scale-95"
                                            on:click=move |_| on_create(())
                                        >
                                            "CREAR Y AÑADIR"
                                        </button>
                                    }.into_any()}
                                </div>
                            </div>
                        </div>
                    </div>
                }.into_any()
            } else {
                view! {}.into_any()
            }}
        </div>
    }
}
