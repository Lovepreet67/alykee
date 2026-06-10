use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Type;
use uuid::Uuid;
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[sqlx(type_name = "user_role", rename_all = "snake_case")]
pub enum UserRole {
    Staff,
    Admin,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub full_name: String,
    pub email: String,
    pub password: String,
    pub role: UserRole,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub full_name: String,
    pub email: String,
    pub role: UserRole,
    pub created_at: DateTime<Utc>,
}
