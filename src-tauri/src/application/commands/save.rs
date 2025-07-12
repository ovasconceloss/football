use tauri::State;
use sqlx::SqlitePool;
use crate::{core::save::model::Save, infrastructure::save::repository::SaveRepository};

#[tauri::command]
pub async fn new_save(name: String, database: State<'_, SqlitePool>) -> Result<(), String> {
  let mut new_save = Save::new(name);
  let save_repository = SaveRepository::new(database.inner().clone());

  if !new_save.is_name_valid() {
    return Err("The save name is invalid.".to_string());
  }

  let _ = save_repository.insert(&mut new_save)
    .await
    .map_err(|_e| format!("An error occurred while saving the game."));

  Ok(())
}