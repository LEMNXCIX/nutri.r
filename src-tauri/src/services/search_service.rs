use crate::models::{PlanIndex, SearchFilters};
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

        // Perform filtering
        let filtered: Vec<PlanIndex> = plans
            .into_iter()
            .filter(|plan| {
                // Filter by query (title/content/date)
                if let Some(query) = &filters.query {
                    let q = query.to_lowercase();
                    if !plan.id.to_lowercase().contains(&q)
                        && !plan.fecha.to_lowercase().contains(&q)
                    {
                        // We don't have full content here, but we can search in protein list
                        if !plan.proteinas.iter().any(|p| p.to_lowercase().contains(&q)) {
                            return false;
                        }
                    }
                }

                // Filter by protein content specifically
                if let Some(protein) = &filters.protein_contains {
                    let p = protein.to_lowercase();
                    if !plan
                        .proteinas
                        .iter()
                        .any(|item| item.to_lowercase().contains(&p))
                    {
                        return false;
                    }
                }

                // Metadata filters
                let metadata = self.metadata_repo.get(&plan.id).ok().flatten();

                if filters.only_favorites {
                    if let Some(meta) = &metadata {
                        if !meta.is_favorite {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }

                if let Some(min_r) = filters.min_rating {
                    if let Some(meta) = &metadata {
                        if meta.rating.unwrap_or(0) < min_r {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }

                true
            })
            .map(|mut plan| {
                // Attach metadata to the result
                if let Ok(Some(meta)) = self.metadata_repo.get(&plan.id) {
                    plan.is_favorite = meta.is_favorite;
                    plan.rating = meta.rating;
                }
                plan
            })
            .collect();

        Ok(filtered)
    }
}
