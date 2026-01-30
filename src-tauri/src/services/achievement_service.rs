use crate::models::achievement::{Achievement, AchievementType};
use crate::repositories::AchievementRepository;
use crate::repositories::IngredientRepository;
use crate::repositories::MetadataRepository;
use crate::repositories::PlanRepository;
use crate::repositories::ShoppingListRepository;
use crate::utils::error::AppResult;

pub struct AchievementService<R, P, S, I, M>
where
    R: AchievementRepository,
    P: PlanRepository,
    S: ShoppingListRepository,
    I: IngredientRepository,
    M: MetadataRepository,
{
    repo: R,
    plan_repo: P,
    shopping_repo: S,
    ingredient_repo: I,
    metadata_repo: M,
}

impl<R, P, S, I, M> AchievementService<R, P, S, I, M>
where
    R: AchievementRepository,
    P: PlanRepository,
    S: ShoppingListRepository,
    I: IngredientRepository,
    M: MetadataRepository,
{
    pub fn new(
        repo: R,
        plan_repo: P,
        shopping_repo: S,
        ingredient_repo: I,
        metadata_repo: M,
    ) -> Self {
        Self {
            repo,
            plan_repo,
            shopping_repo,
            ingredient_repo,
            metadata_repo,
        }
    }

    pub fn get_achievements(&self) -> AppResult<Vec<Achievement>> {
        self.check_all_conditions()?;
        self.repo.get_all()
    }

    fn check_all_conditions(&self) -> AppResult<()> {
        let achievements = self.repo.get_all()?;

        // Get counts from repositories
        let plan_count = self.plan_repo.get_all()?.len() as i32;
        let excluded_count = self.ingredient_repo.get_excluded()?.ingredients.len() as i32;
        let shopping_count = self.shopping_repo.get_all()?.len() as i32;

        // Count rated plans (this is inefficient for large datasets but ok for MVP)
        // We need a method in metadata repo to count ratings or we just iterate if feasible.
        // For MVP assuming get_all from metadata repo (if it existed) or counting is tricky.
        // Let's assume we implement a simple count in metadata repo or skip complex logic.
        // Actually, MetadataRepo keys are plan IDs. We'd need to iterate all metadata files.
        // Let's simplify: just check "first rated plan" via existing metadata service logic if possible,
        // or just rely on manual checks if metadata repo doesn't support "get_all" efficiently.
        // Wait, MetadataRepository::get returns specific ID. It doesn't have get_all.
        // But FileMetadataRepository stores separate files.
        // Let's skip PlanRated check for this step unless I add get_all to MetadataRepo.
        // Or I can iterate plans and check metadata for each.

        let mut rated_count = 0;
        let plans = self.plan_repo.get_all()?;
        for plan in plans {
            if let Ok(Some(meta)) = self.metadata_repo.get(&plan.id) {
                if meta.rating.unwrap_or(0) > 0 {
                    rated_count += 1;
                }
            }
        }

        for achievement in achievements {
            if achievement.unlocked_at.is_none() {
                let should_unlock = match achievement.condition_type {
                    AchievementType::PlanCreated => plan_count >= achievement.condition_value,
                    AchievementType::IngredientExcluded => {
                        excluded_count >= achievement.condition_value
                    }
                    AchievementType::ShoppingListGenerated => {
                        shopping_count >= achievement.condition_value
                    }
                    AchievementType::PlanRated => rated_count >= achievement.condition_value,
                };

                if should_unlock {
                    self.repo.unlock(&achievement.id)?;
                }
            }
        }

        Ok(())
    }
}
