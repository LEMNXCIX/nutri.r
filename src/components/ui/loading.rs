use leptos::prelude::*;

#[component]
pub fn Loading(#[prop(optional, into)] size: String) -> impl IntoView {
    let size_class = if size.is_empty() { "h-12 w-12" } else { &size };
    view! {
        <div class="flex flex-col items-center justify-center p-8 gap-4 animate-in fade-in duration-700">
            <div class=format!("relative {}", size_class)>
                <div class="absolute inset-0 rounded-full border-4 border-blue-500/10"></div>
                <div class="absolute inset-0 rounded-full border-4 border-t-blue-500 animate-spin shadow-[0_0_15px_rgba(59,130,246,0.5)]"></div>
            </div>
            {if size.is_empty() {
                view! {
                    <span class="text-[10px] font-black uppercase tracking-[0.3em] text-blue-400/60 animate-pulse italic">
                        "Procesando"
                    </span>
                }.into_any()
            } else {
                view! { <div/> }.into_any()
            }}
        </div>
    }
}
