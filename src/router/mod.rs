use axum::Router;

use crate::{
    router::{admin::get_admin_router, user::get_user_router},
    state::AppState,
};

mod admin;
mod user;

pub fn get_router() -> Router<AppState> {
    let user_router = get_user_router();
    let admin_router = get_admin_router();
    Router::new().merge(user_router).merge(admin_router)
}
