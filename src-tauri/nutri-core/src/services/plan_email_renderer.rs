use crate::models::{
    PlanEmailContext, PlanNutrition, RenderedEmail, StructuredPlan, StructuredRecipe,
};
use pulldown_cmark::{html, Options, Parser};

const ACCENT: &str = "#0df259";
const INK: &str = "#0a0a0a";
const CANVAS: &str = "#f3f4f6";
const PAPER: &str = "#ffffff";
const MUTED: &str = "#6b7280";
const SOFT: &str = "#e5e7eb";

/// Render a full HTML email for a saved plan using a shared visual language.
pub fn build_plan_email(
    context: &PlanEmailContext,
    nutrition: Option<&PlanNutrition>,
) -> RenderedEmail {
    let title = escape_html(&context.display_name);
    let reference = escape_html(&context.short_reference);
    let created_at = escape_html(
        context
            .created_at_label
            .as_deref()
            .unwrap_or("Sin fecha registrada"),
    );
    let nutrition_summary = nutrition.map(render_nutrition_summary).unwrap_or_default();
    let body = render_plan_body(context);

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="es">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>{title}</title>
  </head>
  <body style="margin:0;padding:0;background:{canvas};color:{ink};">
    <table role="presentation" cellpadding="0" cellspacing="0" width="100%" style="background:{canvas};margin:0;padding:0;width:100%;">
      <tr>
        <td align="center" style="padding:24px 12px;">
          <table role="presentation" cellpadding="0" cellspacing="0" width="100%" style="max-width:720px;width:100%;background:{paper};border:1px solid {ink};">
            <tr>
              <td style="height:12px;background:{accent};font-size:0;line-height:0;">&nbsp;</td>
            </tr>
            <tr>
              <td style="padding:32px 32px 24px 32px;">
                <span style="display:inline-block;background:{accent};border:1px solid {ink};padding:8px 12px;font-family:'Arial Black',Arial,sans-serif;font-size:10px;font-weight:900;letter-spacing:2px;text-transform:uppercase;color:{ink};">
                  PLAN GENERADO
                </span>
                <h1 style="margin:18px 0 0 0;font-family:'Arial Black',Arial,sans-serif;font-size:40px;line-height:0.95;font-weight:900;letter-spacing:-1.5px;text-transform:uppercase;color:{ink};">
                  {title}
                </h1>
                <p style="margin:16px 0 0 0;font-family:Arial,Helvetica,sans-serif;font-size:15px;line-height:1.7;color:#1f2937;">
                  Consulta el detalle completo del plan enviado desde Nutri-R, con una lectura más cercana al estilo editorial de la app.
                </p>
              </td>
            </tr>
            <tr>
              <td style="padding:0 32px 24px 32px;">
                <table role="presentation" cellpadding="0" cellspacing="0" width="100%" style="border-collapse:collapse;width:100%;">
                  <tr>
                    <td width="50%" valign="top" style="padding:16px;border:1px solid {ink};">
                      <div style="font-family:'Arial Black',Arial,sans-serif;font-size:10px;font-weight:900;letter-spacing:2px;text-transform:uppercase;color:{muted};margin-bottom:8px;">
                        REFERENCIA
                      </div>
                      <div style="font-family:'Arial Black',Arial,sans-serif;font-size:22px;font-weight:900;letter-spacing:-0.6px;color:{ink};">
                        Ref {reference}
                      </div>
                    </td>
                    <td width="16" style="font-size:0;line-height:0;">&nbsp;</td>
                    <td width="50%" valign="top" style="padding:16px;border:1px solid {ink};">
                      <div style="font-family:'Arial Black',Arial,sans-serif;font-size:10px;font-weight:900;letter-spacing:2px;text-transform:uppercase;color:{muted};margin-bottom:8px;">
                        FECHA
                      </div>
                      <div style="font-family:Arial,Helvetica,sans-serif;font-size:16px;line-height:1.6;color:{ink};font-weight:700;">
                        {created_at}
                      </div>
                    </td>
                  </tr>
                </table>
              </td>
            </tr>
            {nutrition_summary}
            <tr>
              <td style="padding:0 32px 32px 32px;">
                <table role="presentation" cellpadding="0" cellspacing="0" width="100%" style="border-collapse:collapse;width:100%;border:1px solid {ink};">
                  <tr>
                    <td style="padding:14px 18px;background:{ink};font-family:'Arial Black',Arial,sans-serif;font-size:10px;font-weight:900;letter-spacing:2px;text-transform:uppercase;color:{paper};">
                      PLAN COMPLETO
                    </td>
                  </tr>
                  <tr>
                    <td style="padding:24px;background:{paper};">
                      {body}
                    </td>
                  </tr>
                </table>
              </td>
            </tr>
            <tr>
              <td style="padding:0 32px 32px 32px;">
                <table role="presentation" cellpadding="0" cellspacing="0" width="100%" style="border-collapse:collapse;width:100%;border-top:1px solid {soft};">
                  <tr>
                    <td style="padding-top:18px;font-family:'Arial Black',Arial,sans-serif;font-size:10px;font-weight:900;letter-spacing:2px;text-transform:uppercase;color:{muted};">
                      ENVIADO DESDE NUTRI-R
                    </td>
                  </tr>
                  <tr>
                    <td style="padding-top:8px;font-family:Arial,Helvetica,sans-serif;font-size:13px;line-height:1.7;color:{muted};">
                      Este correo mantiene el detalle completo del plan para que puedas consultarlo fuera de la app con una presentación más clara y consistente.
                    </td>
                  </tr>
                </table>
              </td>
            </tr>
          </table>
        </td>
      </tr>
    </table>
  </body>
</html>"#,
        accent = ACCENT,
        body = body,
        canvas = CANVAS,
        created_at = created_at,
        ink = INK,
        muted = MUTED,
        nutrition_summary = nutrition_summary,
        paper = PAPER,
        reference = reference,
        soft = SOFT,
        title = title,
    );

    RenderedEmail {
        subject: format!("Tu Plan Nutricional - {}", context.display_name),
        html,
    }
}

fn render_nutrition_summary(nutrition: &PlanNutrition) -> String {
    let metrics = [
        (
            "CALORIAS",
            format!("{} kcal", format_metric_value(nutrition.total_calories)),
        ),
        (
            "PROTEINA",
            format!("{} g", format_metric_value(nutrition.total_protein)),
        ),
        (
            "CARBOS",
            format!("{} g", format_metric_value(nutrition.total_carbs)),
        ),
        (
            "GRASA",
            format!("{} g", format_metric_value(nutrition.total_fat)),
        ),
    ]
    .into_iter()
    .map(render_metric_card)
    .collect::<String>();

    format!(
        r#"<tr>
  <td style="padding:0 32px 24px 32px;">
    <table role="presentation" cellpadding="0" cellspacing="0" width="100%" style="border-collapse:collapse;width:100%;">
      <tr>
        <td colspan="4" style="padding:0 0 12px 0;font-family:'Arial Black',Arial,sans-serif;font-size:10px;font-weight:900;letter-spacing:2px;text-transform:uppercase;color:{muted};">
          RESUMEN NUTRICIONAL
        </td>
      </tr>
      <tr>
        {metrics}
      </tr>
    </table>
  </td>
</tr>"#,
        metrics = metrics,
        muted = MUTED,
    )
}

fn render_metric_card((label, value): (&str, String)) -> String {
    format!(
        r#"<td width="25%" valign="top" style="padding:16px;border:1px solid {ink};">
  <div style="font-family:'Arial Black',Arial,sans-serif;font-size:10px;font-weight:900;letter-spacing:2px;text-transform:uppercase;color:{muted};margin-bottom:8px;">
    {label}
  </div>
  <div style="font-family:'Arial Black',Arial,sans-serif;font-size:24px;font-weight:900;letter-spacing:-0.8px;color:{ink};">
    {value}
  </div>
</td>"#,
        ink = INK,
        label = escape_html(label),
        muted = MUTED,
        value = escape_html(&value),
    )
}

fn render_plan_body(context: &PlanEmailContext) -> String {
    if let Some(structured_plan) = context.plan_detail.structured_plan.as_ref() {
        render_structured_plan(structured_plan)
    } else {
        render_markdown_fragment(&context.plan_detail.markdown_content)
    }
}

fn render_structured_plan(plan: &StructuredPlan) -> String {
    let mut sections = Vec::new();

    if let Some(instructions) = plan
        .instructions
        .as_deref()
        .map(str::trim)
        .filter(|text| !text.is_empty())
    {
        sections.push(format!(
            r#"<table role="presentation" cellpadding="0" cellspacing="0" width="100%" style="border-collapse:collapse;width:100%;margin:0 0 28px 0;border:1px solid {soft};background:#fafafa;">
  <tr>
    <td style="padding:16px 18px 8px 18px;font-family:'Arial Black',Arial,sans-serif;font-size:10px;font-weight:900;letter-spacing:2px;text-transform:uppercase;color:{muted};">
      INTRODUCCION
    </td>
  </tr>
  <tr>
    <td style="padding:0 18px 18px 18px;">
      {content}
    </td>
  </tr>
</table>"#,
            content = render_markdown_fragment(instructions),
            muted = MUTED,
            soft = SOFT,
        ));
    }

    for day in &plan.days {
        let recipes = day
            .recipes
            .iter()
            .map(render_recipe_block)
            .collect::<String>();

        sections.push(format!(
            r#"<table role="presentation" cellpadding="0" cellspacing="0" width="100%" style="border-collapse:collapse;width:100%;margin:0 0 28px 0;">
  <tr>
    <td style="padding:0 0 14px 0;border-bottom:1px solid {ink};font-family:'Arial Black',Arial,sans-serif;font-size:14px;font-weight:900;letter-spacing:2px;text-transform:uppercase;color:{ink};">
      {label}
    </td>
  </tr>
  <tr>
    <td style="padding-top:18px;">
      {recipes}
    </td>
  </tr>
</table>"#,
            ink = INK,
            label = escape_html(&day.label),
            recipes = recipes,
        ));
    }

    sections.join("")
}

fn render_recipe_block(recipe: &StructuredRecipe) -> String {
    let ingredients = if recipe.ingredients.is_empty() {
        String::new()
    } else {
        format!(
            r#"<div style="margin-top:16px;">
  <div style="font-family:'Arial Black',Arial,sans-serif;font-size:10px;font-weight:900;letter-spacing:2px;text-transform:uppercase;color:{muted};margin-bottom:8px;">
    INGREDIENTES
  </div>
  <ul style="margin:0;padding-left:20px;font-family:Arial,Helvetica,sans-serif;font-size:14px;line-height:1.8;color:#1f2937;">
    {items}
  </ul>
</div>"#,
            items = recipe
                .ingredients
                .iter()
                .map(|ingredient| {
                    format!(
                        r#"<li style="margin:0 0 8px 0;">{}</li>"#,
                        escape_html(ingredient)
                    )
                })
                .collect::<String>(),
            muted = MUTED,
        )
    };

    let instructions = if recipe.instructions.is_empty() {
        String::new()
    } else {
        format!(
            r#"<div style="margin-top:16px;">
  <div style="font-family:'Arial Black',Arial,sans-serif;font-size:10px;font-weight:900;letter-spacing:2px;text-transform:uppercase;color:{muted};margin-bottom:8px;">
    PREPARACION
  </div>
  <ol style="margin:0;padding-left:22px;font-family:Arial,Helvetica,sans-serif;font-size:14px;line-height:1.8;color:#1f2937;">
    {items}
  </ol>
</div>"#,
            items = recipe
                .instructions
                .iter()
                .map(|instruction| {
                    format!(
                        r#"<li style="margin:0 0 10px 0;">{}</li>"#,
                        escape_html(instruction)
                    )
                })
                .collect::<String>(),
            muted = MUTED,
        )
    };

    let notes = recipe
        .notes
        .as_deref()
        .map(str::trim)
        .filter(|text| !text.is_empty())
        .map(|notes| {
            format!(
                r#"<div style="margin-top:16px;padding:14px 16px;background:#f9fafb;border-left:4px solid {accent};font-family:Arial,Helvetica,sans-serif;font-size:14px;line-height:1.7;color:#1f2937;">
  <strong style="font-family:'Arial Black',Arial,sans-serif;font-size:10px;letter-spacing:2px;text-transform:uppercase;color:{ink};display:block;margin-bottom:8px;">
    NOTAS
  </strong>
  {notes}
</div>"#,
                accent = ACCENT,
                ink = INK,
                notes = escape_html(notes),
            )
        })
        .unwrap_or_default();

    format!(
        r#"<table role="presentation" cellpadding="0" cellspacing="0" width="100%" style="border-collapse:collapse;width:100%;margin:0 0 16px 0;border:1px solid {ink};">
  <tr>
    <td style="padding:16px 18px;">
      <div style="font-family:'Arial Black',Arial,sans-serif;font-size:10px;font-weight:900;letter-spacing:2px;text-transform:uppercase;color:{muted};margin-bottom:10px;">
        {meal_type}
      </div>
      <div style="font-family:'Arial Black',Arial,sans-serif;font-size:24px;font-weight:900;letter-spacing:-0.8px;line-height:1.1;color:{ink};">
        {name}
      </div>
      {ingredients}
      {instructions}
      {notes}
    </td>
  </tr>
</table>"#,
        ingredients = ingredients,
        ink = INK,
        instructions = instructions,
        meal_type = escape_html(recipe.meal_type.display_name()),
        muted = MUTED,
        name = escape_html(&recipe.name),
        notes = notes,
    )
}

fn render_markdown_fragment(markdown: &str) -> String {
    let trimmed = markdown.trim();
    if trimmed.is_empty() {
        return String::new();
    }

    let safe_markdown = escape_html(trimmed);
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);

    let parser = Parser::new_ext(&safe_markdown, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    apply_inline_markdown_styles(html_output)
}

fn apply_inline_markdown_styles(mut html_output: String) -> String {
    let replacements = [
        (
            "<h1>",
            "<h1 style=\"margin:0 0 18px 0;font-family:'Arial Black',Arial,sans-serif;font-size:30px;line-height:1;font-weight:900;letter-spacing:-1px;text-transform:uppercase;color:#0a0a0a;\">",
        ),
        (
            "<h2>",
            "<h2 style=\"margin:28px 0 14px 0;padding-top:22px;border-top:1px solid #0a0a0a;font-family:'Arial Black',Arial,sans-serif;font-size:18px;line-height:1.1;font-weight:900;letter-spacing:1.6px;text-transform:uppercase;color:#0a0a0a;\">",
        ),
        (
            "<h3>",
            "<h3 style=\"margin:22px 0 12px 0;font-family:'Arial Black',Arial,sans-serif;font-size:14px;line-height:1.3;font-weight:900;letter-spacing:1.2px;text-transform:uppercase;color:#111827;\">",
        ),
        (
            "<p>",
            "<p style=\"margin:0 0 16px 0;font-family:Arial,Helvetica,sans-serif;font-size:15px;line-height:1.7;color:#1f2937;\">",
        ),
        (
            "<ul>",
            "<ul style=\"margin:0 0 20px 0;padding-left:22px;font-family:Arial,Helvetica,sans-serif;font-size:14px;line-height:1.8;color:#111827;\">",
        ),
        (
            "<ol>",
            "<ol style=\"margin:0 0 20px 0;padding-left:24px;font-family:Arial,Helvetica,sans-serif;font-size:14px;line-height:1.8;color:#111827;\">",
        ),
        ("<li>", "<li style=\"margin:0 0 10px 0;\">"),
        (
            "<blockquote>",
            "<blockquote style=\"margin:0 0 20px 0;padding:16px 18px;border-left:4px solid #0df259;background:#f3f4f6;font-family:Arial,Helvetica,sans-serif;font-size:14px;line-height:1.7;color:#111827;\">",
        ),
        (
            "<pre>",
            "<pre style=\"margin:0 0 20px 0;padding:16px;background:#111827;color:#f9fafb;border:1px solid #111827;overflow:auto;white-space:pre-wrap;font-size:13px;line-height:1.6;\">",
        ),
        (
            "<code>",
            "<code style=\"font-family:'SFMono-Regular',Consolas,'Liberation Mono',monospace;background:#f3f4f6;padding:2px 6px;border:1px solid #d1d5db;font-size:13px;\">",
        ),
        (
            "<table>",
            "<table cellpadding=\"0\" cellspacing=\"0\" width=\"100%\" style=\"border-collapse:collapse;width:100%;margin:24px 0;border:1px solid #111827;\">",
        ),
        (
            "<th>",
            "<th style=\"background:#111827;color:#ffffff;padding:12px 14px;border:1px solid #111827;text-align:left;font-family:'Arial Black',Arial,sans-serif;font-size:11px;letter-spacing:1.4px;text-transform:uppercase;\">",
        ),
        (
            "<td>",
            "<td style=\"background:#ffffff;color:#111827;padding:12px 14px;border:1px solid #d1d5db;font-family:Arial,Helvetica,sans-serif;font-size:14px;line-height:1.6;vertical-align:top;\">",
        ),
        ("<strong>", "<strong style=\"font-weight:800;color:#0a0a0a;\">"),
        ("<em>", "<em style=\"font-style:italic;\">"),
        (
            "<a href=\"",
            "<a style=\"color:#0a0a0a;font-weight:700;text-decoration:underline;\" href=\"",
        ),
        (
            "<hr />",
            "<hr style=\"border:none;border-top:1px solid #d1d5db;margin:24px 0;\" />",
        ),
        (
            "<hr>",
            "<hr style=\"border:none;border-top:1px solid #d1d5db;margin:24px 0;\">",
        ),
    ];

    for (from, to) in replacements {
        html_output = html_output.replace(from, to);
    }

    html_output
}

fn format_metric_value(value: f32) -> String {
    format!("{:.0}", value)
}

fn escape_html(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{
        MealType, PlanDetail, PlanEmailContext, PlanIndex, PlanMetadata, PlanNutrition,
        StructuredDay, StructuredPlan, StructuredRecipe,
    };

    #[test]
    fn build_plan_email_should_render_structured_summary_and_body() {
        let rendered = build_plan_email(
            &sample_context(Some("Plan Fuerza".to_string()), true),
            Some(&sample_nutrition()),
        );

        assert!(rendered.subject.contains("Plan Fuerza"));
        assert!(rendered.html.contains("PLAN GENERADO"));
        assert!(rendered.html.contains("#0df259"));
        assert!(rendered.html.contains("Lunes"));
        assert!(rendered.html.contains("Desayuno"));
        assert!(rendered.html.contains("RESUMEN NUTRICIONAL"));
    }

    #[test]
    fn build_plan_email_should_style_markdown_fallback_without_raw_heading_text() {
        let rendered = build_plan_email(
            &sample_context(Some("Plan Markdown".to_string()), false),
            None,
        );

        assert!(rendered.html.contains("<h1 style="));
        assert!(rendered.html.contains("Plan semanal"));
        assert!(!rendered.html.contains("# Plan semanal"));
        assert!(!rendered.html.contains("RESUMEN NUTRICIONAL"));
    }

    #[test]
    fn build_plan_email_should_hide_metrics_when_nutrition_is_missing() {
        let rendered =
            build_plan_email(&sample_context(Some("Plan Limpio".to_string()), true), None);

        assert!(!rendered.html.contains("RESUMEN NUTRICIONAL"));
        assert!(!rendered.html.contains("CALORIAS"));
    }

    fn sample_context(display_name: Option<String>, structured: bool) -> PlanEmailContext {
        let structured_plan = StructuredPlan {
            title: "Plan de Potencia".to_string(),
            instructions: Some("## Objetivo\nPrioriza preparaciones simples.".to_string()),
            days: vec![StructuredDay {
                day_id: "day-0".to_string(),
                day_index: 0,
                label: "Lunes".to_string(),
                recipes: vec![
                    StructuredRecipe {
                        recipe_id: "breakfast-0".to_string(),
                        meal_type: MealType::Breakfast,
                        name: "Avena Proteica".to_string(),
                        ingredients: vec!["Avena".to_string(), "Yogur griego".to_string()],
                        instructions: vec![
                            "Mezcla todo en un bowl.".to_string(),
                            "Sirve frio.".to_string(),
                        ],
                        notes: Some("Agrega canela al final.".to_string()),
                    },
                    StructuredRecipe {
                        recipe_id: "lunch-0".to_string(),
                        meal_type: MealType::Lunch,
                        name: "Pollo con arroz".to_string(),
                        ingredients: vec!["Pechuga".to_string(), "Arroz".to_string()],
                        instructions: vec!["Cocina y sirve.".to_string()],
                        notes: None,
                    },
                ],
            }],
        };

        PlanEmailContext {
            plan_id: "plan-001".to_string(),
            short_reference: "PLAN-0".chars().take(6).collect(),
            display_name: display_name.unwrap_or_else(|| "Plan Base".to_string()),
            created_at_label: Some("25 mar 2026 · 7:30 a. m.".to_string()),
            plan_detail: PlanDetail {
                id: "plan-001".to_string(),
                markdown_content: "# Plan semanal\n\n## Lunes\n\n- Avena\n- Pollo".to_string(),
                structured_plan: structured.then_some(structured_plan),
            },
            plan_index: PlanIndex {
                id: "plan-001".to_string(),
                fecha: "2026-03-25 07:30:00".to_string(),
                created_at: Some("2026-03-25T12:30:00Z".to_string()),
                display_name: None,
                proteinas: vec!["Pollo".to_string()],
                enviado: false,
                is_favorite: false,
                rating: None,
                weekly_structure: None,
            },
            metadata: Some(PlanMetadata {
                plan_id: "plan-001".to_string(),
                display_name: Some("Plan Fuerza".to_string()),
                is_favorite: false,
                rating: None,
                notes: String::new(),
                tags: vec![],
            }),
        }
    }

    fn sample_nutrition() -> PlanNutrition {
        PlanNutrition {
            plan_id: "plan-001".to_string(),
            total_calories: 2134.0,
            total_protein: 163.0,
            total_carbs: 195.0,
            total_fat: 72.0,
            breakdown_by_item: Default::default(),
        }
    }
}
