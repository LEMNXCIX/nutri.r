use leptos::prelude::*;

#[component]
pub fn Modal(
    #[prop(into)] on_close: Box<dyn Fn(leptos::ev::MouseEvent) + Send + Sync>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="fixed inset-0 bg-white/90 backdrop-blur-sm flex items-center justify-center z-[500] p-6">
            <div class="bg-white brutalist-border shadow-brutalist max-w-lg w-full relative">
                <button
                    class="absolute top-4 right-4 text-neutral-400 hover:text-neutral-950 transition-colors"
                    on:click=on_close
                >
                    <span class="material-symbols-outlined !text-[24px]">"close"</span>
                </button>
                <div class="p-8">
                    {children()}
                </div>
            </div>
        </div>
    }
}
