use crate::models::{CalendarEntry, MealType};
use crate::repositories::CalendarRepository;
use crate::utils::AppResult;

pub struct CalendarService<R>
where
    R: CalendarRepository,
{
    calendar_repo: R,
}

impl<R> CalendarService<R>
where
    R: CalendarRepository,
{
    pub fn new(calendar_repo: R) -> Self {
        Self { calendar_repo }
    }

    pub fn assign_plan(&self, date: String, meal_type: MealType, plan_id: String) -> AppResult<()> {
        let entry = CalendarEntry {
            date,
            meal_type,
            plan_id,
        };
        self.calendar_repo.save(entry)
    }

    pub fn get_range(&self, start_date: &str, end_date: &str) -> AppResult<Vec<CalendarEntry>> {
        self.calendar_repo.get_range(start_date, end_date)
    }

    pub fn remove_entry(&self, date: &str, meal_type: MealType) -> AppResult<()> {
        self.calendar_repo.remove(date, meal_type)
    }

    pub fn assign_weekly_plan_to_date(
        &self,
        start_date: &str,
        plan_id: &str,
        weekly_structure: Option<Vec<crate::models::WeeklyMealInfo>>,
        default_meal_type: MealType,
    ) -> AppResult<()> {
        let start = chrono::NaiveDate::parse_from_str(start_date, "%Y-%m-%d").map_err(|e| {
            crate::utils::AppError::Validation(format!("Invalid date format: {}", e))
        })?;

        let structure = match weekly_structure {
            Some(s) if !s.is_empty() => s,
            _ => vec![crate::models::WeeklyMealInfo {
                day_index: 0,
                meal_type: String::new(),
                description: None,
            }],
        };

        for meal_info in structure {
            let current_date = start + chrono::Duration::days(meal_info.day_index as i64);
            let meal_type = match meal_info.meal_type.to_lowercase().as_str() {
                "breakfast" => MealType::Breakfast,
                "lunch" => MealType::Lunch,
                "dinner" => MealType::Dinner,
                "snack" => MealType::Snack,
                _ => default_meal_type.clone(),
            };

            let entry = CalendarEntry {
                date: current_date.format("%Y-%m-%d").to_string(),
                meal_type,
                plan_id: plan_id.to_string(),
            };
            self.calendar_repo.save(entry)?;
        }

        Ok(())
    }
}
