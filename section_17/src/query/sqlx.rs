// Import necessary libraries and modules
use axum::{Extension, Json, http::StatusCode, extract::Path};
use sqlx::{PgPool, Error::RowNotFound};
use serde::Serialize;

// Define a struct to represent a task response with its fields
#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct ResponseTask {
    id: i32,
    priority: Option<String>,
    title: String,
    description: Option<String>,
}

// Define an asynchronous function to create a task based on its ID
// The function takes two parameters:
// - `Path(id)`: Extracts the `id` from the URL path.
// - `Extension(database)`: Extracts the `PgPool` instance from the request's extensions.
pub async fn create_task(
    Path(id): Path<i32>,                      // Extracts the `id` from the URL path
    Extension(database): Extension<PgPool>,   // Extracts the `PgPool` instance from the request's extensions
) -> Result<Json<ResponseTask>, StatusCode> {
    // Define the SQL query to fetch a task based on its `id`
    let query: &str = r#"
        SELECT id, priority, title, description
        FROM tasks
        WHERE id = $1
    "#;

    // Call the `fetch_task` function to fetch the task from the database
    // `&database` is a reference to the `PgPool` instance, `query` is the SQL query, and `id` is the task ID.
    match fetch_task(&database, query, id).await {
        Ok(task) => {
            // If the task is successfully fetched from the database, return it as a JSON response
            Ok(Json(ResponseTask {
                id: task.id,
                priority: task.priority,
                title: task.title,
                description: task.description,
            }))
        }
        Err(_) => {
            // If the task is not found in the database, return a 404 Not Found status code.
            Err(StatusCode::NOT_FOUND)
        }
    }
}

// An asynchronous function that fetches a task from the database based on its ID.
// The function takes three parameters:
// - `database`: A reference to the `PgPool` instance representing the database connection pool.
// - `query`: The SQL query to fetch the task.
// - `id`: The ID of the task to fetch.
async fn fetch_task(database: &PgPool, query: &str, id: i32) -> Result<ResponseTask, sqlx::Error> {
    // Execute the SQL query using the `query_as` method, which fetches a row from the database and deserializes it into the `ResponseTask` struct.
    // `bind(id)` binds the `id` parameter to the `$1` placeholder in the SQL query.
    // `fetch_one` executes the query and fetches the single row result from the database.
    match sqlx::query_as::<_, ResponseTask>(query)
        .bind(id)
        .fetch_one(database)
        .await
    {
        Ok(row) => Ok(row), // If the query is successful, return the fetched row as a `ResponseTask`.
        Err(_) => {
            Err(RowNotFound)
        }, // If the query fails (task not found), return a `RowNotFound` error.
    }
}