// Import necessary external crates.
use axum::{response::Json, extract::Extension, http::StatusCode};
use sqlx::PgPool;
use serde::Serialize;

// Define a struct representing the response of a task from the database.
#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct ResponseTask {
    id: i32,
    priority: Option<String>,
    title: String,
    description: Option<String>,
}

// An asynchronous function to get all tasks from the database and return them as JSON.
pub async fn get_all(
    Extension(database): Extension<PgPool>,
) -> Result<Json<Vec<ResponseTask>>, StatusCode> {
    // Define a SQL query to retrieve all tasks from the database.
    let query: &str = r#"
        SELECT id, priority, title, description
        FROM tasks
    "#;

    // Call the `fetch_task` function to execute the SQL query and fetch the tasks.
    match fetch_all(&database, query).await {
        Ok(query_result) => {
            // Convert the fetched tasks into a Vec<ResponseTask> using `map`.
            let response_tasks: Vec<ResponseTask> = query_result
                .into_iter()
                .map(|db_task| ResponseTask {
                    id: db_task.id,
                    priority: db_task.priority,
                    title: db_task.title,
                    description: db_task.description,
                })
                .collect();

            // Wrap the response_tasks in a JSON object and return it.
            Ok(Json(response_tasks))
        }
        Err(_) => {
            // If the task fetching encounters an error (e.g., database connection issue),
            // return a 500 Internal Server Error status code.
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// An asynchronous function to execute a SQL query and fetch tasks from the database.
async fn fetch_all(database: &PgPool, query: &str) -> Result<Vec<ResponseTask>, sqlx::Error> {
    // Execute the SQL query using `sqlx::query_as` and fetch the results as ResponseTask objects.
    match sqlx::query_as::<_, ResponseTask>(query).fetch_all(database).await {
        Ok(row) => Ok(row),
        Err(_) => {
            // If the SQL query execution encounters an error, return a custom error indicating
            // that the task is not found in the database.
            Err(sqlx::Error::RowNotFound)
        }
    }
}