use leptos::*;

use crate::models::todo::Todo;

#[server(GetTodos, "/api")]
pub async fn get_todos(cx: Scope) -> Result<Vec<Todo>, ServerFnError> {
    use crate::components::todo_item::db;
    use sqlx::SqlitePool;

    let pool = use_context::<SqlitePool>(cx)
        .ok_or("Pool missing.")
        .map_err(|_| ServerFnError::ServerError("Pool Missing".to_string()))?;

    let rows = sqlx::query_as::<_, Todo>("SELECT * FROM todos")
        .fetch_all(&pool)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()));

    rows
}

#[server(AddTodo, "/api")]
pub async fn add_todo(cx: Scope, task: String) -> Result<(), ServerFnError> {
    use crate::components::todo_item::db;
    use sqlx::SqlitePool;

    let pool = use_context::<SqlitePool>(cx)
        .ok_or("Pool missing.")
        .map_err(|_| ServerFnError::ServerError("Pool Missing".to_string()))?;

    match sqlx::query("INSERT INTO todos (task) VALUES ($1)")
        .bind(task)
        .execute(&pool)
        .await
    {
        Ok(_row) => Ok(()),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}

#[server(DeleteTodo, "/api")]
pub async fn delete_todo(cx: Scope, id: u32) -> Result<(), ServerFnError> {
    use crate::components::todo_item::db;
    use sqlx::SqlitePool;

    let pool = use_context::<SqlitePool>(cx)
        .ok_or("Pool missing.")
        .map_err(|_| ServerFnError::ServerError("Pool Missing".to_string()))?;

    sqlx::query("DELETE FROM todos WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .map(|_| ())
        .map_err(|e| ServerFnError::ServerError(e.to_string()))
}

#[server(UpdateTodo, "/api")]
pub async fn update_todo(cx: Scope, id: u32, status: bool) -> Result<(), ServerFnError> {
    use crate::components::todo_item::db;
    use sqlx::SqlitePool;

    let pool = use_context::<SqlitePool>(cx)
        .ok_or("Pool missing.")
        .map_err(|_| ServerFnError::ServerError("Pool Missing".to_string()))?;

    sqlx::query(
        r#"
UPDATE todos
SET status = $1
WHERE id = $2
        "#,
    )
    .bind(status)
    .bind(id)
    .execute(&pool)
    .await
    .map(|_| ())
    .map_err(|e| ServerFnError::ServerError(e.to_string()))
}
