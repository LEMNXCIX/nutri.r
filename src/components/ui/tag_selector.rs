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
                class="flex items-center gap-2 px-3 py-2 brutalist-border dark:border-neutral-700 text-[10px] font-black uppercase tracking-wider bg-white dark:bg-neutral-900 text-neutral-400 dark:text-neutral-500 hover:text-neutral-950 dark:hover:text-white transition-all active:scale-95"
                on:click=move |_| set_show_dropdown.update(|v| *v = !*v)
            >
                <span class="material-symbols-outlined !text-[14px]">"add"</span>
                "Add Tag"
            </button>

            {move || if show_dropdown.get() {
                view! {
                    <div class="absolute top-full left-0 mt-2 w-64 bg-white dark:bg-neutral-900 brutalist-border dark:border-neutral-700 shadow-brutalist z-50 p-3 overflow-hidden">
                        <div class="relative space-y-3">
                            <div class="max-h-48 overflow-y-auto space-y-1 pr-1">
                                {move || {
                                    let tags = filtered_tags();
                                    if tags.is_empty() {
                                        view! {
                                            <p class="text-[9px] font-bold uppercase tracking-widest text-neutral-400 dark:text-neutral-500 text-center py-2">"No tags available"</p>
                                        }.into_any()
                                    } else {
                                        tags.into_iter().map(|tag| {
                                            let t = tag.clone();
                                            view! {
                                                <button
                                                    class="w-full text-left px-3 py-2 hover:bg-neutral-50 dark:hover:bg-neutral-800 transition-all flex items-center gap-2 group active:scale-95"
                                                    on:click=move |_| {
                                                        on_select.run(t.clone());
                                                        set_show_dropdown.set(false);
                                                    }
                                                >
                                                    <div class="w-2 h-2 bg-accent"></div>
                                                    <span class="text-[10px] font-bold text-neutral-400 dark:text-neutral-500 group-hover:text-neutral-950 dark:group-hover:text-white uppercase tracking-wider">{tag.name}</span>
                                                </button>
                                            }
                                        }).collect_view().into_any()
                                    }
                                }}
                            </div>

                            <div class="pt-3 border-t border-neutral-100 dark:border-neutral-800">
                                <div class="flex flex-col gap-2">
                                    <input
                                        type="text"
                                        placeholder="New tag..."
                                        class="w-full bg-neutral-50 dark:bg-neutral-800 border border-neutral-200 dark:border-neutral-700 px-3 py-2 text-[10px] font-bold uppercase tracking-wider text-neutral-950 dark:text-white outline-none focus:ring-1 focus:ring-accent"
                                        prop:value=move || new_tag_name.get()
                                        on:input=move |ev| set_new_tag_name.set(event_target_value(&ev))
                                        on:keydown=move |ev| if ev.key() == "Enter" { on_create(()); }
                                    />
                                    <button
                                        class="w-full bg-neutral-950 hover:bg-accent hover:text-neutral-950 text-white text-[10px] font-black uppercase tracking-widest py-3 transition-all active:translate-x-0.5 active:translate-y-0.5"
                                        on:click=move |_| on_create(())
                                    >
                                        "Create & Add"
                                    </button>
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
