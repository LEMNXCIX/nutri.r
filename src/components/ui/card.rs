use leptos::prelude::*;

#[component]
pub fn Card(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! {
        <div class=format!("card p-6 {}", class)>
            {children()}
        </div>
    }
}
