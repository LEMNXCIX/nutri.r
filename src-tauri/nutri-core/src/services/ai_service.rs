use crate::models::{
    ChatMessage, OllamaChatRequest, OllamaChatResponse, OllamaListResponse, OllamaModelInfo,
};
use crate::utils::{AppError, AppResult};
use reqwest::Client;

/// Service for interacting with Ollama API
pub struct OllamaService {
    client: Client,
}

impl OllamaService {
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

        // Step 2: Extract proteins
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

        // Try to parse as JSON directly
        let proteins: Vec<String> = match serde_json::from_str::<Vec<String>>(&content_text) {
            Ok(p) => p,
            Err(_) => {
                // Try to find JSON array in the response
                if let Some(start) = content_text.find('[') {
                    if let Some(end) = content_text.rfind(']') {
                        let json_str = &content_text[start..=end];
                        serde_json::from_str(json_str).unwrap_or_else(|_| Vec::new())
                    } else {
                        Vec::new()
                    }
                } else {
                    Vec::new()
                }
            }
        };

        log::info!("Extracted proteins: {:?}", proteins);

        // Step 3: Extract weekly structure
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
        log::info!("Raw struct text: {}", struct_text);

        let weekly_structure: Option<Vec<crate::models::WeeklyMealInfo>> =
            match serde_json::from_str(&struct_text) {
                Ok(s) => Some(s),
                Err(_) => {
                    if let Some(start) = struct_text.find('[') {
                        if let Some(end) = struct_text.rfind(']') {
                            let json_str = &struct_text[start..=end];
                            serde_json::from_str(json_str).ok()
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
            };

        log::info!(
            "Extracted structure items: {:?}",
            weekly_structure.as_ref().map(|v| v.len())
        );

        Ok((markdown, proteins, weekly_structure))
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
