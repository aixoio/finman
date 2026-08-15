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
    SetExact {
        amount_cents: i64,
    },
    Add {
        amount_cents: i64,
    },
    Subtract {
        amount_cents: i64,
    },
    Edit {
        name: String,
        target_cents: i64,
        current_cents: i64,
    },
}

#[tauri::command]
pub async fn update_item_with_uuid(
    state: State<'_, AppState>,
    uuid: String,
    action: ItemUpdateAction,
) -> AppResult<()> {
    match action {
        ItemUpdateAction::CompleteGoal => {
            let Some(item) = state.database.select_item_by_uuid(&uuid).await? else {
                return Err(AppError::InputError(
                    "uuid does not exist in database".into(),
                ));
            };

            state
                .database
                .update_item_amount_with_uuid(&uuid, item.target_cents)
                .await?;
        }
        ItemUpdateAction::SetExact { amount_cents } => {
            state
                .database
                .update_item_amount_with_uuid(&uuid, amount_cents)
                .await?;
        }
        ItemUpdateAction::Add { amount_cents } => {
            let amount_cents = amount_cents.abs();

            let Some(item) = state.database.select_item_by_uuid(&uuid).await? else {
                return Err(AppError::InputError(
                    "uuid does not exist in database".into(),
                ));
            };

            let current_cents = item.current_cents + amount_cents;

            state
                .database
                .update_item_amount_with_uuid(&uuid, current_cents)
                .await?;
        }
        ItemUpdateAction::Subtract { amount_cents } => {
            let amount_cents = amount_cents.abs();

            let Some(item) = state.database.select_item_by_uuid(&uuid).await? else {
                return Err(AppError::InputError(
                    "uuid does not exist in database".into(),
                ));
            };

            let current_cents = item.current_cents - amount_cents;

            state
                .database
                .update_item_amount_with_uuid(&uuid, current_cents)
                .await?;
        }
        ItemUpdateAction::Edit {
            name,
            target_cents,
            current_cents,
        } => {
            let name = name.trim();
            if name.is_empty() {
                return Err(AppError::InputError("name is empty".into()));
            }

            let target_cents = target_cents.abs();

            state
                .database
                .edit_item_with_uuid(&uuid, name, target_cents, current_cents)
                .await?;
        }
    }

    Ok(())
}
