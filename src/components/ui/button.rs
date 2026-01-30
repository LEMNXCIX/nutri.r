use leptos::prelude::*;

#[component]
pub fn Button(
    on_click: Callback<leptos::ev::MouseEvent>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] disabled: Signal<bool>,
    children: Children,
) -> impl IntoView {
    view! {
        <button
            class=format!(
                "glass-light px-8 py-3.5 rounded-2xl font-black text-[10px] uppercase tracking-[0.2em] text-white hover:bg-white/10 hover:border-blue-500/30 transition-all active:scale-95 disabled:opacity-50 disabled:cursor-not-allowed shadow-lg {}",
                class
            )
            on:click=move |ev| on_click.run(ev)
            disabled=move || disabled.get()
        >
            {children()}
        </button>
    }
}
