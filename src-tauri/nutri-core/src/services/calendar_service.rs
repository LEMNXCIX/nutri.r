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
}
