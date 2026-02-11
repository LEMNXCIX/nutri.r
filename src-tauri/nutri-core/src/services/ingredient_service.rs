use crate::models::ExcludedIngredients;
use crate::repositories::{IngredientRepository, PlanRepository};
use crate::utils::AppResult;
use serde::Serialize;
use std::collections::HashMap;

/// Statistics about ingredient usage
#[derive(Debug, Clone, Serialize)]
pub struct IngredientStats {
    pub name: String,
    pub count: usize,
    pub is_excluded: bool,
}

/// Service for ingredient management
pub struct IngredientService<P, I>
where
    P: PlanRepository,
    I: IngredientRepository,
{
    plan_repo: P,
    ingredient_repo: I,
}

impl<P, I> IngredientService<P, I>
where
    P: PlanRepository,
    I: IngredientRepository,
{
    pub fn new(plan_repo: P, ingredient_repo: I) -> Self {
        Self {
            plan_repo,
            ingredient_repo,
        }
    }

    /// Get ingredient statistics from all plans
    pub fn get_statistics(&self) -> AppResult<Vec<IngredientStats>> {
        let plans = self.plan_repo.get_all()?;
        let excluded = self.ingredient_repo.get_excluded()?;

        // Count ingredient frequency
        let mut ingredient_count: HashMap<String, usize> = HashMap::new();

        for plan in plans {
            for protein in plan.proteinas {
                *ingredient_count.entry(protein).or_insert(0) += 1;
            }
        }

        // Convert to stats and sort by frequency
        let mut stats: Vec<IngredientStats> = ingredient_count
            .into_iter()
            .map(|(name, count)| IngredientStats {
                name: name.clone(),
                count,
                is_excluded: excluded.ingredients.contains(&name),
            })
            .collect();

        stats.sort_by(|a, b| b.count.cmp(&a.count));

        Ok(stats)
    }

    /// Get excluded ingredients
    pub fn get_excluded(&self) -> AppResult<Vec<String>> {
        let excluded = self.ingredient_repo.get_excluded()?;
        Ok(excluded.ingredients)
    }

    /// Save excluded ingredients
    pub fn save_excluded(&self, ingredients: Vec<String>) -> AppResult<()> {
        let excluded = ExcludedIngredients { ingredients };
        self.ingredient_repo.save_excluded(&excluded)
    }

    /// Toggle ingredient exclusion
    pub fn toggle_exclusion(&self, ingredient: &str) -> AppResult<Vec<String>> {
        let mut excluded = self.ingredient_repo.get_excluded()?;

        if let Some(pos) = excluded.ingredients.iter().position(|x| x == ingredient) {
            excluded.ingredients.remove(pos);
        } else {
            excluded.ingredients.push(ingredient.to_string());
        }

        self.ingredient_repo.save_excluded(&excluded)?;
        Ok(excluded.ingredients)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::{FileIngredientRepository, FilePlanRepository};
    use std::env;

    #[test]
    fn test_ingredient_service_creation() {
        let temp_dir = env::temp_dir().join("nutri_r_test_ingredient_service");
        let plan_repo = FilePlanRepository::new(temp_dir.clone());
        let ingredient_repo = FileIngredientRepository::new(temp_dir.join("excluded.json"));

        let _service = IngredientService::new(plan_repo, ingredient_repo);

        // Cleanup
        let _ = std::fs::remove_dir_all(temp_dir);
    }
}
