use tauri::State;

use crate::{
    database::{Item, ItemType},
    errors::AppResult,
    AppState,
};

#[tauri::command]
pub async fn select_all_items_not_archived(state: State<'_, AppState>) -> AppResult<Vec<Item>> {
    state.database.select_all_items_not_archived().await
}

#[tauri::command]
pub async fn insert_item(
    state: State<'_, AppState>,
    name: String,
    comment: Option<String>,
    item_type: ItemType,
    target_cents: i64,
    current_cents: i64,
) -> AppResult<String> {
    let uuid = state
        .database
        .insert_item(
            &name,
            comment.as_deref(),
            item_type,
            target_cents,
            current_cents,
        )
        .await?;

    Ok(uuid)
}
