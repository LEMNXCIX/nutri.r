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
                "btn disabled:opacity-50 disabled:cursor-not-allowed {}",
                class
            )
            on:click=move |ev| on_click.run(ev)
            disabled=move || disabled.get()
        >
            {children()}
        </button>
    }
}
