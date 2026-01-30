use leptos::prelude::*;

#[component]
pub fn Card(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! {
        <div class=format!("glass rounded-[2rem] p-8 shadow-2xl transition-all {}", class)>
            {children()}
        </div>
    }
}
