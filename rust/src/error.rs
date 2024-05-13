use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Serialize)]
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
