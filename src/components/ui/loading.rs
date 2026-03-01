use leptos::prelude::*;

#[component]
pub fn Loading(#[prop(optional, into)] size: String) -> impl IntoView {
    let size_class = if size.is_empty() { "h-12 w-12" } else { &size };

    view! {
        <div class="flex flex-col items-center justify-center p-12 gap-6 animate-in fade-in duration-500">
            <div class=format!("relative {} border border-neutral-100 flex items-center justify-center group", size_class)>
                // Outer rotating frame
                <div class="absolute inset-0 border-2 border-t-accent border-r-transparent border-b-transparent border-l-transparent animate-spin"></div>

                // Inner solid square pulsing
                <div class="w-1/2 h-1/2 bg-neutral-950 animate-pulse"></div>

                // Corner accents
                <div class="absolute -top-1 -left-1 w-2 h-2 bg-accent"></div>
                <div class="absolute -bottom-1 -right-1 w-2 h-2 bg-neutral-950"></div>
            </div>

            <div class="flex flex-col items-center gap-1">
                <span class="text-[10px] font-black uppercase tracking-[0.4em] text-neutral-950">
                    "Analizando"
                </span>
                <span class="text-[8px] font-bold uppercase tracking-[0.2em] text-neutral-400">
                    "Verificación de integridad de datos en curso"
                </span>
            </div>
        </div>
    }
}
