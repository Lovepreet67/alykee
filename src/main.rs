use std::env;

use axum::Router;
use dotenv::dotenv;
use sqlx::{PgPool, Postgres, pool::PoolOptions};
use uuid::Uuid;

use crate::{
    error::APIError, model::users::UserRole, repository::users::UserRepository, router::get_router,
    state::AppState,
};

mod controller;
mod error;
mod middleware;
mod model;
mod repository;
mod router;
mod service;
mod state;
// this function will create the admin user
const ROOT_USER_ID: &str = "00000000-0000-0000-0000-000000000001";

pub async fn init(pool: &PgPool) -> Result<(), APIError> {
    let root_user_email = env::var("ROOT_USER_EMAIL").expect("ROOT_USER_EMAIL is required");

    let root_user_password =
        env::var("ROOT_USER_PASSWORD").expect("ROOT_USER_PASSWORD is required");

    let root_user_id = Uuid::parse_str(ROOT_USER_ID).expect("Invalid ROOT_USER_ID");

    // Check if the root user already exists.
    let existing = UserRepository::find_by_id(pool, root_user_id).await?;

    if existing.is_some() {
        println!("Root user already exists.");
        return Ok(());
    }

    let hashed_password = bcrypt::hash(&root_user_password, 10)?;

    sqlx::query!(
        r#"
        INSERT INTO users (
            id,
            full_name,
            email,
            role,
            hashed_password
        )
        VALUES (
            $1,
            $2,
            $3,
            $4,
            $5
        )
        "#,
        root_user_id,
        "Root Admin",
        root_user_email,
        UserRole::Admin as _,
        hashed_password,
    )
    .execute(pool)
    .await?;
    println!("Root user created successfully.");

    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is rqd");
    let pool = PoolOptions::<Postgres>::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Connection to database failed please check you connection");
    init(&pool).await.expect("Error during init");
    let state = AppState { pool };
    let router = get_router();
    let state_router = router.with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Error while creating tcp bind listener at port 3000");
    axum::serve(listener, state_router).await.unwrap();
}
