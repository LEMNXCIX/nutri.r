use leptos::portal::Portal;
use leptos::prelude::*;

#[component]
pub fn Toast(
    #[prop(into)] message: Signal<String>,
    #[prop(into)] on_close: Callback<()>,
    #[prop(optional)] is_error: bool,
) -> impl IntoView {
    view! {
        <Portal>
            <div class="fixed bottom-32 left-6 right-6 z-[1000] md:bottom-32 md:right-8 md:left-auto md:w-96 pointer-events-none">
                <div class="pointer-events-auto brutalist-border bg-white p-4 shadow-brutalist animate-in slide-in-from-bottom-4 flex items-center gap-4">
                    <div class=format!("w-10 h-10 flex items-center justify-center shrink-0 {}",
                        if is_error { "bg-red-500 text-white" } else { "bg-accent text-neutral-950" })
                    >
                        <span class="material-symbols-outlined !text-xl">
                            {if is_error { "error" } else { "check_circle" }}
                        </span>
                    </div>

                    <div class="flex-1 min-w-0">
                        <p class="text-[10px] font-black uppercase tracking-widest text-neutral-400 mb-0.5">
                            {if is_error { "System Error" } else { "System Notification" }}
                        </p>
                        <p class="text-xs font-bold text-neutral-950 uppercase tracking-tight truncate">
                            {move || message.get()}
                        </p>
                    </div>

                    <button
                        on:click=move |_| on_close.run(())
                        class="p-2 hover:bg-neutral-50 text-neutral-300 hover:text-neutral-950 transition-colors"
                    >
                        <span class="material-symbols-outlined !text-lg">"close"</span>
                    </button>
                </div>
            </div>
        </Portal>
    }
}
