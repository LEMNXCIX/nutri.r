use super::{PlanDetail, PlanIndex, PlanMetadata};
use chrono::{DateTime, Datelike, Local, NaiveDate, NaiveDateTime, Timelike};
use serde::{Deserialize, Serialize};

/// Shared context required to render a plan email consistently across runtimes.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlanEmailContext {
    pub plan_id: String,
    pub short_reference: String,
    pub display_name: String,
    pub created_at_label: Option<String>,
    pub plan_detail: PlanDetail,
    pub plan_index: PlanIndex,
    pub metadata: Option<PlanMetadata>,
}

/// Final subject/body pair ready to pass to the SMTP transport.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RenderedEmail {
    pub subject: String,
    pub html: String,
}

pub fn short_plan_reference(plan_id: &str) -> String {
    plan_id.chars().take(6).collect::<String>().to_uppercase()
}

pub fn format_plan_created_at_for_email(plan: &PlanIndex) -> Option<String> {
    if let Some(local_dt) = plan
        .created_at
        .as_deref()
        .and_then(|value| DateTime::parse_from_rfc3339(value).ok())
        .map(|value| value.with_timezone(&Local))
    {
        return Some(format_local_datetime(local_dt));
    }

    if let Some(fecha) = legacy_created_at_label(plan) {
        return Some(fecha);
    }

    let fecha = plan.fecha.trim();
    (!fecha.is_empty()).then(|| fecha.to_string())
}

fn legacy_created_at_label(plan: &PlanIndex) -> Option<String> {
    let fecha = plan.fecha.trim();
    if !fecha.is_empty() {
        if let Ok(datetime) = NaiveDateTime::parse_from_str(fecha, "%Y-%m-%d %H:%M:%S") {
            return Some(format_naive_datetime(datetime));
        }

        if let Ok(date) = NaiveDate::parse_from_str(fecha, "%Y-%m-%d") {
            return Some(format_date(date));
        }
    }

    let digits = plan
        .id
        .chars()
        .filter(|char| char.is_ascii_digit())
        .collect::<String>();

    if digits.len() >= 14 {
        if let Ok(datetime) = NaiveDateTime::parse_from_str(&digits[..14], "%Y%m%d%H%M%S") {
            return Some(format_naive_datetime(datetime));
        }
    }

    if digits.len() >= 12 {
        if let Ok(datetime) = NaiveDateTime::parse_from_str(&digits[..12], "%Y%m%d%H%M") {
            return Some(format_naive_datetime(datetime));
        }
    }

    None
}

fn format_local_datetime(datetime: DateTime<Local>) -> String {
    format!(
        "{} {} {} · {}",
        datetime.day(),
        month_label(datetime.month()),
        datetime.year(),
        format_time(datetime.hour(), datetime.minute()),
    )
}

fn format_naive_datetime(datetime: NaiveDateTime) -> String {
    format!(
        "{} {} {} · {}",
        datetime.day(),
        month_label(datetime.month()),
        datetime.year(),
        format_time(datetime.hour(), datetime.minute()),
    )
}

fn format_date(date: NaiveDate) -> String {
    format!(
        "{} {} {}",
        date.day(),
        month_label(date.month()),
        date.year()
    )
}

fn format_time(hour: u32, minute: u32) -> String {
    let (display_hour, suffix) = match hour {
        0 => (12, "a. m."),
        1..=11 => (hour, "a. m."),
        12 => (12, "p. m."),
        _ => (hour - 12, "p. m."),
    };

    format!("{}:{:02} {}", display_hour, minute, suffix)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_plan_created_at_should_prefer_rfc3339() {
        let label = format_plan_created_at_for_email(&PlanIndex {
            id: "20260325121030".to_string(),
            fecha: String::new(),
            created_at: Some("2026-03-25T12:10:30Z".to_string()),
            display_name: None,
            proteinas: vec![],
            enviado: false,
            is_favorite: false,
            rating: None,
            weekly_structure: None,
        });

        assert!(label.as_deref().unwrap_or_default().contains("mar 2026"));
    }

    #[test]
    fn short_plan_reference_should_uppercase_first_six_chars() {
        assert_eq!(short_plan_reference("abc123xyz"), "ABC123");
    }
}
