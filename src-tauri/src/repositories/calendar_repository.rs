use crate::models::{CalendarEntry, MealType};
use crate::utils::{AppError, AppResult};
use std::fs;
use std::path::PathBuf;

pub trait CalendarRepository {
    fn get_range(&self, start_date: &str, end_date: &str) -> AppResult<Vec<CalendarEntry>>;
    fn save(&self, entry: CalendarEntry) -> AppResult<()>;
    fn remove(&self, date: &str, meal_type: MealType) -> AppResult<()>;
    fn get_all(&self) -> AppResult<Vec<CalendarEntry>>;
    fn save_all(&self, entries: Vec<CalendarEntry>) -> AppResult<()>;
}

#[derive(Clone)]
pub struct FileCalendarRepository {
    data_dir: PathBuf,
}

impl FileCalendarRepository {
    pub fn new(data_dir: PathBuf) -> Self {
        Self { data_dir }
    }

    fn file_path(&self) -> PathBuf {
        self.data_dir.join("calendar.json")
    }

    fn read_all(&self) -> AppResult<Vec<CalendarEntry>> {
        let path = self.file_path();
        if !path.exists() {
            return Ok(Vec::new());
        }
        let content = fs::read_to_string(&path)
            .map_err(|e| AppError::Database(format!("Failed to read calendar: {}", e)))?;
        serde_json::from_str(&content)
            .map_err(|e| AppError::Serialization(format!("Failed to parse calendar: {}", e)))
    }

    fn write_all(&self, entries: Vec<CalendarEntry>) -> AppResult<()> {
        let path = self.file_path();
        let json = serde_json::to_string_pretty(&entries)
            .map_err(|e| AppError::Serialization(format!("Failed to serialize calendar: {}", e)))?;
        fs::write(&path, json)
            .map_err(|e| AppError::Database(format!("Failed to write calendar: {}", e)))
    }
}

impl CalendarRepository for FileCalendarRepository {
    fn get_range(&self, start_date: &str, end_date: &str) -> AppResult<Vec<CalendarEntry>> {
        let entries = self.read_all()?;
        Ok(entries
            .into_iter()
            .filter(|e| e.date >= start_date.to_string() && e.date <= end_date.to_string())
            .collect())
    }

    fn save(&self, entry: CalendarEntry) -> AppResult<()> {
        let mut entries = self.read_all()?;
        // Remove existing entry for same date/meal_type if exists
        entries.retain(|e| !(e.date == entry.date && e.meal_type == entry.meal_type));
        entries.push(entry);
        self.write_all(entries)
    }

    fn remove(&self, date: &str, meal_type: MealType) -> AppResult<()> {
        let mut entries = self.read_all()?;
        let initial_len = entries.len();
        entries.retain(|e| !(e.date == date && e.meal_type == meal_type));

        if entries.len() == initial_len {
            return Err(AppError::NotFound(format!(
                "No calendar entry found for {} {:?}",
                date, meal_type
            )));
        }

        self.write_all(entries)
    }

    fn get_all(&self) -> AppResult<Vec<CalendarEntry>> {
        self.read_all()
    }

    fn save_all(&self, entries: Vec<CalendarEntry>) -> AppResult<()> {
        self.write_all(entries)
    }
}
