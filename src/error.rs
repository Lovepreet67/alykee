use axum::{
    Json,
    http::{StatusCode, header::ToStrError},
    response::IntoResponse,
};
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
        (self.status_code, Json(ErrorResponse::from(self))).into_response()
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
    fn from(_: BcryptError) -> Self {
        APIError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to process password.",
            "BCRYPT_ERROR",
        )
    }
}

impl From<sqlx::Error> for APIError {
    fn from(_: sqlx::Error) -> Self {
        APIError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Database operation failed.",
            "DATABASE_ERROR",
        )
    }
}

impl From<jsonwebtoken::errors::Error> for APIError {
    fn from(_: jsonwebtoken::errors::Error) -> Self {
        APIError::new(
            StatusCode::UNAUTHORIZED,
            "Invalid or expired authentication token.",
            "INVALID_TOKEN",
        )
    }
}

impl From<std::env::VarError> for APIError {
    fn from(_: std::env::VarError) -> Self {
        APIError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Server configuration error.",
            "ENVIRONMENT_VARIABLE_MISSING",
        )
    }
}

impl From<lettre::address::AddressError> for APIError {
    fn from(_: lettre::address::AddressError) -> Self {
        APIError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error while generating the email address",
            "EMAIL_ADDRESS_ERROR",
        )
    }
}

impl From<lettre::error::Error> for APIError {
    fn from(_: lettre::error::Error) -> Self {
        APIError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error while generating the email address",
            "EMAIL_ADDRESS_ERROR",
        )
    }
}
impl From<redis::RedisError> for APIError {
    fn from(val: redis::RedisError) -> Self {
        eprintln!("{:?}", val);
        APIError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Error from redis".into(),
            error: "Error from redis".into(),
        }
    }
}

impl From<lettre::transport::smtp::Error> for APIError {
    fn from(value: lettre::transport::smtp::Error) -> Self {
        eprintln!("{:?}", value);
        APIError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Error from mailing service".into(),
            error: "Error from service".into(),
        }
    }
}

impl From<ToStrError> for APIError {
    fn from(_: ToStrError) -> Self {
        APIError {
            status_code: StatusCode::BAD_REQUEST,
            message: "Invalid String provided".into(),
            error: "INVALID_STRING".into(),
        }
    }
}
