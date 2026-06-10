use axum::routing::{get, post};
use axum::{Router, middleware};

use crate::controller::admin::{create_task, create_user};
use crate::controller::not_implemented;
use crate::middleware::is_authenticated::{have_admin_role, is_authenticated};
use crate::state::AppState;

pub fn get_admin_router() -> Router<AppState> {
    let protected = Router::new()
        .route("/user", post(create_user))
        .route("/task", post(create_task))
        .route_layer(middleware::from_fn(have_admin_role))
        .route_layer(middleware::from_fn(is_authenticated));
    protected
}
