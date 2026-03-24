use leptos::prelude::*;

#[component]
pub fn Modal(
    on_close: Callback<leptos::ev::MouseEvent>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="fixed inset-0 bg-white/90 dark:bg-background-dark/90 backdrop-blur-sm flex items-center justify-center z-[500] p-6">
            <div class="bg-white dark:bg-neutral-900 brutalist-border dark:border-neutral-700 shadow-brutalist max-w-lg w-full relative">
                <button
                    class="absolute top-4 right-4 text-neutral-400 dark:text-neutral-500 hover:text-neutral-950 dark:hover:text-white transition-colors"
                    on:click=move |ev| on_close.run(ev)
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
