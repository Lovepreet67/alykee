use axum::http::StatusCode;
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    error::APIError,
    model::users::{User, UserRole},
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Claims {
    /// Subject (user id)
    pub sub: Uuid,

    /// Email
    pub email: String,

    /// User role
    pub role: UserRole,

    /// Expiration timestamp
    pub exp: usize,
}
pub async fn sign_jwt(user: &User) -> Result<String, APIError> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        //TODO: Update this unwrap
        .unwrap()
        .timestamp() as usize;

    let claims = Claims {
        sub: user.id,
        email: user.email.clone(),
        role: user.role.clone(),
        exp: expiration,
    };

    let secret = std::env::var("JWT_SECRET")?;

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;

    Ok(token)
}

pub fn verify_jwt(token: &str) -> Result<Claims, APIError> {
    let secret = std::env::var("JWT_SECRET")?;

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_e| {
        return APIError::new(
            StatusCode::UNAUTHORIZED,
            "Invalid Jwt token",
            "INVALID_TOKEN",
        );
    })?;

    Ok(token_data.claims)
}
