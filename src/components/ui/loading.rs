use leptos::prelude::*;

#[component]
pub fn Loading(#[prop(optional, into)] size: String) -> impl IntoView {
    let size_class = if size.is_empty() { "h-8 w-8" } else { &size };
    view! {
        <div class="flex flex-col items-center justify-center p-8 gap-4 animate-fade-in">
            <div class=format!("relative {}", size_class)>
                <div class="absolute inset-0 rounded-full border-2 border-gray-100"></div>
                <div class="absolute inset-0 rounded-full border-2 border-t-black animate-spin"></div>
            </div>
            {if size.is_empty() {
                view! {
                    <span class="text-[10px] font-medium uppercase tracking-widest text-gray-400">
                        "Cargando"
                    </span>
                }.into_any()
            } else {
                view! { <div/> }.into_any()
            }}
        </div>
    }
}
