use std::str::FromStr;

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    Row, SqlitePool,
};
use uuid::Uuid;

use crate::errors::{AppError, AppResult};

pub const DATABASE_FILENAME: &str = "finman_database.sqlite3";

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum ItemType {
    Savings,
    SelfLoan,
    ExternalLoan,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub uuid: String,
    pub name: String,
    pub comment: Option<String>,
    pub item_type: ItemType,
    pub target_cents: i64,
    pub current_cents: i64,
    pub archived: bool,
    pub created_at: String,
    pub updated_at: String,
}

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn build(filename: &str) -> Result<Self, sqlx::Error> {
        let path = format!("sqlite:{filename}");

        let options = SqliteConnectOptions::from_str(&path)?.create_if_missing(true);
        let pool = SqlitePoolOptions::new().connect_with(options).await?;

        sqlx::migrate!().run(&pool).await?;

        Ok(Self { pool })
    }

    pub async fn select_item_by_uuid(&self, uuid: &str) -> AppResult<Option<Item>> {
        let Some(row) = sqlx::query("SELECT * FROM items WHERE uuid = $1 LIMIT 1")
            .bind(uuid)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?
        else {
            return Ok(None);
        };

        let uuid = row.get("uuid");
        let name = row.get("name");
        let comment = row.get("comment");
        let item_type: String = row.get("item_type");
        let item_type: ItemType =
            serde_json::from_str(&item_type).map_err(|e| AppError::SerdeError(e.to_string()))?;
        let target_cents = row.get("target_cents");
        let current_cents = row.get("current_cents");
        let archived = row.get("archived");
        let created_at = row.get("created_at");
        let updated_at = row.get("updated_at");

        let item = Item {
            uuid,
            name,
            comment,
            item_type,
            target_cents,
            current_cents,
            archived,
            created_at,
            updated_at,
        };

        Ok(Some(item))
    }

    pub async fn select_all_items_not_archived(&self) -> AppResult<Vec<Item>> {
        let rows = sqlx::query("SELECT * FROM items WHERE archived = FALSE")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let mut items = Vec::new();

        for row in rows {
            let uuid = row.get("uuid");
            let name = row.get("name");
            let comment = row.get("comment");
            let item_type: String = row.get("item_type");
            let item_type: ItemType = serde_json::from_str(&item_type)
                .map_err(|e| AppError::SerdeError(e.to_string()))?;
            let target_cents = row.get("target_cents");
            let current_cents = row.get("current_cents");
            let archived = row.get("archived");
            let created_at = row.get("created_at");
            let updated_at = row.get("updated_at");

            let item = Item {
                uuid,
                name,
                comment,
                item_type,
                target_cents,
                current_cents,
                archived,
                created_at,
                updated_at,
            };

            items.push(item);
        }

        Ok(items)
    }

    pub async fn insert_item(
        &self,
        name: &str,
        comment: Option<&str>,
        item_type: ItemType,
        target_cents: i64,
        current_cents: i64,
    ) -> AppResult<String> {
        let item_type =
            serde_json::to_string(&item_type).map_err(|e| AppError::SerdeError(e.to_string()))?;
        let uuid = Uuid::new_v4().to_string();
        let created_at: DateTime<Local> = Local::now();
        let created_at = created_at.to_rfc3339();

        sqlx::query("INSERT INTO items (uuid, name, comment, item_type, target_cents, current_cents, archived, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, FALSE, $7, $8)")
            .bind(&uuid)
            .bind(name)
            .bind(comment)
            .bind(&item_type)
            .bind(target_cents)
            .bind(current_cents)
            .bind(&created_at)
            .bind(&created_at)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(uuid)
    }

    pub async fn update_item_amount_with_uuid(
        &self,
        uuid: &str,
        amount_cents: i64,
    ) -> AppResult<()> {
        let updated_at: DateTime<Local> = Local::now();
        let updated_at = updated_at.to_rfc3339();

        let result =
            sqlx::query("UPDATE items SET current_cents = $1, updated_at = $2 WHERE uuid = $3")
                .bind(amount_cents)
                .bind(&updated_at)
                .bind(uuid)
                .execute(&self.pool)
                .await
                .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        if result.rows_affected() == 0 {
            return Err(AppError::DatabaseError(
                "cannot update item, as the uuid must not exist".into(),
            ));
        }

        Ok(())
    }

    pub async fn edit_item_with_uuid(
        &self,
        uuid: &str,
        name: &str,
        target_cents: i64,
        current_cents: i64,
    ) -> AppResult<()> {
        let updated_at: DateTime<Local> = Local::now();
        let updated_at = updated_at.to_rfc3339();

        let result = sqlx::query(
            "UPDATE items SET name = $1, target_cents = $2, current_cents = $3, updated_at = $4 WHERE uuid = $5",
        )
        .bind(name)
        .bind(target_cents)
        .bind(current_cents)
        .bind(&updated_at)
        .bind(uuid)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        if result.rows_affected() == 0 {
            return Err(AppError::DatabaseError(
                "cannot update item, as the uuid must not exist".into(),
            ));
        }

        Ok(())
    }
}
