use leptos::prelude::*;

#[component]
pub fn Input(
    #[prop(optional, into)] id: String,
    #[prop(optional, into)] type_: String,
    #[prop(optional, into)] placeholder: String,
    #[prop(into)] value: Signal<String>,
    on_input: Callback<String>,
    #[prop(optional, into)] class: String,
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
                "glass-light px-5 py-3.5 rounded-2xl border-white/5 text-white placeholder-gray-500 outline-none focus:ring-2 focus:ring-blue-500/30 focus:border-blue-500/20 transition-all font-medium text-sm w-full {}",
                class
            )
            placeholder=placeholder
            prop:value=value
            on:input=move |ev| on_input.run(event_target_value(&ev))
        />
    }
}
