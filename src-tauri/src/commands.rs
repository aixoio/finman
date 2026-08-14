use serde::{Deserialize, Serialize};
use tauri::State;

use crate::{
    database::{Item, ItemType},
    errors::{AppError, AppResult},
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
    let name = name.trim();
    if name.is_empty() {
        return Err(AppError::InputError("name cannot be empty".into()));
    }

    let comment = comment
        .map(|c| c.trim().to_string())
        .filter(|c| c.is_empty());

    let target_cents = target_cents.abs();
    if target_cents == 0 {
        return Err(AppError::InputError("target cannot be zero".into()));
    }

    let current_cents = current_cents.abs();

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

#[tauri::command]
pub async fn fetch_item_with_uuid(
    state: State<'_, AppState>,
    uuid: String,
) -> AppResult<Option<Item>> {
    state.database.select_item_by_uuid(&uuid).await
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "data")]
pub enum ItemUpdateAction {
    CompleteGoal,
    SetExact { amount_cents: i64 },
    Add { amount_cents: i64 },
    Subtract { amount_cents: i64 },
}
