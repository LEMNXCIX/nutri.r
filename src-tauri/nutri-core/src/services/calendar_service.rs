use crate::models::{CalendarEntry, MealType, StructuredPlan, WeeklyMealInfo};
use crate::repositories::CalendarRepository;
use crate::utils::{AppError, AppResult};

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

    pub fn assign_plan(
        &self,
        date: String,
        meal_type: MealType,
        plan_id: String,
        recipe_id: Option<String>,
        plan_day_index: Option<u8>,
        assignment_id: Option<String>,
    ) -> AppResult<()> {
        let entry = CalendarEntry {
            date,
            meal_type,
            plan_id,
            assignment_id,
            plan_day_index,
            recipe_id,
        };
        self.calendar_repo.save(entry)
    }

    pub fn get_range(&self, start_date: &str, end_date: &str) -> AppResult<Vec<CalendarEntry>> {
        self.calendar_repo.get_range(start_date, end_date)
    }

    pub fn remove_entry(&self, date: &str, meal_type: MealType) -> AppResult<()> {
        self.calendar_repo.remove(date, meal_type)
    }

    pub fn swap_entries(
        &self,
        first_date: &str,
        first_meal_type: MealType,
        second_date: &str,
        second_meal_type: MealType,
    ) -> AppResult<()> {
        if first_meal_type != second_meal_type {
            return Err(AppError::Validation(
                "Solo se pueden intercambiar recetas del mismo tipo de comida".to_string(),
            ));
        }

        let mut entries = self.calendar_repo.get_all()?;
        let first_index = entries
            .iter()
            .position(|entry| entry.date == first_date && entry.meal_type == first_meal_type)
            .ok_or_else(|| {
                AppError::NotFound(format!(
                    "No se encontró la entrada {} {:?}",
                    first_date, first_meal_type
                ))
            })?;
        let second_index = entries
            .iter()
            .position(|entry| entry.date == second_date && entry.meal_type == second_meal_type)
            .ok_or_else(|| {
                AppError::NotFound(format!(
                    "No se encontró la entrada {} {:?}",
                    second_date, second_meal_type
                ))
            })?;

        let first_entry = entries[first_index].clone();
        let second_entry = entries[second_index].clone();

        if first_entry.plan_id != second_entry.plan_id {
            return Err(AppError::Validation(
                "Solo se pueden intercambiar recetas del mismo plan".to_string(),
            ));
        }

        if first_entry.assignment_id.is_none()
            || first_entry.assignment_id != second_entry.assignment_id
        {
            return Err(AppError::Validation(
                "Solo se pueden intercambiar recetas de la misma asignación semanal".to_string(),
            ));
        }

        if first_entry.recipe_id.is_none() || second_entry.recipe_id.is_none() {
            return Err(AppError::Validation(
                "Estas entradas no tienen recetas intercambiables".to_string(),
            ));
        }

        entries[first_index].recipe_id = second_entry.recipe_id.clone();
        entries[first_index].plan_day_index = second_entry.plan_day_index;
        entries[second_index].recipe_id = first_entry.recipe_id.clone();
        entries[second_index].plan_day_index = first_entry.plan_day_index;

        self.calendar_repo.save_all(entries)
    }

    pub fn assign_weekly_plan_to_date(
        &self,
        start_date: &str,
        plan_id: &str,
        structured_plan: Option<StructuredPlan>,
        weekly_structure: Option<Vec<WeeklyMealInfo>>,
        default_meal_type: MealType,
    ) -> AppResult<()> {
        let start = chrono::NaiveDate::parse_from_str(start_date, "%Y-%m-%d").map_err(|e| {
            crate::utils::AppError::Validation(format!("Invalid date format: {}", e))
        })?;

        let assignment_id = format!(
            "{}-{}",
            plan_id,
            chrono::Local::now().format("%Y%m%d%H%M%S")
        );

        if let Some(plan) = structured_plan
            .map(StructuredPlan::normalized)
            .filter(|plan| !plan.days.is_empty())
        {
            for day in plan.days {
                let current_date = start + chrono::Duration::days(day.day_index as i64);
                for recipe in day.recipes {
                    let entry = CalendarEntry {
                        date: current_date.format("%Y-%m-%d").to_string(),
                        meal_type: recipe.meal_type,
                        plan_id: plan_id.to_string(),
                        assignment_id: Some(assignment_id.clone()),
                        plan_day_index: Some(day.day_index),
                        recipe_id: Some(recipe.recipe_id),
                    };
                    self.calendar_repo.save(entry)?;
                }
            }

            return Ok(());
        }

        let structure = match weekly_structure {
            Some(s) if !s.is_empty() => s,
            _ => vec![WeeklyMealInfo {
                day_index: 0,
                meal_type: String::new(),
                description: None,
                day_id: None,
                recipe_id: None,
            }],
        };

        for meal_info in structure {
            let current_date = start + chrono::Duration::days(meal_info.day_index as i64);
            let meal_type =
                MealType::from_label(&meal_info.meal_type).unwrap_or(default_meal_type.clone());

            let entry = CalendarEntry {
                date: current_date.format("%Y-%m-%d").to_string(),
                meal_type,
                plan_id: plan_id.to_string(),
                assignment_id: Some(assignment_id.clone()),
                plan_day_index: Some(meal_info.day_index),
                recipe_id: meal_info.recipe_id.clone(),
            };
            self.calendar_repo.save(entry)?;
        }

        Ok(())
    }
}
