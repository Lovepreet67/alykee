use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Type;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[sqlx(type_name = "task_status", rename_all = "snake_case")]
pub enum TaskStatus {
    Todo,
    InProgress,
    Completed,
    Cancelled,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Task {
    pub id: Uuid,

    pub title: String,
    pub description: String,

    pub status: TaskStatus,
    pub priority: i64,

    pub created_by_id: Uuid,
    pub assigned_to_id: Uuid,

    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTaskRequest {
    pub title: String,
    pub description: String,

    pub priority: i64,

    pub assigned_to_id: Uuid,
}
