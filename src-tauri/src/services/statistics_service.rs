use crate::models::{IngredientTrend, MealType, MonthlyData, Statistics};
use crate::repositories::{CalendarRepository, MetadataRepository, PlanRepository};
use crate::utils::AppResult;
use chrono::{Datelike, NaiveDate};
use std::collections::HashMap;

pub struct StatisticsService<P, M, C>
where
    P: PlanRepository,
    M: MetadataRepository,
    C: CalendarRepository,
{
    plan_repo: P,
    metadata_repo: M,
    calendar_repo: C,
}

impl<P, M, C> StatisticsService<P, M, C>
where
    P: PlanRepository,
    M: MetadataRepository,
    C: CalendarRepository,
{
    pub fn new(plan_repo: P, metadata_repo: M, calendar_repo: C) -> Self {
        Self {
            plan_repo,
            metadata_repo,
            calendar_repo,
        }
    }

    pub fn get_statistics(&self) -> AppResult<Statistics> {
        let plans = self.plan_repo.get_all()?;
        let all_metadata = self.metadata_repo.get_all()?;

        // Use a wide range for calendar entries to get a good distribution
        let entries = self.calendar_repo.get_range("2024-01-01", "2026-12-31")?;

        let mut stats = Statistics::default();
        stats.total_plans = plans.len();
        stats.favorite_plans = all_metadata.iter().filter(|m| m.is_favorite).count();

        let mut meal_distribution = HashMap::new();
        for entry in entries {
            let meal_name = match entry.meal_type {
                MealType::Breakfast => "Desayuno",
                MealType::Lunch => "Almuerzo",
                MealType::Dinner => "Cena",
                MealType::Snack => "Merienda",
            };
            *meal_distribution.entry(meal_name.to_string()).or_insert(0) += 1;
        }
        stats.meal_distribution = meal_distribution;

        // Monthly activity from plan dates
        let mut monthly_counts: HashMap<String, usize> = HashMap::new();
        for plan in plans {
            if let Ok(date) = NaiveDate::parse_from_str(&plan.fecha, "%Y-%m-%d") {
                let month_key = format!("{}-{:02}", date.year(), date.month());
                *monthly_counts.entry(month_key).or_insert(0) += 1;
            }
        }

        let mut monthly_activity: Vec<MonthlyData> = monthly_counts
            .into_iter()
            .map(|(month, count)| MonthlyData { month, count })
            .collect();

        monthly_activity.sort_by(|a, b| a.month.cmp(&b.month));
        stats.monthly_activity = monthly_activity;

        Ok(stats)
    }

    pub fn get_ingredient_trends(&self) -> AppResult<Vec<IngredientTrend>> {
        let plans = self.plan_repo.get_all()?;
        let mut counts: HashMap<String, usize> = HashMap::new();

        for plan in plans {
            for protein in plan.proteinas {
                *counts.entry(protein).or_insert(0) += 1;
            }
        }

        let mut trends: Vec<IngredientTrend> = counts
            .into_iter()
            .map(|(name, count)| IngredientTrend { name, count })
            .collect();

        trends.sort_by(|a, b| b.count.cmp(&a.count));

        // Take top 10
        trends.truncate(10);

        Ok(trends)
    }
}
