use axum::{
    Router,
    routing::{get, post},
};

use crate::controller::not_implemented;

pub fn get_user_router() -> Router {
    axum::Router::new()
        .route("/login", post(not_implemented))
        .route("/tasks", get(not_implemented))
}
