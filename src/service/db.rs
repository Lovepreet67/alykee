use std::env;

use sqlx::{PgPool, Postgres, pool::PoolOptions};

use crate::model::health_check::Status;

pub async fn get_db_pool() -> PgPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is rqd");
    PoolOptions::<Postgres>::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Connection to database failed please check you connection")
}

pub async fn test_connection(pool: &PgPool) -> Status {
    match sqlx::query("SELECT 1").execute(pool).await {
        Ok(_) => Status::Ok,
        Err(e) => Status::Error(format!("{:?}", e)),
    }
}
