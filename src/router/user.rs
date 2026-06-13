use axum::{
    Router, middleware,
    routing::{get, post},
};

use crate::{
    controller::user::{get_tasks, login, verify_otp},
    middleware::is_authenticated::is_authenticated,
    state::AppState,
};

pub fn get_user_router() -> Router<AppState> {
    let protected = Router::new()
        .route("/task", get(get_tasks))
        .layer(middleware::from_fn(is_authenticated));
    axum::Router::new()
        .route("/login", post(login))
        .route("/verify-otp", post(verify_otp))
        .merge(protected)
}
