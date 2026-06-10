use sqlx::PgPool;
use uuid::Uuid;

use crate::model::users::{CreateUserRequest, User, UserResponse, UserRole};

pub struct UserRepository;

impl UserRepository {
    pub async fn create(
        pool: &PgPool,
        req: CreateUserRequest,
        hashed_password: String,
    ) -> Result<UserResponse, sqlx::Error> {
        let user = sqlx::query_as!(
            UserResponse,
            r#"
            INSERT INTO users (
                full_name,
                email,
                role,
                hashed_password
            )
            VALUES (
                $1,
                $2,
                $3,
                $4
            )
            RETURNING
                id,
                full_name,
                email,
                role as "role: UserRole",
                created_at
            "#,
            req.full_name,
            req.email,
            req.role as _,
            hashed_password
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT
                id,
                full_name,
                email,
                role as "role: UserRole",
                hashed_password,
                created_at
            FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT
                id,
                full_name,
                email,
                role as "role: UserRole",
                hashed_password,
                created_at
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }
}
