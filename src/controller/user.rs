use axum::{Extension, Json, extract::State, http::StatusCode};

use crate::{
    error::APIError,
    model::{
        ListedResponse,
        auth::{LoginRequest, LoginResponse},
        tasks::Task,
        users::User,
    },
    repository::{tasks::TaskRepository, users::UserRepository},
    service::auth::{Claims, sign_jwt},
    state::AppState,
};

// this should be paginated
pub async fn get_tasks(
    State(app_state): State<AppState>,
    Extension(auth_claims): Extension<Claims>,
) -> Result<Json<ListedResponse<Task>>, APIError> {
    let pool = app_state.pool;
    let user_id = auth_claims.sub;
    let tasks = TaskRepository::list_by_user_id(&pool, user_id).await?;
    let listed = ListedResponse { list: tasks };
    return Ok(Json(listed));
}

pub async fn login(
    State(app_state): State<AppState>,
    Json(login_req): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, APIError> {
    let pool = app_state.pool;
    let user = UserRepository::find_by_email(&pool, &login_req.email).await?;
    if user.is_none() {
        return Err(APIError::new(
            StatusCode::NOT_FOUND,
            "Incorrect Username or passwor",
            "INVALID_CREDENTIALS",
        ));
    }
    let user = user.unwrap();
    let test = bcrypt::verify(&login_req.password, &user.hashed_password)?;
    if !test {
        return Err(APIError::new(
            StatusCode::NOT_FOUND,
            "Incorrect Username or passwor",
            "INVALID_CREDENTIALS",
        ));
    }
    // else we will generate the jwt
    let jwt = sign_jwt(&user).await?;
    return Ok(Json(LoginResponse { jwt }));
}
