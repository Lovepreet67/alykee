use axum::{Json, extract::State};

use crate::{
    model::{tasks::CreateTaskRequest, users::CreateUserRequest},
    state::AppState,
};

pub async fn create_task(
    State(app_state): State<AppState>,
    Json(task_req): Json<CreateTaskRequest>,
) {
    let pool = app_state.pool;
}
pub async fn create_user(
    State(app_state): State<AppState>,
    Json(user_req): Json<CreateUserRequest>,
) {
    let pool = app_state.pool;
}
