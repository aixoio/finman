use tauri::State;

use crate::{database::Item, errors::AppResult, AppState};

#[tauri::command]
pub async fn select_all_items_not_archived(state: State<'_, AppState>) -> AppResult<Vec<Item>> {
    state.database.select_all_items_not_archived().await
}
