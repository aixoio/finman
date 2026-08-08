use std::str::FromStr;

use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    SqlitePool,
};

pub const DATABASE_FILENAME: &str = "finman_database.sqlite3";

pub enum ItemType {
    Savings,
    SelfLoan,
    ExternalLoan,
}

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
}
