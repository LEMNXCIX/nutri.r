pub fn initial_theme() -> String {
    stored_theme().unwrap_or_else(system_theme)
}

pub fn resolved_theme(preferred_theme: Option<&str>) -> String {
    active_theme()
        .or_else(stored_theme)
        .or_else(|| preferred_theme.and_then(normalize_theme).map(str::to_string))
        .unwrap_or_else(system_theme)
}

pub fn set_theme(theme: &str) -> String {
    let normalized = normalize_theme(theme).unwrap_or("light");
    apply_theme(normalized);
    persist_theme(normalized);
    normalized.to_string()
}

pub fn toggle_theme() -> String {
    let next_theme = if active_theme().as_deref() == Some("dark") {
        "light"
    } else {
        "dark"
    };

    set_theme(next_theme)
}

fn active_theme() -> Option<String> {
    let html = web_sys::window()?.document()?.document_element()?;
    Some(
        if html.class_list().contains("dark") {
            "dark"
        } else {
            "light"
        }
        .to_string(),
    )
}

fn stored_theme() -> Option<String> {
    let storage = web_sys::window()?.local_storage().ok().flatten()?;
    let theme = storage.get_item("theme").ok().flatten()?;
    normalize_theme(&theme).map(str::to_string)
}

fn system_theme() -> String {
    let prefers_dark = web_sys::window()
        .and_then(|window| window.match_media("(prefers-color-scheme: dark)").ok().flatten())
        .map(|query| query.matches())
        .unwrap_or(false);

    if prefers_dark {
        "dark".to_string()
    } else {
        "light".to_string()
    }
}

pub fn apply_theme(theme: &str) {
    if let Some(html) = web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| document.document_element())
    {
        if theme == "dark" {
            let _ = html.class_list().add_1("dark");
        } else {
            let _ = html.class_list().remove_1("dark");
        }
    }
}

fn persist_theme(theme: &str) {
    if let Some(storage) = web_sys::window().and_then(|window| window.local_storage().ok().flatten())
    {
        let _ = storage.set_item("theme", theme);
    }
}

fn normalize_theme(theme: &str) -> Option<&'static str> {
    match theme {
        "dark" => Some("dark"),
        "light" => Some("light"),
        _ => None,
    }
}
