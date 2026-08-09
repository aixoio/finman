use serde::Serialize;
use thiserror::Error;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Error, Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(tag = "type", content = "message")]
pub enum AppError {
    #[error("database error")]
    DatabaseError(String),

    #[error("input error")]
    InputError(String),
}
