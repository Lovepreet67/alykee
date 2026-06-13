use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct TwoFAResponse {
    pub message: String,
}
#[derive(Serialize)]
pub struct JWTResponse {
    pub jwt: String,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum LoginResponse {
    TwoFA(TwoFAResponse),
    Jwt(JWTResponse),
}

#[derive(Deserialize)]
pub struct TokenVerificationRequest {
    pub email: String,
    pub otp: String,
}
