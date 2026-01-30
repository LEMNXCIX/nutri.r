use leptos::prelude::*;

#[component]
pub fn Modal(
    #[prop(into)] on_close: Box<dyn Fn(leptos::ev::MouseEvent) + Send + Sync>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="fixed inset-0 bg-black bg-opacity-75 flex items-center justify-center z-50 p-4">
            <div class="bg-gray-800 rounded-lg shadow-xl max-w-lg w-full relative border border-gray-700">
                <button
                    class="absolute top-3 right-3 text-gray-400 hover:text-white transition-colors"
                    on:click=on_close
                >
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                    </svg>
                </button>
                <div class="p-6">
                    {children()}
                </div>
            </div>
        </div>
    }
}
