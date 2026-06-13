use serde::Serialize;
#[derive(Serialize)]
pub enum Status {
    Ok,
    Error(String),
}

#[derive(Serialize)]
pub struct HealthCheckResult {
    pub redis: Status,
    pub db: Status,
}
