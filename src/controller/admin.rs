use axum::{Extension, Json, extract::State};

use crate::{
    error::APIError,
    model::{
        tasks::{CreateTaskRequest, Task},
        users::{CreateUserRequest, UserResponse},
    },
    repository::{tasks::TaskRepository, users::UserRepository},
    service::auth::Claims,
    state::AppState,
};

pub async fn create_task(
    State(app_state): State<AppState>,
    Extension(auth_claims): Extension<Claims>,
    Json(task_req): Json<CreateTaskRequest>,
) -> Result<Json<Task>, APIError> {
    let pool = app_state.pool;
    // as user will be authenticated so there will be id
    let task = TaskRepository::create(&pool, task_req, auth_claims.sub).await?;
    Ok(Json(task))
}
pub async fn create_user(
    State(app_state): State<AppState>,
    Json(user_req): Json<CreateUserRequest>,
) -> Result<Json<UserResponse>, APIError> {
    let pool = app_state.pool;
    let hashed_password = bcrypt::hash(&user_req.password, 10)?;
    // here we will create a password hash for the user
    let user = UserRepository::create(&pool, user_req, hashed_password).await?;
    Ok(Json(user))
}
