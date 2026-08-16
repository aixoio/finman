use serde::Serialize;
use thiserror::Error;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Error, Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(tag = "type", content = "message")]
pub enum AppError {
    #[error("database error")]
    Database(String),

    #[error("input error")]
    Input(String),

    #[error("serde error")]
    Serde(String),
}
