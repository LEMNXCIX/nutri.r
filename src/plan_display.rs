use crate::tauri_bridge::PlanIndex;
use chrono::{DateTime, Datelike, Local, NaiveDate, NaiveDateTime, Timelike, Utc};

pub fn plan_display_name(plan: &PlanIndex) -> String {
    plan.display_name
        .as_ref()
        .map(|name| name.trim())
        .filter(|name| !name.is_empty())
        .map(ToString::to_string)
        .unwrap_or_else(|| format!("Plan {}", plan.id.chars().take(6).collect::<String>()))
}

pub fn resolve_plan_header_title(
    display_name: Option<&str>,
    structured_title: Option<&str>,
    id: &str,
) -> String {
    if let Some(name) = display_name
        .map(str::trim)
        .filter(|name| !name.is_empty())
    {
        return name.to_string();
    }

    if let Some(title) = structured_title
        .map(str::trim)
        .filter(|title| !title.is_empty() && !is_generic_title(title))
    {
        return title.to_string();
    }

    format!("Plan {}", id.chars().take(6).collect::<String>())
}

pub fn format_plan_created_at(plan: &PlanIndex) -> String {
    if let Some(local_dt) = plan
        .created_at
        .as_deref()
        .and_then(parse_rfc3339_utc_to_local)
    {
        let today = Local::now().date_naive();
        let date_label = if local_dt.date_naive() == today {
            "Hoy".to_string()
        } else {
            format_day_month(local_dt.date_naive())
        };

        return format!("{}, {}", date_label, format_time(local_dt));
    }

    format_legacy_date(plan)
}

pub fn plan_sort_key(plan: &PlanIndex) -> i64 {
    if let Some(created_at) = plan
        .created_at
        .as_deref()
        .and_then(|value| DateTime::parse_from_rfc3339(value).ok())
        .map(|value| value.with_timezone(&Utc))
    {
        return created_at.timestamp_millis();
    }

    if let Some(legacy) = parse_legacy_naive_datetime(plan) {
        return legacy.and_utc().timestamp_millis();
    }

    0
}

pub fn plan_search_blob(plan: &PlanIndex) -> String {
    let display_name = plan_display_name(plan);
    let display_date = format_plan_created_at(plan);
    format!(
        "{} {} {} {}",
        plan.id,
        display_name,
        display_date,
        plan.proteinas.join(" ")
    )
}

fn parse_rfc3339_utc_to_local(value: &str) -> Option<DateTime<Local>> {
    DateTime::parse_from_rfc3339(value)
        .ok()
        .map(|value| value.with_timezone(&Local))
}

fn parse_legacy_naive_datetime(plan: &PlanIndex) -> Option<NaiveDateTime> {
    let fecha = plan.fecha.trim();
    if !fecha.is_empty() {
        if let Ok(datetime) = NaiveDateTime::parse_from_str(fecha, "%Y-%m-%d %H:%M:%S") {
            return Some(datetime);
        }

        if let Ok(date) = NaiveDate::parse_from_str(fecha, "%Y-%m-%d") {
            return date.and_hms_opt(0, 0, 0);
        }
    }

    let digits = plan
        .id
        .chars()
        .filter(|char| char.is_ascii_digit())
        .collect::<String>();
    if digits.len() >= 14 {
        if let Ok(datetime) = NaiveDateTime::parse_from_str(&digits[..14], "%Y%m%d%H%M%S") {
            return Some(datetime);
        }
    }

    if digits.len() >= 12 {
        if let Ok(datetime) = NaiveDateTime::parse_from_str(&digits[..12], "%Y%m%d%H%M") {
            return Some(datetime);
        }
    }

    None
}

fn format_legacy_date(plan: &PlanIndex) -> String {
    if let Some(datetime) = parse_legacy_naive_datetime(plan) {
        return format_day_month_year(datetime.date());
    }

    let fecha = plan.fecha.trim();
    if !fecha.is_empty() {
        fecha.to_string()
    } else {
        format!("Plan {}", plan.id.chars().take(6).collect::<String>())
    }
}

fn format_day_month(date: NaiveDate) -> String {
    format!("{} {}", date.day(), month_label(date.month()))
}

fn format_day_month_year(date: NaiveDate) -> String {
    format!("{} {} {}", date.day(), month_label(date.month()), date.year())
}

fn format_time(datetime: DateTime<Local>) -> String {
    let hour = datetime.hour();
    let (display_hour, suffix) = match hour {
        0 => (12, "a. m."),
        1..=11 => (hour, "a. m."),
        12 => (12, "p. m."),
        _ => (hour - 12, "p. m."),
    };

    format!("{}:{:02} {}", display_hour, datetime.minute(), suffix)
}

fn month_label(month: u32) -> &'static str {
    match month {
        1 => "ene",
        2 => "feb",
        3 => "mar",
        4 => "abr",
        5 => "may",
        6 => "jun",
        7 => "jul",
        8 => "ago",
        9 => "sep",
        10 => "oct",
        11 => "nov",
        12 => "dic",
        _ => "mes",
    }
}

fn is_generic_title(title: &str) -> bool {
    let normalized = title.trim().to_lowercase();
    normalized == "plan nutricional"
        || normalized == "plan semanal"
        || normalized == "plan alimenticio"
        || normalized == "plan de comidas"
        || normalized.starts_with("plan nutricional #")
}
