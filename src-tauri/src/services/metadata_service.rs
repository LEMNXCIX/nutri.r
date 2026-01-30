use crate::models::metadata::PlanMetadata;
use crate::repositories::metadata_repository::MetadataRepository;
use crate::utils::AppResult;

pub struct MetadataService<R: MetadataRepository> {
    repository: R,
}

impl<R: MetadataRepository> MetadataService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn get_metadata(&self, plan_id: String) -> AppResult<PlanMetadata> {
        let meta = self
            .repository
            .get(&plan_id)?
            .unwrap_or_else(|| PlanMetadata {
                plan_id,
                ..Default::default()
            });
        Ok(meta)
    }

    pub fn toggle_favorite(&self, plan_id: String) -> AppResult<bool> {
        let mut meta = self
            .repository
            .get(&plan_id)?
            .unwrap_or_else(|| PlanMetadata {
                plan_id: plan_id.clone(),
                ..Default::default()
            });

        meta.is_favorite = !meta.is_favorite;
        self.repository.save(meta.clone())?;
        Ok(meta.is_favorite)
    }

    pub fn set_rating(&self, plan_id: String, rating: u8) -> AppResult<()> {
        let mut meta = self
            .repository
            .get(&plan_id)?
            .unwrap_or_else(|| PlanMetadata {
                plan_id: plan_id.clone(),
                ..Default::default()
            });

        let r = rating.max(1).min(5);
        meta.rating = Some(r);

        self.repository.save(meta)?;
        Ok(())
    }

    pub fn set_note(&self, plan_id: String, note: String) -> AppResult<()> {
        let mut meta = self
            .repository
            .get(&plan_id)?
            .unwrap_or_else(|| PlanMetadata {
                plan_id: plan_id.clone(),
                ..Default::default()
            });

        meta.notes = note;
        self.repository.save(meta)?;
        Ok(())
    }

    pub fn get_favorites(&self) -> AppResult<Vec<PlanMetadata>> {
        let all = self.repository.get_all()?;
        Ok(all.into_iter().filter(|m| m.is_favorite).collect())
    }
}
