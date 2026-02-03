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
            <div class="fixed inset-x-0 bottom-0 pointer-events-none z-[1000] p-6 flex flex-col items-center md:items-end md:bottom-8 md:right-8 md:inset-x-auto">
                <div class="pointer-events-auto animate-in slide-in-from-bottom-8 fade-in duration-300">
                    <div class=format!("flex items-center gap-3 px-6 py-4 bg-white rounded-[1.5rem] shadow-2xl border border-gray-100 min-w-[280px] max-w-md {}",
                        if is_error { "border-red-100" } else { "border-gray-100" })
                    >
                        <div class=format!("p-2 rounded-xl {}",
                            if is_error { "bg-red-50 text-red-500" } else { "bg-black text-white" })
                        >
                            {if is_error {
                                view! { <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M6 18L18 6M6 6l12 12" /></svg> }.into_any()
                            } else {
                                view! { <span class="text-[10px] font-black uppercase">"n"</span> }.into_any()
                            }}
                        </div>

                        <div class="flex-1 pr-4">
                            <p class="text-xs font-black text-black uppercase tracking-tight leading-tight">
                                {move || message.get()}
                            </p>
                        </div>

                        <button
                            on:click=move |_| on_close.run(())
                            class="p-1 hover:bg-gray-100 rounded-lg text-gray-300 hover:text-black transition-all"
                        >
                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" /></svg>
                        </button>
                    </div>

                    // Visual spacer for Mobile BottomNav
                    <div class="h-20 md:hidden"></div>
                </div>
            </div>
        </Portal>
    }
}
