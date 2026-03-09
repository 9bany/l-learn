use sqlx::PgPool;
use uuid::Uuid;

use models::Todo;

pub async fn create(pool: &PgPool, title: String) -> Result<Todo, sqlx::Error> {
    let id = Uuid::new_v4();

    let todo = sqlx::query_as::<_, Todo>(
        r#"
        INSERT INTO todos (id, title, completed)
        VALUES ($1, $2, false)
        RETURNING id, title, completed
        "#
    )
    .bind(id)
    .bind(title)
    .fetch_one(pool)
    .await?;

    Ok(todo)
}

pub async fn list(pool: &PgPool) -> Result<Vec<Todo>, sqlx::Error> {
    let todos = sqlx::query_as::<_, Todo>(
        "SELECT id, title, completed FROM todos"
    )
    .fetch_all(pool)
    .await?;

    Ok(todos)
}