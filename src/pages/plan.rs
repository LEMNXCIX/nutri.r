use crate::tauri_bridge::{get_index, calculate_nutrition, PlanIndex};
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Plan() -> impl IntoView {
    let plans_resource = LocalResource::new(move || async move {
        get_index().await.unwrap_or_default()
    });

    let (search_query, set_search_query) = signal(String::new());

    let filtered_plans = move || {
        let query = search_query.get().to_lowercase();
        plans_resource.get().map(|plans| {
            plans.into_iter()
                .filter(|p| p.id.to_lowercase().contains(&query) || p.fecha.contains(&query))
                .collect::<Vec<_>>()
        }).unwrap_or_default()
    };

    view! {
        <div class="bg-white min-h-screen font-sans text-neutral-950 pb-32 selection:bg-accent selection:text-neutral-950">
            <header class="flex items-center justify-between px-6 py-6 sticky top-0 bg-white/90 backdrop-blur-md z-40">
                <A href="/" attr:class="flex items-center">
                    <span class="material-symbols-outlined">arrow_back_ios</span>
                </A>
                <div class="text-[10px] font-bold tracking-[0.25em] uppercase text-neutral-400">Library / V2.4</div>
                <div class="w-64 h-8 flex items-center justify-end border-b border-neutral-100 focus-within:border-neutral-950 transition-colors">
                    <input 
                        type="text" 
                        placeholder="SEARCH" 
                        class="w-full bg-transparent border-none text-[10px] font-bold uppercase tracking-widest focus:ring-0 placeholder:text-neutral-300"
                        on:input=move |ev| set_search_query.set(event_target_value(&ev))
                        prop:value=search_query
                    />
                    <span class="material-symbols-outlined text-neutral-300">search</span>
                </div>
            </header>

            <main>
                <section class="px-6 pt-8 pb-12">
                    <h1 class="text-6xl font-extrabold uppercase leading-[0.85] tracking-tighter mb-4">
                        Saved<br/>Plans
                    </h1>
                    <div class="flex items-center gap-2 text-[10px] font-bold uppercase tracking-widest text-neutral-400">
                        <span>{move || filtered_plans().len()} " Active Archotypes"</span>
                        <span class="w-1 h-1 bg-neutral-300 rounded-full"></span>
                        <span>"Sorted by Recent"</span>
                    </div>
                </section>

                <section>
                    <div class="hairline-divider"></div>
                    <Suspense fallback=move || view! { <div class="px-6 py-8 animate-pulse text-neutral-300">"LOADING ARCHIVE..."</div> }>
                        {move || {
                            let plans = filtered_plans();
                            plans.into_iter().map(|plan| {
                                view! {
                                    <PlanListItem plan=plan />
                                }
                            }).collect_view()
                        }}
                    </Suspense>
                </section>

                <section class="px-6 py-12">
                    <div class="relative group aspect-[16/9] overflow-hidden bg-neutral-100 mb-8">
                        <img 
                            alt="Professional nutrition preparation" 
                            class="w-full h-full object-cover grayscale brightness-75 transition-all duration-700 group-hover:grayscale-0 group-hover:brightness-100" 
                            src="https://lh3.googleusercontent.com/aida-public/AB6AXuCuLIs4J3BB-Asz5cdNOorESMj1X3AVHQ_CyacDzU2zpMKJ4AmCCVsAedD5NzL-tBYxXv2eygd4hFNASqgdKD0gQnv78equgwci1mxJTvwA2XoV8I5GKSnShEzhTNk-Sfq7lK0QTcqEUsgGCWjJnyFLnU1YJVwoIJEK5Hfo3fFegV_Qf78T58vwbdtEQOflSZsT_ZYtWI8zXgmyhEojqt3UqYpvZwNrIO1VYttV3E3A3lfStG6x_jIYbQxMszgc2jS4Z_ticQKZ8Mha"
                        />
                        <div class="absolute inset-0 bg-neutral-950/20"></div>
                        <div class="absolute bottom-6 left-6 right-6">
                            <p class="text-[9px] font-bold uppercase tracking-[0.3em] text-white/70 mb-2">Featured Protocol</p>
                            <h3 class="text-white text-2xl font-bold uppercase tracking-tight leading-none">The Longevity<br/>Sprint 2024</h3>
                        </div>
                    </div>
                </section>
            </main>
        </div>
    }
}

#[component]
fn PlanListItem(plan: PlanIndex) -> impl IntoView {
    let nutrition = LocalResource::new({
        let id = plan.id.clone();
        move || {
            let id = id.clone();
            async move { calculate_nutrition(&id).await }
        }
    });

    let id_display = plan.id.clone();
    let id_for_link = plan.id.clone();

    view! {
        <A href=format!("/plan/{}", id_for_link) attr:class="block group">
            <div class="px-6 py-8 flex justify-between items-start group-hover:bg-neutral-50 transition-colors">
                <div class="space-y-4">
                    <div class="flex items-center gap-3">
                        <h2 class="text-2xl font-bold uppercase tracking-tight">
                            {id_display.chars().take(12).collect::<String>()}
                        </h2>
                        {if plan.is_favorite {
                            view! { <span class="bg-accent px-2 py-0.5 text-[9px] font-black uppercase tracking-widest">"Favorite"</span> }.into_any()
                        } else {
                            ().into_any()
                        }}
                    </div>
                    <div class="flex items-center gap-4 text-[10px] font-medium uppercase tracking-[0.15em] text-neutral-500">
                        <span>{plan.fecha}</span>
                        <span class="w-1 h-1 bg-neutral-200 rounded-full"></span>
                        <Suspense fallback=move || view! { <span>"..."</span> }>
                            {move || nutrition.get().and_then(|r| r.ok()).map(|n| {
                                view! {
                                    <span>{format!("{:.0} Kcal", n.total_calories)}</span>
                                }
                            })}
                        </Suspense>
                        {if !plan.proteinas.is_empty() {
                            view! {
                                <>
                                    <span class="w-1 h-1 bg-neutral-200 rounded-full"></span>
                                    <span>{plan.proteinas[0].clone()}</span>
                                </>
                            }.into_any()
                        } else {
                            ().into_any()
                        }}
                    </div>
                </div>
                <span class="material-symbols-outlined text-neutral-300">more_vert</span>
            </div>
            <div class="hairline-divider"></div>
        </A>
    }
}
