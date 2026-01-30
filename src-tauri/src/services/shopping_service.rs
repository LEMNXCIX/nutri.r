use crate::models::{ChatMessage, ShoppingItem, ShoppingList};
use crate::repositories::ConfigRepository;
use crate::repositories::{PlanRepository, ShoppingListRepository};
use crate::services::ai_service::OllamaService;
use crate::utils::{AppError, AppResult};
use chrono::Local;

pub struct ShoppingListService<S, P, C>
where
    S: ShoppingListRepository,
    P: PlanRepository,
    C: ConfigRepository,
{
    shopping_repo: S,
    plan_repo: P,
    config_repo: C,
    ai_service: OllamaService,
}

impl<S, P, C> ShoppingListService<S, P, C>
where
    S: ShoppingListRepository,
    P: PlanRepository,
    C: ConfigRepository,
{
    pub fn new(shopping_repo: S, plan_repo: P, config_repo: C) -> Self {
        Self {
            shopping_repo,
            plan_repo,
            config_repo,
            ai_service: OllamaService::new(),
        }
    }

    pub async fn generate_list_for_plan(&self, plan_id: &str) -> AppResult<ShoppingList> {
        // Get plan content
        let plan = self.plan_repo.get_by_id(plan_id)?;
        let config = self.config_repo.get()?;

        let prompt = format!(
            "Extrae todos los ingredientes necesarios para este plan nutricional y devuélvelos en formato JSON. 
            El formato debe ser un array de objetos con los campos: 'name' (nombre del ingrediente), 
            'category' (categoría como 'Carnes', 'Vegetales', 'Lácteos', 'Especias', etc.), 
            'quantity' (cantidad sugerida, ej: '500g', '2 unidades').
            
            IMPORTANTE: Solo devuelve el JSON puro, sin explicaciones, ni etiquetas markdown.
            
            Plan:\n{}",
            plan.markdown_content
        );

        let messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content:
                    "Eres un experto nutricionista que extrae listas de compras precisas en JSON."
                        .to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: prompt,
            },
        ];

        let response = self
            .ai_service
            .chat(&config.ollama_url, &config.ollama_model, messages)
            .await?;

        // Parse items from AI response
        // Try to find JSON array in the response if AI adds fluff
        let response_clean = if let Some(start) = response.find('[') {
            if let Some(end) = response.rfind(']') {
                &response[start..=end]
            } else {
                &response
            }
        } else {
            &response
        };

        let items: Vec<ShoppingItem> = serde_json::from_str(response_clean).map_err(|e| {
            AppError::Serialization(format!(
                "Failed to parse AI shopping items: {}. Response was: {}",
                e, response
            ))
        })?;

        let shopping_list = ShoppingList {
            id: format!("shop_{}", plan_id),
            plan_id: plan_id.to_string(),
            created_at: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            items,
        };

        self.shopping_repo.save(shopping_list.clone())?;

        Ok(shopping_list)
    }

    pub fn get_list(&self, plan_id: &str) -> AppResult<Option<ShoppingList>> {
        self.shopping_repo.get_by_plan_id(plan_id)
    }

    pub fn toggle_item(&self, plan_id: &str, item_name: &str, checked: bool) -> AppResult<()> {
        self.shopping_repo.update_item(plan_id, item_name, checked)
    }
}
