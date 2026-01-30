use crate::models::ShoppingList;
use crate::utils::{AppError, AppResult};
use std::fs;
use std::path::PathBuf;

pub trait ShoppingListRepository {
    fn get_by_plan_id(&self, plan_id: &str) -> AppResult<Option<ShoppingList>>;
    fn get_all(&self) -> AppResult<Vec<ShoppingList>>;
    fn save(&self, list: ShoppingList) -> AppResult<()>;
    fn update_item(&self, plan_id: &str, item_name: &str, checked: bool) -> AppResult<()>;
}

pub struct FileShoppingListRepository {
    data_dir: PathBuf,
}

impl FileShoppingListRepository {
    pub fn new(data_dir: PathBuf) -> Self {
        Self { data_dir }
    }

    fn file_path(&self) -> PathBuf {
        self.data_dir.join("shopping_lists.json")
    }

    fn read_all(&self) -> AppResult<Vec<ShoppingList>> {
        let path = self.file_path();
        if !path.exists() {
            return Ok(Vec::new());
        }
        let content = fs::read_to_string(&path)
            .map_err(|e| AppError::Database(format!("Failed to read shopping lists: {}", e)))?;
        serde_json::from_str(&content)
            .map_err(|e| AppError::Serialization(format!("Failed to parse shopping lists: {}", e)))
    }

    fn write_all(&self, lists: Vec<ShoppingList>) -> AppResult<()> {
        let path = self.file_path();
        let json = serde_json::to_string_pretty(&lists).map_err(|e| {
            AppError::Serialization(format!("Failed to serialize shopping lists: {}", e))
        })?;
        fs::write(&path, json)
            .map_err(|e| AppError::Database(format!("Failed to write shopping lists: {}", e)))
    }
}

impl ShoppingListRepository for FileShoppingListRepository {
    fn get_by_plan_id(&self, plan_id: &str) -> AppResult<Option<ShoppingList>> {
        let lists = self.read_all()?;
        Ok(lists.into_iter().find(|l| l.plan_id == plan_id))
    }

    fn get_all(&self) -> AppResult<Vec<ShoppingList>> {
        self.read_all()
    }

    fn save(&self, list: ShoppingList) -> AppResult<()> {
        let mut lists = self.read_all()?;
        if let Some(existing) = lists.iter_mut().find(|l| l.plan_id == list.plan_id) {
            *existing = list;
        } else {
            lists.push(list);
        }
        self.write_all(lists)
    }

    fn update_item(&self, plan_id: &str, item_name: &str, checked: bool) -> AppResult<()> {
        let mut lists = self.read_all()?;
        if let Some(list) = lists.iter_mut().find(|l| l.plan_id == plan_id) {
            if let Some(item) = list.items.iter_mut().find(|i| i.name == item_name) {
                item.checked = checked;
                return self.write_all(lists);
            }
        }
        Err(AppError::NotFound(format!(
            "Shopping item {} not found for plan {}",
            item_name, plan_id
        )))
    }
}
