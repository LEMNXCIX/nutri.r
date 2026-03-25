use super::calendar::MealType;
use chrono::{DateTime, Datelike, Utc};
use serde::{Deserialize, Deserializer, Serialize};

fn deserialize_meal_type<'de, D>(deserializer: D) -> Result<MealType, D::Error>
where
    D: Deserializer<'de>,
{
    let raw = String::deserialize(deserializer)?;
    MealType::from_label(&raw).ok_or_else(|| {
        serde::de::Error::custom(format!("Unsupported meal type value: {}", raw))
    })
}

fn deserialize_vec_or_string<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = serde_json::Value::deserialize(deserializer)?;
    match value {
        serde_json::Value::Null => Ok(Vec::new()),
        serde_json::Value::String(text) => Ok(text
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(ToString::to_string)
            .collect()),
        serde_json::Value::Array(items) => Ok(items
            .into_iter()
            .filter_map(|item| item.as_str().map(ToString::to_string))
            .collect()),
        other => Err(serde::de::Error::custom(format!(
            "Expected string or array for recipe content, got {}",
            other
        ))),
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WeeklyMealInfo {
    #[serde(alias = "day_index")]
    pub day_index: u8,
    #[serde(alias = "meal_type")]
    pub meal_type: String,
    pub description: Option<String>,
    #[serde(default)]
    pub day_id: Option<String>,
    #[serde(default)]
    pub recipe_id: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StructuredPlan {
    #[serde(default, alias = "titulo")]
    pub title: String,
    #[serde(default, alias = "instrucciones")]
    pub instructions: Option<String>,
    #[serde(default, alias = "dias")]
    pub days: Vec<StructuredDay>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StructuredDay {
    #[serde(default)]
    pub day_id: String,
    #[serde(default)]
    pub day_index: u8,
    #[serde(default, alias = "dia")]
    pub label: String,
    #[serde(default, alias = "comidas")]
    pub recipes: Vec<StructuredRecipe>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StructuredRecipe {
    #[serde(default)]
    pub recipe_id: String,
    #[serde(default, alias = "tipo", deserialize_with = "deserialize_meal_type")]
    pub meal_type: MealType,
    #[serde(default, alias = "nombre")]
    pub name: String,
    #[serde(default, alias = "ingredientes")]
    pub ingredients: Vec<String>,
    #[serde(default, alias = "instrucciones", deserialize_with = "deserialize_vec_or_string")]
    pub instructions: Vec<String>,
    #[serde(default)]
    pub notes: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RecipeSuggestion {
    pub plan_id: String,
    pub day_id: String,
    pub recipe_id: String,
    pub original_recipe: StructuredRecipe,
    pub suggested_recipe: StructuredRecipe,
}

impl StructuredPlan {
    pub fn normalized(mut self) -> Self {
        if self.title.trim().is_empty() {
            self.title = "Plan Nutricional".to_string();
        }

        for (day_position, day) in self.days.iter_mut().enumerate() {
            if day.day_id.trim().is_empty() {
                day.day_id = format!("day-{}", day_position);
            }

            if day.label.trim().is_empty() {
                day.label = match day.day_index {
                    0 => "Lunes".to_string(),
                    1 => "Martes".to_string(),
                    2 => "Miércoles".to_string(),
                    3 => "Jueves".to_string(),
                    4 => "Viernes".to_string(),
                    5 => "Sábado".to_string(),
                    6 => "Domingo".to_string(),
                    _ => format!("Día {}", day_position + 1),
                };
            }

            if day.day_index == 0 && day_position > 0 {
                day.day_index = day_position as u8;
            }

            for (recipe_position, recipe) in day.recipes.iter_mut().enumerate() {
                if recipe.recipe_id.trim().is_empty() {
                    recipe.recipe_id = format!(
                        "{}-{}-{}",
                        day.day_id,
                        recipe.meal_type.key(),
                        recipe_position
                    );
                }
            }
        }

        self.days.sort_by_key(|day| day.day_index);
        self
    }

    pub fn to_weekly_structure(&self) -> Vec<WeeklyMealInfo> {
        self.days
            .iter()
            .flat_map(|day| {
                day.recipes.iter().map(move |recipe| WeeklyMealInfo {
                    day_index: day.day_index,
                    meal_type: recipe.meal_type.key().to_string(),
                    description: Some(recipe.name.clone()),
                    day_id: Some(day.day_id.clone()),
                    recipe_id: Some(recipe.recipe_id.clone()),
                })
            })
            .collect()
    }

    pub fn to_markdown(&self) -> String {
        let mut lines = Vec::new();
        lines.push(format!("# {}", self.title));

        if let Some(instructions) = self.instructions.as_ref().filter(|text| !text.trim().is_empty())
        {
            lines.push(String::new());
            lines.push(instructions.trim().to_string());
        }

        for day in &self.days {
            lines.push(String::new());
            lines.push(format!("## {}", day.label));

            for recipe in &day.recipes {
                lines.push(String::new());
                lines.push(format!(
                    "### {}: {}",
                    recipe.meal_type.display_name(),
                    recipe.name
                ));

                if !recipe.ingredients.is_empty() {
                    lines.push(String::new());
                    lines.push("**Ingredientes**".to_string());
                    for ingredient in &recipe.ingredients {
                        lines.push(format!("- {}", ingredient));
                    }
                }

                if !recipe.instructions.is_empty() {
                    lines.push(String::new());
                    lines.push("**Instrucciones**".to_string());
                    for (step_index, instruction) in recipe.instructions.iter().enumerate() {
                        lines.push(format!("{}. {}", step_index + 1, instruction));
                    }
                }

                if let Some(notes) = recipe.notes.as_ref().filter(|text| !text.trim().is_empty()) {
                    lines.push(String::new());
                    lines.push(format!("**Notas:** {}", notes.trim()));
                }
            }
        }

        lines.join("\n")
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlanIndex {
    pub id: String,
    #[serde(default, alias = "date")]
    pub fecha: String,
    #[serde(default, alias = "created_at")]
    pub created_at: Option<String>,
    #[serde(default, alias = "display_name")]
    pub display_name: Option<String>,
    #[serde(alias = "proteins", alias = "protein_list")]
    pub proteinas: Vec<String>,
    #[serde(alias = "sent", alias = "is_sent")]
    pub enviado: bool,
    #[serde(default, alias = "is_favorite")]
    pub is_favorite: bool,
    #[serde(default)]
    pub rating: Option<u8>,
    #[serde(default)]
    pub weekly_structure: Option<Vec<WeeklyMealInfo>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlanDetail {
    pub id: String,
    pub markdown_content: String,
    #[serde(default)]
    pub structured_plan: Option<StructuredPlan>,
}

impl StructuredPlan {
    pub fn preferred_title(&self) -> Option<String> {
        let title = self.title.trim();
        if title.is_empty() || is_generic_plan_title(title) {
            None
        } else {
            Some(title.to_string())
        }
    }
}

pub fn is_generic_plan_title(title: &str) -> bool {
    let normalized = title.trim().to_lowercase();
    normalized.is_empty()
        || normalized == "plan nutricional"
        || normalized == "plan semanal"
        || normalized == "plan alimenticio"
        || normalized == "plan de comidas"
        || normalized.starts_with("plan nutricional #")
}

pub fn derive_plan_display_name(
    structured_plan: Option<&StructuredPlan>,
    created_at: Option<&DateTime<Utc>>,
    proteins: &[String],
    id: &str,
) -> String {
    if let Some(title) = structured_plan.and_then(StructuredPlan::preferred_title) {
        return title;
    }

    let protein = proteins
        .iter()
        .find_map(|protein| {
            let trimmed = protein.trim();
            (!trimmed.is_empty()).then(|| trimmed.to_string())
        });

    match (created_at, protein) {
        (Some(created_at), Some(protein)) => {
            format!("Plan del {} · {}", short_spanish_date(created_at), protein)
        }
        (Some(created_at), None) => format!("Plan del {}", short_spanish_date(created_at)),
        (None, Some(protein)) => format!("Plan · {}", protein),
        (None, None) => format!("Plan {}", id.chars().take(6).collect::<String>()),
    }
}

fn short_spanish_date(created_at: &DateTime<Utc>) -> String {
    let month = match created_at.month() {
        1 => "ene",
        2 => "feb",
        3 => "mar",
        4 => "abr",
        5 => "may",
        6 => "jun",
        7 => "jul",
        8 => "ago",
        9 => "sep",
        10 => "oct",
        11 => "nov",
        12 => "dic",
        _ => "mes",
    };

    format!("{} {}", created_at.day(), month)
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum VariationType {
    Vegan,
    Keto,
    GlutenFree,
    LowCarb,
    HighProtein,
}

impl std::fmt::Display for VariationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VariationType::Vegan => write!(f, "Vegano"),
            VariationType::Keto => write!(f, "Keto"),
            VariationType::GlutenFree => write!(f, "Sin Gluten"),
            VariationType::LowCarb => write!(f, "Bajo en Carbohidratos"),
            VariationType::HighProtein => write!(f, "Alto en Proteínas"),
        }
    }
}
