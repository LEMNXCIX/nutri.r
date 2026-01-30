#![allow(dead_code)]
use leptos::prelude::*;

#[component]
pub fn Spacer(
    /// El tamaño del espacio en píxeles. Por defecto es 20.
    #[prop(default = 20)]
    size: i32,
) -> impl IntoView {
    let _ = size; // Explicitly mark as used
    view! {
        <div style=move || format!("height: {}px; width: 100%;", size) />
    }
}
