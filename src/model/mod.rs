use serde::Serialize;

pub mod auth;
pub mod health_check;
pub mod tasks;
pub mod users;

#[derive(Serialize)]
pub struct ListedResponse<T>
where
    T: Serialize,
{
    pub list: Vec<T>,
}

#[allow(unused)]
#[derive(Serialize)]
pub struct GeneralResponse {
    pub message: String,
}
