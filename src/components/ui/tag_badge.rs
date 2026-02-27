use leptos::prelude::*;

#[component]
pub fn TagBadge(
    name: String,
    _color: String,
    #[prop(optional)] on_remove: Option<Callback<()>>,
) -> impl IntoView {
    view! {
        <span
            class="inline-flex items-center gap-2 px-3 py-1.5 border border-neutral-950 text-[10px] font-black uppercase tracking-widest bg-white text-neutral-950 group h-8"
        >
            <div class="w-2 h-2 bg-accent"></div>
            {name}
            {match on_remove {
                Some(cb) => view! {
                    <button
                        class="ml-2 hover:bg-neutral-950 hover:text-white transition-colors p-1"
                        on:click=move |e| {
                            e.stop_propagation();
                            cb.run(());
                        }
                    >
                        <span class="material-symbols-outlined !text-[14px]">"close"</span>
                    </button>
                }.into_any(),
                None => view! {}.into_any()
            }}
        </span>
    }
}
