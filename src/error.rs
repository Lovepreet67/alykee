use std::env;

use axum::{Json, http::StatusCode, response::IntoResponse};
use bcrypt::BcryptError;
use serde::Serialize;

#[derive(Debug)]
pub struct APIError {
    status_code: StatusCode,
    message: String,
    error: String,
}
impl APIError {
    pub fn new(
        status_code: StatusCode,
        message: impl Into<String>,
        error: impl Into<String>,
    ) -> Self {
        Self {
            status_code,
            message: message.into(),
            error: error.into(),
        }
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
}
impl From<APIError> for ErrorResponse {
    fn from(value: APIError) -> Self {
        Self {
            error: value.error,
            message: value.message,
        }
    }
}

impl IntoResponse for APIError {
    fn into_response(self) -> axum::response::Response {
        return (self.status_code, Json(ErrorResponse::from(self))).into_response();
    }
}

// implementation to create inner error to user facing errors
impl Default for APIError {
    fn default() -> Self {
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Something Went Wrong".into(),
            error: "Something Went Wrong".into(),
        }
    }
}

// TODO: Panding implementations
impl From<BcryptError> for APIError {
    fn from(value: BcryptError) -> Self {
        APIError::default()
    }
}

impl From<sqlx::Error> for APIError {
    fn from(value: sqlx::Error) -> Self {
        APIError::default()
    }
}

impl From<jsonwebtoken::errors::Error> for APIError {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        APIError::default()
    }
}

impl From<std::env::VarError> for APIError {
    fn from(value: std::env::VarError) -> Self {
        APIError::default()
    }
}
