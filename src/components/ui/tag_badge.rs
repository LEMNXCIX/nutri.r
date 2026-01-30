use leptos::prelude::*;

#[component]
pub fn TagBadge(
    name: String,
    color: String,
    #[prop(optional)] on_remove: Option<Callback<()>>,
) -> impl IntoView {
    view! {
        <span
            class="inline-flex items-center gap-2 px-3 py-1.5 rounded-xl text-[10px] font-black uppercase tracking-widest border transition-all glass-light"
            style=format!("border-color: {}40; color: {};", color, color)
        >
            <div class="w-1.5 h-1.5 rounded-full" style=format!("background-color: {}", color)></div>
            {name}
            {match on_remove {
                Some(cb) => view! {
                    <button
                        class="p-0.5 rounded-md hover:bg-white/10 text-gray-400 hover:text-white transition-all active:scale-90"
                        on:click=move |e| {
                            e.stop_propagation();
                            cb.run(());
                        }
                    >
                        <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                        </svg>
                    </button>
                }.into_any(),
                None => view! {}.into_any()
            }}
        </span>
    }
}
