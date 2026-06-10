use axum::{
    Router, middleware,
    routing::{get, post},
};

use crate::{
    controller::user::{get_tasks, login},
    middleware::is_authenticated::is_authenticated,
    state::AppState,
};

pub fn get_user_router() -> Router<AppState> {
    let protected = Router::new()
        .route("/task", get(get_tasks))
        .layer(middleware::from_fn(is_authenticated));
    axum::Router::new()
        .route("/login", post(login))
        .merge(protected)
}
