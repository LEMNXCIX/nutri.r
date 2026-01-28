use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppConfig {
    pub prompt_maestro: String,
    pub smtp_user: String,
    pub smtp_password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlanIndex {
    pub id: String,
    pub fecha: String,
    pub proteinas: Vec<String>,
    pub enviado: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlanDetail {
    pub markdown_content: String,
}
