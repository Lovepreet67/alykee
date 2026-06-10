use serde::Serialize;
use sqlx::FromRow;

pub mod auth;
pub mod tasks;
pub mod users;

#[derive(Serialize)]
pub struct ListedResponse<T>
where
    T: Serialize,
{
    pub list: Vec<T>,
}
