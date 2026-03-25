use crate::models::{
    ChatMessage, OllamaChatRequest, OllamaChatResponse, OllamaListResponse, OllamaModelInfo,
    RecipeSuggestion, StructuredDay, StructuredPlan, StructuredRecipe, WeeklyMealInfo,
};
use crate::utils::{AppError, AppResult};
use reqwest::Client;
use serde::de::DeserializeOwned;

/// Service for interacting with Ollama API
pub struct OllamaService {
    client: Client,
}

impl OllamaService {
    fn extract_json<T: DeserializeOwned>(&self, text: &str) -> AppResult<T> {
        serde_json::from_str::<T>(text)
            .or_else(|_| {
                let start = text.find('{').or_else(|| text.find('[')).ok_or_else(|| {
                    AppError::Serialization("No JSON payload found in AI response".to_string())
                })?;
                let end = text.rfind('}').or_else(|| text.rfind(']')).ok_or_else(|| {
                    AppError::Serialization("Incomplete JSON payload in AI response".to_string())
                })?;
                serde_json::from_str::<T>(&text[start..=end]).map_err(AppError::from)
            })
            .map_err(|e| match e {
                AppError::Serialization(_) => e,
                other => AppError::Serialization(other.to_string()),
            })
    }

    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    /// List available Ollama models
    pub async fn list_models(&self, ollama_url: &str) -> AppResult<Vec<OllamaModelInfo>> {
        let list_url = format!("{}/api/models", ollama_url);

        log::info!("Listing Ollama models from: {}", list_url);

        let response = self
            .client
            .get(&list_url)
            .send()
            .await
            .map_err(|e| AppError::Ollama(format!("Failed to connect to Ollama: {}", e)))?;

        let status = response.status();
        if !status.is_success() {
            let error_body = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::Ollama(format!(
                "Ollama returned error status {}: {}",
                status.as_u16(),
                error_body
            )));
        }

        let list_response: OllamaListResponse = response
            .json()
            .await
            .map_err(|e| AppError::Ollama(format!("Failed to parse Ollama response: {}", e)))?;

        Ok(list_response.models)
    }

    /// Generate a chat completion using Ollama
    pub async fn chat(
        &self,
        ollama_url: &str,
        model: &str,
        messages: Vec<ChatMessage>,
    ) -> AppResult<String> {
        let chat_url = format!("{}/api/chat", ollama_url);

        log::info!(
            "Sending chat request to Ollama: model={}, messages={}",
            model,
            messages.len()
        );

        let chat_request = OllamaChatRequest {
            model: model.to_string(),
            messages,
            stream: false,
        };

        let response = self
            .client
            .post(&chat_url)
            .json(&chat_request)
            .send()
            .await
            .map_err(|e| AppError::Ollama(format!("Failed to send request to Ollama: {}", e)))?;

        let status = response.status();
        if !status.is_success() {
            let error_body = response
                .text()
                .await
                .unwrap_or_else(|_| "Could not read error body".to_string());
            return Err(AppError::Ollama(format!(
                "Ollama returned error status {}: {}. URL: {}",
                status.as_u16(),
                error_body,
                chat_url
            )));
        }

        let chat_response: OllamaChatResponse = response
            .json()
            .await
            .map_err(|e| AppError::Ollama(format!("Failed to parse Ollama response: {}", e)))?;

        Ok(chat_response.message.content)
    }

    async fn extract_proteins(
        &self,
        ollama_url: &str,
        model: &str,
        markdown: &str,
    ) -> AppResult<Vec<String>> {
        let extract_prompt = format!(
            "Analiza el siguiente plan nutricional y extrae TODAS las fuentes de proteína (animal y vegetal).\n\
            Responde SOLAMENTE con un array JSON de strings.\n\
            EJEMPLO: [\"Pollo\", \"Lentejas\", \"Huevo\", \"Tofu\"]\n\
            NO añadas texto adicional, ni markdown, ni explicaciones.\n\
            PLAN:\n{}",
            markdown
        );

        let extract_messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content:
                    "Eres un extractor de datos JSON preciso. Solo respondes con el array JSON."
                        .to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: extract_prompt,
            },
        ];

        let content_text = self.chat(ollama_url, model, extract_messages).await?;
        log::info!("Raw extract text: {}", content_text);
        self.extract_json::<Vec<String>>(&content_text)
            .or_else(|_| Ok(Vec::new()))
    }

    async fn extract_structured_plan(
        &self,
        ollama_url: &str,
        model: &str,
        markdown: &str,
    ) -> AppResult<Option<StructuredPlan>> {
        let struct_prompt = format!(
            "Analiza el siguiente plan nutricional y conviertelo a JSON estructurado.\n\
            Responde SOLAMENTE con un objeto JSON con esta forma:\n\
            {{\"title\":\"...\",\"instructions\":\"...\",\"days\":[{{\"dayIndex\":0,\"label\":\"Lunes\",\"recipes\":[{{\"mealType\":\"Breakfast\",\"name\":\"...\",\"ingredients\":[\"...\"],\"instructions\":[\"...\"],\"notes\":\"...\"}}]}}]}}\n\
            Reglas:\n\
            - Usa mealType solo con Breakfast, Lunch, Dinner o Snack.\n\
            - Conserva el contenido del plan, no inventes dias vacios.\n\
            - instructions puede ser string a nivel del plan y array de strings en cada receta.\n\
            - No agregues texto fuera del JSON.\n\
            PLAN:\n{}",
            markdown
        );

        let struct_messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content:
                    "Eres un extractor de datos JSON preciso. Solo respondes con el objeto JSON."
                        .to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: struct_prompt,
            },
        ];

        let struct_text = self.chat(ollama_url, model, struct_messages).await?;
        log::info!("Raw structured plan text: {}", struct_text);
        Ok(self
            .extract_json::<StructuredPlan>(&struct_text)
            .ok()
            .map(StructuredPlan::normalized))
    }

    async fn extract_weekly_structure(
        &self,
        ollama_url: &str,
        model: &str,
        markdown: &str,
    ) -> AppResult<Option<Vec<WeeklyMealInfo>>> {
        let struct_prompt = format!(
            "Analiza el siguiente plan nutricional y extrae la estructura semanal de comidas.\n\
            Responde SOLAMENTE con un array JSON de objetos.\n\
            Formato de cada objeto:\n\
            {{\"dayIndex\": numero_de_0_a_6, \"mealType\": \"Breakfast\"|\"Lunch\"|\"Dinner\"|\"Snack\"|\"Unknown\", \"description\": \"breve resumen\"}}\n\
            Lunes = 0, Domingo = 6.\n\
            NO añadas texto adicional.\n\
            PLAN:\n{}",
            markdown
        );

        let struct_messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content:
                    "Eres un extractor de datos JSON preciso. Solo respondes con el array JSON."
                        .to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: struct_prompt,
            },
        ];

        let struct_text = self.chat(ollama_url, model, struct_messages).await?;
        log::info!("Raw weekly structure text: {}", struct_text);
        Ok(self.extract_json::<Vec<WeeklyMealInfo>>(&struct_text).ok())
    }

    /// Generate a meal plan with protein extraction
    pub async fn generate_plan(
        &self,
        ollama_url: &str,
        model: &str,
        prompt: String,
        exclusion_list: String,
        pantry_list: String,
    ) -> AppResult<(
        String,
        Vec<String>,
        Option<Vec<crate::models::WeeklyMealInfo>>,
        Option<StructuredPlan>,
    )> {
        // Step 1: Generate meal plan
        let mut full_prompt = prompt;

        if !exclusion_list.is_empty() {
            full_prompt = format!(
                "{}\n\nIMPORTANTE: NO uses los siguientes ingredientes (están excluidos por el usuario): {}",
                full_prompt, exclusion_list
            );
        }

        if !pantry_list.is_empty() {
            full_prompt = format!(
                "{}\n\nRECOMENDACIÓN: El usuario dispone de los siguientes ingredientes en su despensa con sus respectivas cantidades disponibles: {}.\n\nInstrucciones para la receta:\n- Usa TODOS estos ingredientes en la medida de lo posible.\n- Ajusta las porciones y cantidades de la receta para reflejar exactamente lo que está disponible.\n- Si algún ingrediente falta o no es suficiente, complementa con otros ingredientes externos, pero nunca ignores lo que ya está en la despensa.\n- Explica claramente cómo se usan y en qué cantidad dentro de la receta.",
                full_prompt, pantry_list
            );
        }

        let messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content: "Eres un chef experto en nutrición. Genera planes nutricionales balanceados en formato Markdown sin emojis.".to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: full_prompt,
            },
        ];

        let markdown = self.chat(ollama_url, model, messages).await?;

        if markdown.is_empty() {
            return Err(AppError::Ollama("Generated plan is empty".to_string()));
        }

        let proteins = self.extract_proteins(ollama_url, model, &markdown).await?;
        log::info!("Extracted proteins: {:?}", proteins);

        let structured_plan = self
            .extract_structured_plan(ollama_url, model, &markdown)
            .await?;

        let weekly_structure = if let Some(plan) = structured_plan.as_ref() {
            Some(plan.to_weekly_structure())
        } else {
            self.extract_weekly_structure(ollama_url, model, &markdown)
                .await?
        };

        log::info!(
            "Extracted structure items: {:?}",
            weekly_structure.as_ref().map(|v| v.len())
        );

        Ok((markdown, proteins, weekly_structure, structured_plan))
    }

    pub async fn suggest_recipe_edit(
        &self,
        ollama_url: &str,
        model: &str,
        plan_id: &str,
        plan: &StructuredPlan,
        day: &StructuredDay,
        recipe: &StructuredRecipe,
        suggestion: &str,
    ) -> AppResult<RecipeSuggestion> {
        let prompt = format!(
            "Actualiza SOLO la receta indicada a partir de la sugerencia del usuario.\n\
            Responde SOLAMENTE con un objeto JSON para la receta con esta forma:\n\
            {{\"mealType\":\"{}\",\"name\":\"...\",\"ingredients\":[\"...\"],\"instructions\":[\"...\"],\"notes\":\"...\"}}\n\
            Reglas:\n\
            - Mantén el mismo mealType de la receta original.\n\
            - No edites otras recetas ni otros días.\n\
            - Devuelve una receta completa y coherente, lista para reemplazar a la original.\n\
            - No agregues texto fuera del JSON.\n\
            Contexto del plan: {}\n\
            Día actual: {}\n\
            Receta original:\n{}\n\
            Sugerencia del usuario:\n{}",
            recipe.meal_type.key(),
            plan.title,
            day.label,
            serde_json::to_string_pretty(recipe)
                .unwrap_or_else(|_| "{}".to_string()),
            suggestion
        );

        let messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content:
                    "Eres un chef experto en nutrición. Solo respondes con JSON válido para una receta."
                        .to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: prompt,
            },
        ];

        let suggested_text = self.chat(ollama_url, model, messages).await?;
        let mut suggested_recipe = self.extract_json::<StructuredRecipe>(&suggested_text)?;
        suggested_recipe.recipe_id = recipe.recipe_id.clone();
        suggested_recipe.meal_type = recipe.meal_type.clone();
        if suggested_recipe.name.trim().is_empty() {
            suggested_recipe.name = recipe.name.clone();
        }
        if suggested_recipe.ingredients.is_empty() {
            suggested_recipe.ingredients = recipe.ingredients.clone();
        }
        if suggested_recipe.instructions.is_empty() {
            suggested_recipe.instructions = recipe.instructions.clone();
        }

        Ok(RecipeSuggestion {
            plan_id: plan_id.to_string(),
            day_id: day.day_id.clone(),
            recipe_id: recipe.recipe_id.clone(),
            original_recipe: recipe.clone(),
            suggested_recipe,
        })
    }
}

impl Default for OllamaService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ollama_service_creation() {
        let service = OllamaService::new();
        assert!(service.client.get("http://example.com").build().is_ok());
    }
}
