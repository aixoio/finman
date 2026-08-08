use std::{error::Error, str::FromStr};

use serde::{Deserialize, Serialize};
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    Row, SqlitePool,
};

pub const DATABASE_FILENAME: &str = "finman_database.sqlite3";

#[derive(Debug, Serialize, Deserialize)]
pub enum ItemType {
    Savings,
    SelfLoan,
    ExternalLoan,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    uuid: String,
    name: String,
    comment: Option<String>,
    item_type: ItemType,
    target_cents: i64,
    current_cents: i64,
    archived: bool,
    created_at: String,
    updated_at: String,
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

    pub async fn select_all_items_not_archived(&self) -> Result<Vec<Item>, Box<dyn Error>> {
        let rows = sqlx::query("SELECT * FROM items WHERE archived = FALSE")
            .fetch_all(&self.pool)
            .await?;

        let mut items = Vec::new();

        for row in rows {
            let uuid = row.get("uuid");
            let name = row.get("name");
            let comment = row.get("comment");
            let item_type: String = row.get("item_type");
            let item_type: ItemType = serde_json::from_str(&item_type)?;
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
}
