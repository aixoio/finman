use serde::{Deserialize, Serialize};
use tauri::State;

use crate::{
    database::{Item, ItemType},
    errors::{AppError, AppResult},
    AppState,
};

const MAX_MONEY_CENTS: i64 = 9_007_199_254_740_991;

fn normalize_money_cents(value: i64, field: &str, allow_zero: bool) -> AppResult<i64> {
    let value = value
        .checked_abs()
        .ok_or_else(|| AppError::Input(format!("{field} is outside the supported range")))?;

    if value > MAX_MONEY_CENTS {
        return Err(AppError::Input(format!(
            "{field} exceeds the maximum supported amount"
        )));
    }

    if !allow_zero && value == 0 {
        return Err(AppError::Input(format!("{field} cannot be zero")));
    }

    Ok(value)
}

fn validate_current_cents(value: i64) -> AppResult<i64> {
    if !(0..=MAX_MONEY_CENTS).contains(&value) {
        return Err(AppError::Input(
            "current amount is outside the supported range".into(),
        ));
    }

    Ok(value)
}

#[tauri::command]
pub async fn select_all_items_not_archived(state: State<'_, AppState>) -> AppResult<Vec<Item>> {
    state.database.select_all_items_not_archived().await
}

#[tauri::command]
pub async fn select_all_items_archived(state: State<'_, AppState>) -> AppResult<Vec<Item>> {
    state.database.select_all_items_archived().await
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
        return Err(AppError::Input("name cannot be empty".into()));
    }

    let comment = comment
        .map(|c| c.trim().to_string())
        .filter(|c| !c.is_empty());

    let target_cents = normalize_money_cents(target_cents, "target", false)?;
    let current_cents = normalize_money_cents(current_cents, "current amount", true)?;

    let uuid = state
        .database
        .insert_item(
            name,
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
    Comment {
        comment: Option<String>,
    },
    Archive {
        archived: bool,
    },
}

#[tauri::command]
pub async fn update_item_with_uuid(
    state: State<'_, AppState>,
    uuid: String,
    action: ItemUpdateAction,
) -> AppResult<()> {
    let Some(item) = state.database.select_item_by_uuid(&uuid).await? else {
        return Err(AppError::Input("uuid does not exist in database".into()));
    };

    let is_unarchive = matches!(&action, ItemUpdateAction::Archive { archived: false });
    if item.archived && !is_unarchive {
        return Err(AppError::Input(
            "archived items cannot be updated until they are unarchived".into(),
        ));
    }

    match action {
        ItemUpdateAction::CompleteGoal => {
            let target_cents = normalize_money_cents(item.target_cents, "target", false)?;

            state
                .database
                .update_item_amount_with_uuid(&uuid, target_cents)
                .await?;
        }
        ItemUpdateAction::SetExact { amount_cents } => {
            let amount_cents = normalize_money_cents(amount_cents, "amount", true)?;

            state
                .database
                .update_item_amount_with_uuid(&uuid, amount_cents)
                .await?;
        }
        ItemUpdateAction::Add { amount_cents } => {
            let amount_cents = normalize_money_cents(amount_cents, "amount", true)?;
            let current_cents = validate_current_cents(item.current_cents)?
                .checked_add(amount_cents)
                .ok_or_else(|| AppError::Input("current amount overflowed".into()))?;
            let current_cents = validate_current_cents(current_cents)?;

            state
                .database
                .update_item_amount_with_uuid(&uuid, current_cents)
                .await?;
        }
        ItemUpdateAction::Subtract { amount_cents } => {
            let amount_cents = normalize_money_cents(amount_cents, "amount", true)?;
            let current_cents = validate_current_cents(item.current_cents)?
                .checked_sub(amount_cents)
                .ok_or_else(|| AppError::Input("current amount underflowed".into()))?;
            let current_cents = validate_current_cents(current_cents)?;

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
                return Err(AppError::Input("name is empty".into()));
            }

            let target_cents = normalize_money_cents(target_cents, "target", false)?;
            let current_cents = normalize_money_cents(current_cents, "current amount", true)?;

            state
                .database
                .edit_item_with_uuid(&uuid, name, target_cents, current_cents)
                .await?;
        }
        ItemUpdateAction::Comment { comment } => {
            let comment = comment
                .map(|c| c.trim().to_string())
                .filter(|c| !c.is_empty());

            state
                .database
                .update_comment_with_uuid(&uuid, comment.as_deref())
                .await?;
        }
        ItemUpdateAction::Archive { archived } => {
            state
                .database
                .set_archived_state_with_uuid(&uuid, archived)
                .await?;
        }
    }

    Ok(())
}

#[tauri::command]
pub async fn delete_item_with_uuid(state: State<'_, AppState>, uuid: String) -> AppResult<()> {
    state.database.delete_item_with_uuid(&uuid).await
}
