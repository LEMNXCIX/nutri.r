use leptos::prelude::*;

#[component]
pub fn Input(
    #[prop(optional, into)] id: String,
    #[prop(optional, into)] type_: String,
    #[prop(optional, into)] placeholder: String,
    #[prop(into)] value: Signal<String>,
    on_input: Callback<String>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] disabled: Signal<bool>,
) -> impl IntoView {
    let type_ = if type_.is_empty() {
        "text".to_string()
    } else {
        type_
    };

    view! {
        <input
            id=if id.is_empty() { None } else { Some(id) }
            type=type_
            class=format!(
                "input w-full {}",
                class
            )
            placeholder=placeholder
            prop:value=value
            on:input=move |ev| on_input.run(event_target_value(&ev))
            disabled=move || disabled.get()
        />
    }
}
