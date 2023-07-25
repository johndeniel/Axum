// Import necessary dependencies
use axum::{Extension, extract::Path, http::StatusCode, Json}; // Imports from the Axum framework
use sea_orm::{DatabaseConnection, EntityTrait}; // Imports from the SeaORM library
use crate::database::tasks::Entity as Tasks; // Imports the "Entity" trait from the "tasks" module in the "database" crate
use serde::Serialize; // Import the Serialize trait for the custom struct

// Define a custom struct `ResponseTask` that will be used to serialize the response as JSON
#[derive(Serialize)]
pub struct ResponseTask {
    id: i32,
    priority: Option<String>,
    title: String,
    description: Option<String>,
}

// The `create_task` function is an asynchronous function.
// It handles an HTTP request to create a task based on the given `id`.
// The function receives `id` from the HTTP path and `database` as an extension.
// The result of this function is either a JSON response containing task details
// or an HTTP status code indicating "Not Found" if the task with the given `id` is not found.
pub async fn create_task(
    Path(id): Path<i32>, // Extract the `id` from the HTTP path parameter
    Extension(database): Extension<DatabaseConnection>, // Extract the `database` from the HTTP extension
) -> Result<Json<ResponseTask>, StatusCode> {

    // Use the `Tasks` entity trait from the SeaORM library to find a task with the given `id`.
    // The `one` function is an asynchronous method that fetches one record from the database.
    let query: Option<crate::database::tasks::Model> = Tasks::find_by_id(id).one(&database).await.unwrap();

    // Check if the task with the given `id` was found in the database.
    if let Some(tasks) = query {
        // If the task was found, construct a `ResponseTask` instance and return it as a JSON response.
        // The `Ok` variant of the `Result` indicates a successful response.
        Ok(Json(ResponseTask {
            id: tasks.id,
            priority: tasks.priority,
            title: tasks.title,
            description: tasks.description,
        }))
    } else {
        // If the task was not found, return an "HTTP 404 Not Found" status code.
        // The `Err` variant of the `Result` indicates an error response.
        Err(StatusCode::NOT_FOUND)
    }
}