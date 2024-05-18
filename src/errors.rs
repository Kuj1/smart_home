use axum::{http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub type DataResult<T> = Result<T, DataError>;

#[derive(Debug, Error, Serialize, Deserialize, Clone)]
pub enum DataError {
    #[error("Endpoint is not found: {0}")]
    NotFound(String),
    #[error("Internal server error: {0}")]
    InternalError(String),
}

impl From<sqlx::Error> for DataError {
    fn from(value: sqlx::Error) -> Self {
        Self::InternalError(value.to_string())
    }
}

impl IntoResponse for DataError {
    fn into_response(self) -> axum::response::Response {
        match self {
            DataError::NotFound(e) => (StatusCode::NOT_FOUND, e.to_string()).into_response(),
            DataError::InternalError(e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            }
        }
    }
}
