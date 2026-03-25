use crate::models::{derive_plan_display_name, PlanIndex, SearchFilters};
use crate::repositories::metadata_repository::MetadataRepository;
use crate::repositories::PlanRepository;
use crate::utils::AppResult;

pub struct SearchService<P, M>
where
    P: PlanRepository,
    M: MetadataRepository,
{
    plan_repo: P,
    metadata_repo: M,
}

impl<P, M> SearchService<P, M>
where
    P: PlanRepository,
    M: MetadataRepository,
{
    pub fn new(plan_repo: P, metadata_repo: M) -> Self {
        Self {
            plan_repo,
            metadata_repo,
        }
    }

    pub fn search(&self, filters: SearchFilters) -> AppResult<Vec<PlanIndex>> {
        let plans = self.plan_repo.get_all()?;
        let mut filtered = Vec::new();

        for mut plan in plans {
            let metadata = self.metadata_repo.get(&plan.id)?;
            let structured_plan = if metadata
                .as_ref()
                .and_then(|meta| meta.display_name.as_ref())
                .is_some()
            {
                None
            } else {
                self.plan_repo
                    .get_by_id(&plan.id)
                    .ok()
                    .and_then(|detail| detail.structured_plan)
            };

            if let Some(meta) = metadata.as_ref() {
                plan.is_favorite = meta.is_favorite;
                plan.rating = meta.rating;
            }

            let created_at = plan
                .created_at
                .as_deref()
                .and_then(|value| chrono::DateTime::parse_from_rfc3339(value).ok())
                .map(|value| value.with_timezone(&chrono::Utc));
            let resolved_name = metadata
                .as_ref()
                .and_then(|meta| meta.display_name.as_ref())
                .map(|value| value.trim())
                .filter(|value| !value.is_empty())
                .map(|value| value.to_string())
                .unwrap_or_else(|| {
                    derive_plan_display_name(
                        structured_plan.as_ref(),
                        created_at.as_ref(),
                        &plan.proteinas,
                        &plan.id,
                    )
                });
            plan.display_name = Some(resolved_name.clone());

            if let Some(query) = &filters.query {
                let q = query.to_lowercase();
                let matches_query = plan.id.to_lowercase().contains(&q)
                    || plan.fecha.to_lowercase().contains(&q)
                    || resolved_name.to_lowercase().contains(&q)
                    || plan
                        .proteinas
                        .iter()
                        .any(|protein| protein.to_lowercase().contains(&q));

                if !matches_query {
                    continue;
                }
            }

            if let Some(protein) = &filters.protein_contains {
                let protein_query = protein.to_lowercase();
                if !plan
                    .proteinas
                    .iter()
                    .any(|item| item.to_lowercase().contains(&protein_query))
                {
                    continue;
                }
            }

            if filters.only_favorites && !plan.is_favorite {
                continue;
            }

            if let Some(min_rating) = filters.min_rating {
                if plan.rating.unwrap_or(0) < min_rating {
                    continue;
                }
            }

            filtered.push(plan);
        }

        Ok(filtered)
    }
}
