use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ExcludedIngredients {
    pub ingredients: Vec<String>,
}

impl Default for ExcludedIngredients {
    fn default() -> Self {
        Self {
            ingredients: Vec::new(),
        }
    }
}
