use crate::models::Tag;
use crate::repositories::metadata_repository::MetadataRepository;
use crate::repositories::tag_repository::TagRepository;
use crate::utils::AppResult;

pub struct TagService<T, M>
where
    T: TagRepository,
    M: MetadataRepository,
{
    tag_repo: T,
    metadata_repo: M,
}

impl<T, M> TagService<T, M>
where
    T: TagRepository,
    M: MetadataRepository,
{
    pub fn new(tag_repo: T, metadata_repo: M) -> Self {
        Self {
            tag_repo,
            metadata_repo,
        }
    }

    pub fn get_all_tags(&self) -> AppResult<Vec<Tag>> {
        self.tag_repo.get_all()
    }

    pub fn create_tag(&self, name: String, color: String) -> AppResult<Tag> {
        let mut tags = self.tag_repo.get_all()?;
        let id = name.to_lowercase().replace(' ', "-");

        // Check if already exists
        if let Some(existing) = tags.iter().find(|t| t.id == id) {
            return Ok(existing.clone());
        }

        let new_tag = Tag {
            id: id.clone(),
            name,
            color,
        };

        tags.push(new_tag.clone());
        self.tag_repo.save_all(tags)?;
        Ok(new_tag)
    }

    pub fn delete_tag(&self, id: String) -> AppResult<()> {
        let mut tags = self.tag_repo.get_all()?;
        tags.retain(|t| t.id != id);
        self.tag_repo.save_all(tags)?;

        // Optional: clean up metadata?
        // For simplicity, we'll keep the string ID in metadata,
        // it just won't resolve to a known tag name/color in UI.

        Ok(())
    }

    pub fn add_tag_to_plan(&self, plan_id: String, tag_id: String) -> AppResult<()> {
        let mut meta = self.metadata_repo.get(&plan_id)?.unwrap_or_else(|| {
            crate::models::metadata::PlanMetadata {
                plan_id: plan_id.clone(),
                ..Default::default()
            }
        });

        if !meta.tags.contains(&tag_id) {
            meta.tags.push(tag_id);
            self.metadata_repo.save(meta)?;
        }

        Ok(())
    }

    pub fn remove_tag_from_plan(&self, plan_id: String, tag_id: String) -> AppResult<()> {
        if let Some(mut meta) = self.metadata_repo.get(&plan_id)? {
            meta.tags.retain(|t| t != &tag_id);
            self.metadata_repo.save(meta)?;
        }
        Ok(())
    }
}
