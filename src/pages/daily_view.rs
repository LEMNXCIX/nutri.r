use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use chrono::{NaiveDate, Datelike};

#[component]
pub fn DailyView() -> impl IntoView {
    let params = use_params_map();
    let date_param = move || params.read().get("date").unwrap_or_else(|| "2026-02-25".to_string());
    
    let parsed_date = move || NaiveDate::parse_from_str(&date_param(), "%Y-%m-%d").unwrap_or_else(|_| NaiveDate::from_ymd_opt(2026, 2, 25).unwrap());

    let day_name = move || {
        match parsed_date().weekday() {
            chrono::Weekday::Mon => "Monday",
            chrono::Weekday::Tue => "Tuesday",
            chrono::Weekday::Wed => "Wednesday",
            chrono::Weekday::Thu => "Thursday",
            chrono::Weekday::Fri => "Friday",
            chrono::Weekday::Sat => "Saturday",
            chrono::Weekday::Sun => "Sunday",
        }
    };

    let month_name = move || {
        match parsed_date().month() {
            1 => "January",
            2 => "February",
            3 => "March",
            4 => "April",
            5 => "May",
            6 => "June",
            7 => "July",
            8 => "August",
            9 => "September",
            10 => "October",
            11 => "November",
            12 => "December",
            _ => "Unknown",
        }
    };

    view! {
        <div class="bg-white dark:bg-[#0A0A0A] text-black dark:text-white min-h-screen">
            <header class="sticky top-0 z-50 bg-white dark:bg-[#0A0A0A] border-b border-black dark:border-zinc-800 px-6 py-4 flex justify-between items-center">
                <button on:click=move |_| {
                    if let Some(window) = web_sys::window() {
                        if let Ok(history) = window.history() {
                            let _ = history.back();
                        }
                    }
                } class="flex items-center justify-center">
                    <span class="material-icons-outlined text-2xl">arrow_back</span>
                </button>
                <div class="text-[10px] tracking-[0.2em] font-black uppercase text-zinc-400">Planning Mastery</div>
                <button class="flex items-center justify-center">
                    <span class="material-icons-outlined text-2xl">more_vert</span>
                </button>
            </header>
            
            <main class="px-6 pb-32">
                <section class="py-8 border-b-2 border-black dark:border-white">
                    <div class="flex flex-col">
                        <span class="text-sm font-black uppercase tracking-widest text-zinc-500 dark:text-zinc-400">{move || day_name()}</span>
                        <h1 class="text-7xl font-black uppercase leading-none mt-1">{move || parsed_date().day()}</h1>
                        <div class="mt-4 flex justify-between items-end">
                            <span class="text-xs font-bold uppercase tracking-tighter text-zinc-400">{move || format!("{} {}", month_name(), parsed_date().year())}</span>
                            <div class="flex gap-2">
                                <div class="px-3 py-1 bg-black text-white dark:bg-white dark:text-black text-[10px] font-black uppercase">Optimal</div>
                                <div class="px-3 py-1 border border-black dark:border-white text-[10px] font-black uppercase">Active</div>
                            </div>
                        </div>
                    </div>
                </section>
                
                <section class="grid grid-cols-3 gap-0 border-b border-black dark:border-zinc-800">
                    <div class="py-4 border-r border-black dark:border-zinc-800">
                        <div class="text-[10px] uppercase font-bold text-zinc-400">Calories</div>
                        <div class="text-lg font-black">1,840<span class="text-[10px] text-zinc-400 ml-1">/ 2.2k</span></div>
                    </div>
                    <div class="py-4 px-4 border-r border-black dark:border-zinc-800">
                        <div class="text-[10px] uppercase font-bold text-zinc-400">Protein</div>
                        <div class="text-lg font-black">142g</div>
                    </div>
                    <div class="py-4 px-4">
                        <div class="text-[10px] uppercase font-bold text-zinc-400">Status</div>
                        <div class="flex items-center gap-1">
                            <div class="w-2 h-2 bg-[#00FF66]"></div>
                            <div class="text-lg font-black uppercase">92%</div>
                        </div>
                    </div>
                </section>
                
                <div class="mt-8 space-y-12">
                    <section>
                        <div class="flex justify-between items-baseline mb-4">
                            <h2 class="text-3xl font-black italic uppercase tracking-tighter">Breakfast</h2>
                            <span class="text-xs font-bold text-zinc-400">07:30 AM</span>
                        </div>
                        <div class="space-y-4">
                            <div class="flex items-center gap-4 group">
                                <div class="w-16 h-16 bg-zinc-100 dark:bg-zinc-900 overflow-hidden border border-zinc-200 dark:border-zinc-800 flex-shrink-0">
                                    <img alt="Greek Yogurt Bowl" class="grayscale w-full h-full object-cover" src="https://lh3.googleusercontent.com/aida-public/AB6AXuBNEHW93nqqC6FjrTPAfgFIQ-yp5mwk99YPL9wkpIL6Zs9_ovpGg7bxh-YKjicwtuNIUU4V5CZCVxgqwcaF1QpVN_jT4WrP8k6V6GnL3gVawVPCuo2_ZCClNP7V9cyB3yr1NFoiJI62ob_kI4ZFdiAsETQSm-fgx2FSuF4C2PoJ9sx17H5RtoRYBXn152zEyYYMIHMo-z9Zn4ysq-AMlas9UDGNnwdaoz7O3fXesSiCnbGGZN43miL7F2svxwc34Uu--L5puRWTdJH4"/>
                                </div>
                                <div class="flex-grow">
                                    <div class="flex justify-between items-start">
                                        <div>
                                            <h3 class="font-bold text-sm uppercase">Greek Yogurt Bowl</h3>
                                            <p class="text-xs text-zinc-500">Honey, Walnuts, Blueberries</p>
                                        </div>
                                        <span class="material-icons-outlined text-[#00FF66] text-xl">check_circle</span>
                                    </div>
                                    <div class="mt-2 text-[10px] font-bold text-zinc-400 uppercase tracking-wider flex gap-3">
                                        <span>340 kcal</span>
                                        <span>24g P</span>
                                        <span>12g F</span>
                                    </div>
                                </div>
                            </div>
                        </div>
                        <div class="mt-6 h-[1px] bg-zinc-200 dark:bg-zinc-800"></div>
                    </section>
                    
                    <section>
                        <div class="flex justify-between items-baseline mb-4">
                            <h2 class="text-3xl font-black italic uppercase tracking-tighter">Lunch</h2>
                            <span class="text-xs font-bold text-zinc-400">01:15 PM</span>
                        </div>
                        <div class="space-y-6">
                            <div class="flex items-center gap-4">
                                <div class="w-16 h-16 bg-zinc-100 dark:bg-zinc-900 overflow-hidden border border-zinc-200 dark:border-zinc-800 flex-shrink-0">
                                    <img alt="Grilled Salmon Salad" class="grayscale w-full h-full object-cover" src="https://lh3.googleusercontent.com/aida-public/AB6AXuASHwVHo5iU7NZSV8t9afO2mkvP4ngQR-Q5Zl8dmqykgBa1zGWtAkri8BP1Oc-oSrwGtBomDE7e80t5r8gp0oSJ4kmSAerU3rB3RfikPS2HCVwQApEs5v1Zjbh7TUypg1zpI1-OQBM_YOVpI6AQ2mI-3F2ykYKtptqlmoQjH2V6xmb_zb4VuUx0nZuICNuONzKCGSJnStUBfqksFI7zGS9i1CR2m6vq9ROeLzqbGawpFtd0Stm9QF86bOJuNyvQg8g3omy9aY_zsoB6"/>
                                </div>
                                <div class="flex-grow">
                                    <div class="flex justify-between items-start">
                                        <div>
                                            <h3 class="font-bold text-sm uppercase">Grilled Salmon Salad</h3>
                                            <p class="text-xs text-zinc-500">Spinach, Quinoa, Lemon Vinaigrette</p>
                                        </div>
                                        <span class="material-icons-outlined text-[#00FF66] text-xl">check_circle</span>
                                    </div>
                                    <div class="mt-2 text-[10px] font-bold text-zinc-400 uppercase tracking-wider flex gap-3">
                                        <span>520 kcal</span>
                                        <span>38g P</span>
                                        <span>22g F</span>
                                    </div>
                                </div>
                            </div>
                            <div class="flex items-center gap-4">
                                <div class="w-16 h-16 bg-zinc-100 dark:bg-zinc-900 overflow-hidden border border-zinc-200 dark:border-zinc-800 flex-shrink-0">
                                    <img alt="Sourdough Bread" class="grayscale w-full h-full object-cover opacity-50" src="https://lh3.googleusercontent.com/aida-public/AB6AXuC-fJcLRr4P0_lOJw_XtBombMSQ_iOxngMRNi8umaD1G1HxZpT5kJX6N3-i9R-3fgsV3Zyg2aDYltgyAR-19_C3JTdDaqHQDeQ8jVoMFAtlVALpbrTJJDiuHiZ2VH7vpSLnB43Cf6Rz8raFcD6YcTq1sx9gLcrKukyzsFgrqu2-bRR3VC-4JYT2-prHct7h6DBUC3KmIGvrF8LOmMUzXIpU5AP1rPMeecTuLc6mDahTCIMuA1dd_F7TB6U_yuEreNkmq8XM1u4Jh3v6"/>
                                </div>
                                <div class="flex-grow">
                                    <div class="flex justify-between items-start">
                                        <div>
                                            <h3 class="font-bold text-sm uppercase opacity-50">Sourdough Slice</h3>
                                            <p class="text-xs text-zinc-500 italic">Optional side</p>
                                        </div>
                                        <div class="w-5 h-5 border border-zinc-300 dark:border-zinc-700"></div>
                                    </div>
                                    <div class="mt-2 text-[10px] font-bold text-zinc-400 uppercase tracking-wider flex gap-3">
                                        <span>110 kcal</span>
                                        <span>4g P</span>
                                    </div>
                                </div>
                            </div>
                        </div>
                        <div class="mt-6 h-[1px] bg-zinc-200 dark:bg-zinc-800"></div>
                    </section>
                    
                    <section>
                        <div class="flex justify-between items-baseline mb-4">
                            <h2 class="text-3xl font-black italic uppercase tracking-tighter">Dinner</h2>
                            <span class="text-xs font-bold text-zinc-400">08:00 PM</span>
                        </div>
                        <div class="space-y-4">
                            <div class="flex items-center gap-4">
                                <div class="w-16 h-16 bg-zinc-100 dark:bg-zinc-900 overflow-hidden border-2 border-dashed border-zinc-300 dark:border-zinc-700 flex-shrink-0 flex items-center justify-center">
                                    <span class="material-icons-outlined text-zinc-400">restaurant</span>
                                </div>
                                <div class="flex-grow">
                                    <div class="flex justify-between items-start">
                                        <div>
                                            <h3 class="font-bold text-sm uppercase">Black Garlic Chicken</h3>
                                            <p class="text-xs text-zinc-500">Broccoli, Roasted Sweet Potato</p>
                                        </div>
                                        <div class="w-5 h-5 border border-black dark:border-white"></div>
                                    </div>
                                    <div class="mt-2 text-[10px] font-bold text-zinc-400 uppercase tracking-wider flex gap-3">
                                        <span>610 kcal</span>
                                        <span>45g P</span>
                                        <span>14g F</span>
                                    </div>
                                </div>
                            </div>
                        </div>
                        <div class="mt-6 h-[1px] bg-zinc-200 dark:bg-zinc-800"></div>
                    </section>
                </div>
                
                <button class="mt-12 w-full bg-black dark:bg-white text-white dark:text-black py-5 font-black uppercase tracking-[0.3em] text-sm">
                    Add Supplement
                </button>
            </main>
        </div>
    }
}
