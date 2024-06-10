use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use thiserror::Error;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse {
    message: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    errors: Vec<String>,
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    DatabaseError(#[from] diesel::result::Error),
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),
    #[error("{0}")]
    InternalServerError(String),
    #[error("{0}")]
    BadRequest(String),
    #[error("{0}")]
    NotFound(String),
}

impl From<deadpool_diesel::PoolError> for AppError {
    fn from(err: deadpool_diesel::PoolError) -> Self {
        AppError::InternalServerError(err.to_string())
    }
}

impl From<deadpool_diesel::InteractError> for AppError {
    fn from(err: deadpool_diesel::InteractError) -> Self {
        AppError::InternalServerError(err.to_string())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::DatabaseError(_) => {
                let message = self.to_string();
                (
                    StatusCode::BAD_REQUEST,
                    Json(ErrorResponse {
                        message,
                        errors: vec![],
                    }),
                )
            }
            AppError::ValidationError(_) => {
                let message = "Error during validation".to_string();
                (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    Json(ErrorResponse {
                        message,
                        errors: self.to_string().split('\n').map(String::from).collect(),
                    }),
                )
            }
            AppError::InternalServerError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    message: msg,
                    errors: vec![],
                }),
            ),
            AppError::BadRequest(msg) => (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    message: msg,
                    errors: vec![],
                }),
            ),
            AppError::NotFound(msg) => (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    message: msg,
                    errors: vec![],
                }),
            ),
        }
        .into_response()
    }
}
