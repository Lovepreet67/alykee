use axum::Router;
use axum::routing::{get, post};

use crate::controller::not_implemented;

fn get_admin_router() -> Router {
    let protected = Router::new()
        .route("/user", post(not_implemented))
        .route("/task", post(not_implemented));
    protected
}
