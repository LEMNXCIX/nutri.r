use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AppConfig {
    #[serde(default, alias = "smtp_host")]
    pub smtp_host: String,
    #[serde(
        default,
        alias = "smtp_port",
        deserialize_with = "deserialize_smtp_port"
    )]
    pub smtp_port: u16,
    #[serde(default, alias = "smtp_user")]
    pub smtp_user: String,
    #[serde(default, alias = "smtp_password")]
    pub smtp_password: String,
    #[serde(default, alias = "smtp_to")]
    pub smtp_to: String,
    #[serde(alias = "prompt_maestro")]
    pub prompt_maestro: String,
    #[serde(alias = "ollama_model")]
    pub ollama_model: String,
    #[serde(alias = "ollama_url")]
    pub ollama_url: String,
    #[serde(alias = "usda_api_key")]
    pub usda_api_key: String,
    #[serde(default, alias = "sync_server_url")]
    pub sync_server_url: String,
    #[serde(default, alias = "last_updated")]
    pub last_updated: String,
}

fn deserialize_smtp_port<'de, D>(deserializer: D) -> Result<u16, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::{self, Visitor};
    use std::fmt;

    struct SmtpPortVisitor;

    impl<'de> Visitor<'de> for SmtpPortVisitor {
        type Value = u16;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a number or a string")
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value as u16)
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value as u16)
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if value.is_empty() {
                Ok(0)
            } else {
                value.parse::<u16>().map_err(de::Error::custom)
            }
        }
    }

    deserializer.deserialize_any(SmtpPortVisitor)
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            smtp_host: String::new(),
            smtp_port: 587,
            smtp_user: String::new(),
            smtp_password: String::new(),
            smtp_to: String::new(),
            prompt_maestro:
                "Eres un chef experto en nutrición. Genera un plan nutricional semanal balanceado."
                    .to_string(),
            ollama_model: "llama3.2".to_string(),
            ollama_url: "http://127.0.0.1:11434".to_string(),
            usda_api_key: String::new(),
            sync_server_url: String::new(),
            last_updated: String::new(),
        }
    }
}
