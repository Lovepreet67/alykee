use axum::{Json, extract::State};

use crate::{
    model::health_check::HealthCheckResult, service::db::test_connection as test_db_connection,
    service::redis::test_connection as test_redis_connection, state::AppState,
};

pub mod admin;
pub mod user;

#[allow(unused)]
pub async fn not_implemented() -> &'static str {
    "NOT implemented yet"
}

pub async fn health_check(State(app_state): State<AppState>) -> Json<HealthCheckResult> {
    let redis = test_redis_connection(app_state.redis_client).await;
    let db = test_db_connection(&app_state.pool).await;
    Json(HealthCheckResult { redis, db })
}
