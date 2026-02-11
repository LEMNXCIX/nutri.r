use crate::models::PantryItem;
use crate::repositories::PantryRepository;
use crate::utils::error::AppResult;

pub struct PantryService<R: PantryRepository> {
    repo: R,
}

impl<R: PantryRepository> PantryService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub fn get_all_items(&self) -> AppResult<Vec<PantryItem>> {
        self.repo.get_all()
    }

    pub fn add_item(&self, item: PantryItem) -> AppResult<()> {
        let mut items = self.repo.get_all()?;
        // En una implementación real podríamos verificar si ya existe un item con el mismo ID o nombre
        items.push(item);
        self.repo.save_all(items)
    }

    pub fn update_item(&self, updated_item: PantryItem) -> AppResult<()> {
        let mut items = self.repo.get_all()?;
        if let Some(item) = items.iter_mut().find(|i| i.id == updated_item.id) {
            *item = updated_item;
        }
        self.repo.save_all(items)
    }

    pub fn delete_item(&self, id: &str) -> AppResult<()> {
        let mut items = self.repo.get_all()?;
        items.retain(|i| i.id != id);
        self.repo.save_all(items)
    }
}
