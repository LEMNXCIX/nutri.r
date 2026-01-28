use reqwest::Client;

pub async fn generate_plan(
    prompt: String,
    exclusion: String,
) -> Result<(String, Vec<String>), String> {
    let client = Client::new();
    let url = "http://127.0.0.1:8080/v1/chat/completions";

    let mut prompt_final = prompt;

    if !exclusion.is_empty() {
        prompt_final.push_str("\nRESTRICCIÓN: No uses en esta receta: ");
        prompt_final.push_str(&exclusion);
    }

    log::info!("Prompt final: {}", prompt_final);

    // PASO 1: Generación del Plan
    let response = client
        .post(url)
        .json(&serde_json::json!({
            "messages": [
                {"role": "system", "content": "Eres un nutricionista experto en dietas OMAD y Keto."},
                {"role": "user", "content": prompt_final}
            ],
            "stream": false,
            "temperature": 0.7
        }))
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;

    let json_gen: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    let markdown = json_gen["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("")
        .to_string();

    log::info!("Markdown generado correctamente");

    if markdown.is_empty() {
        return Err("Markdown is empty".to_string());
    }

    // PASO 2: Extracción de Proteínas
    let prompt_extract = format!(
        "Analiza el siguiente plan nutricional y extrae TODAS las fuentes de proteína (animal y vegetal).
Responde SOLAMENTE con un array JSON de strings.
EJEMPLO: [\"Pollo\", \"Lentejas\", \"Huevo\", \"Tofu\"]
NO añadas texto adicional, ni markdown, ni explicaciones.
PLAN:
{}",
        markdown
    );

    let response_extract = client
        .post(url)
        .json(&serde_json::json!({
            "messages": [
                {"role": "system", "content": "Eres un extractor de datos JSON preciso. Solo respondes con el array JSON."},
                {"role": "user", "content": prompt_extract}
            ],
            "stream": false
        }))
        .send()
        .await
        .map_err(|e| format!("Failed to send extract request: {}", e))?;

    let json_ext: serde_json::Value = response_extract
        .json()
        .await
        .map_err(|e| format!("Failed to parse JSON extract: {}", e))?;

    let content_text = json_ext["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("")
        .to_string();

    log::info!("Raw extract text: {}", content_text);

    // Intentar parsear como JSON directo usando la lógica existente
    let proteins: Vec<String> = match serde_json::from_str::<Vec<String>>(&content_text) {
        Ok(p) => p,
        Err(_) => {
            log::warn!("Could not parse specific JSON, trying regex fallback");
            let re = regex::Regex::new(r"\[.*?\]").map_err(|e| e.to_string())?;
            if let Some(mat) = re.find(&content_text) {
                serde_json::from_str::<Vec<String>>(mat.as_str())
                    .unwrap_or_else(|_| fallback_cleanup(&content_text))
            } else {
                fallback_cleanup(&content_text)
            }
        }
    };

    Ok((markdown, proteins))
}

fn fallback_cleanup(text: &str) -> Vec<String> {
    text.split(&[',', '\n'][..])
        .map(|s| s.trim())
        .map(|s| {
            s.trim_matches(|c| {
                c == '"' || c == '\'' || c == '[' || c == ']' || c == '*' || c == '-'
            })
        })
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty() && s.len() > 2) // Filter noise
        .collect()
}
