use crate::models::AppBackup;
use crate::models::WaterRecord;
use crate::repositories::{
    CalendarRepository, ConfigRepository, IngredientRepository, MetadataRepository,
    PantryRepository, PlanRepository, TagRepository,
};
use crate::utils::error::AppResult;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub struct ImportExportService<
    P: PlanRepository,
    C: ConfigRepository,
    I: IngredientRepository,
    M: MetadataRepository,
    CL: CalendarRepository,
    T: TagRepository,
    PY: PantryRepository,
> {
    plan_repo: P,
    config_repo: C,
    ingredient_repo: I,
    metadata_repo: M,
    calendar_repo: CL,
    tag_repo: T,
    pantry_repo: PY,
    data_dir: PathBuf,
}

impl<
        P: PlanRepository,
        C: ConfigRepository,
        I: IngredientRepository,
        M: MetadataRepository,
        CL: CalendarRepository,
        T: TagRepository,
        PY: PantryRepository,
    > ImportExportService<P, C, I, M, CL, T, PY>
{
    pub fn new(
        plan_repo: P,
        config_repo: C,
        ingredient_repo: I,
        metadata_repo: M,
        calendar_repo: CL,
        tag_repo: T,
        pantry_repo: PY,
        data_dir: PathBuf,
    ) -> Self {
        Self {
            plan_repo,
            config_repo,
            ingredient_repo,
            metadata_repo,
            calendar_repo,
            tag_repo,
            pantry_repo,
            data_dir,
        }
    }

    pub fn create_backup(&self) -> AppResult<AppBackup> {
        let config = self.config_repo.get()?;
        let plans = self.plan_repo.get_all()?;

        let mut plan_details = Vec::new();
        for plan in &plans {
            // get_by_id devuelve AppResult<PlanDetail> (no Option)
            if let Ok(detail) = self.plan_repo.get_by_id(&plan.id) {
                plan_details.push(detail);
            }
        }

        let mut metadata = Vec::new();
        for plan in &plans {
            if let Ok(Some(meta)) = self.metadata_repo.get(&plan.id) {
                metadata.push(meta);
            }
        }

        let tags = self.tag_repo.get_all()?;
        let calendar = self.calendar_repo.get_all()?;
        let pantry = self.pantry_repo.get_all()?;
        let excluded_ingredients = self.ingredient_repo.get_excluded()?;

        let last_updated = if config.last_updated.is_empty() {
            chrono::Local::now().to_rfc3339()
        } else {
            config.last_updated.clone()
        };

        // Read water data
        let water_path = self.data_dir.join("water.json");
        let water: HashMap<String, WaterRecord> = if water_path.exists() {
            fs::read_to_string(&water_path)
                .ok()
                .and_then(|content| serde_json::from_str(&content).ok())
                .unwrap_or_default()
        } else {
            HashMap::new()
        };

        Ok(AppBackup {
            version: "1.0.0".to_string(),
            last_updated,
            config,
            plans,
            plan_details,
            metadata,
            tags,
            calendar,
            pantry,
            excluded_ingredients,
            water,
        })
    }

    pub fn restore_backup(&self, backup: AppBackup) -> AppResult<()> {
        // Restaurar cada componente
        self.config_repo.save(&backup.config)?;

        // Para planes, sobreescribimos el índice y guardamos cada detalle
        self.plan_repo.overwrite_index(backup.plans)?;
        for detail in backup.plan_details {
            self.plan_repo.save_detail(detail)?;
        }

        // Metadatos
        for meta in backup.metadata {
            self.metadata_repo.save(meta)?;
        }

        self.tag_repo.save_all(backup.tags)?;
        self.calendar_repo.save_all(backup.calendar)?;
        self.pantry_repo.save_all(backup.pantry)?;
        self.ingredient_repo
            .save_excluded(&backup.excluded_ingredients)?;

        // Restore water data directly to file
        let water_path = self.data_dir.join("water.json");
        if let Ok(content) = serde_json::to_string_pretty(&backup.water) {
            let _ = fs::write(water_path, content);
        }

        Ok(())
    }
}
