use leptos::prelude::*;

#[component]

pub fn Navbar() -> impl IntoView {
    view! {
        <h1 >"nutri.r"</h1>

        <nav class="navbar">
            <ul class="nav-links">
                <li><a href="/">inicio</a></li>
                <li><a href="/plans">planes</a></li>
                <li><a href="/config">config</a></li>
            </ul>
        </nav>
    }
}
