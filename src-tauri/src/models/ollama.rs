use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct OllamaModelInfo {
    pub name: String,
    pub modified_at: String,
    pub size: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OllamaListResponse {
    pub models: Vec<OllamaModelInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OllamaChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub stream: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OllamaChatResponse {
    pub model: String,
    pub created_at: String,
    pub message: ChatMessage,
    pub done: bool,
    #[serde(default)]
    pub total_duration: Option<i64>,
    #[serde(default)]
    pub load_duration: Option<i64>,
    #[serde(default)]
    pub prompt_eval_count: Option<i32>,
    #[serde(default)]
    pub prompt_eval_duration: Option<i64>,
    #[serde(default)]
    pub eval_count: Option<i32>,
    #[serde(default)]
    pub eval_duration: Option<i64>,
}
