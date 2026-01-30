use leptos::prelude::*;

#[component]
pub fn StarRating(
    #[prop(into)] rating: Signal<Option<u8>>,
    #[prop(into)] on_rate: Callback<u8>,
    #[prop(optional)] max: Option<u8>,
) -> impl IntoView {
    let max_val = max.unwrap_or(5);

    view! {
        <div class="flex items-center gap-1.5 p-2 rounded-2xl bg-white/5 border border-white/5 w-fit">
            {
                (1..=max_val).map(|i| {
                    let is_active = move || rating.get().unwrap_or(0) >= i;
                    let i_val = i;
                    view! {
                        <button
                            type="button"
                            class="focus:outline-none transition-all duration-300 transform hover:scale-125 active:scale-90"
                            on:click=move |_| on_rate.run(i_val)
                        >
                            <svg
                                class=move || if is_active() { "w-6 h-6 text-yellow-400 fill-current drop-shadow-[0_0_8px_rgba(250,204,21,0.5)]" } else { "w-6 h-6 text-gray-700 fill-none hover:text-gray-500" }
                                xmlns="http://www.w3.org/2000/svg"
                                viewBox="0 0 24 24"
                                stroke="currentColor"
                                stroke-width="2"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                            >
                                <polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2" />
                            </svg>
                        </button>
                    }
                }).collect::<Vec<_>>()
            }
        </div>
    }
}
