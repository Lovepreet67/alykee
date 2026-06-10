use sqlx::PgPool;
use uuid::Uuid;

use crate::model::tasks::{CreateTaskRequest, Task};

pub struct TaskRepository;

impl TaskRepository {
    pub async fn create(
        pool: &PgPool,
        req: CreateTaskRequest,
        created_by_id: Uuid,
    ) -> Result<Task, sqlx::Error> {
        let task = sqlx::query_as!(
            Task,
            r#"
            INSERT INTO task (
                title,
                description,
                status,
                priority,
                created_by_id,
                assigned_to_id
            )
            VALUES (
                $1,
                $2,
                'todo'::task_status,
                $3,
                $4,
                $5
            )
            RETURNING
                id,
                title,
                description,
                status as "status: TaskStatus",
                priority,
                created_by_id,
                assigned_to_id,
                created_at,
                updated_at
            "#,
            req.title,
            req.description,
            req.priority,
            created_by_id,
            req.assigned_to_id
        )
        .fetch_one(pool)
        .await?;

        Ok(task)
    }

    pub async fn list_by_user_id(pool: &PgPool, user_id: Uuid) -> Result<Vec<Task>, sqlx::Error> {
        let tasks = sqlx::query_as!(
            Task,
            r#"
            SELECT
                id,
                title,
                description,
                status as "status: TaskStatus",
                priority,
                created_by_id,
                assigned_to_id,
                created_at,
                updated_at
            FROM task
            WHERE assigned_to_id = $1
            ORDER BY created_at DESC
            "#,
            user_id
        )
        .fetch_all(pool)
        .await?;

        Ok(tasks)
    }
}
