// Import necessary dependencies
use axum::{Extension, http::StatusCode, Json}; // Imports from the Axum framework
use sea_orm::{DatabaseConnection, EntityTrait}; // Imports from the SeaORM library
use crate::database::tasks::Entity as Tasks; // Imports the "Entity" trait from the "tasks" module in the "database" crate
use serde::Serialize; // Import the Serialize trait for the custom struct

// Define a custom struct `ResponseTask` that represents the data to be returned as JSON
#[derive(Serialize)]
pub struct ResponseTask {
    id: i32,
    priority: Option<String>,
    title: String,
    description: Option<String>,
}

// Define the function `get_all` that will be called to create a task
pub async fn get_all(

    // The function takes an `Extension` parameter that holds the database connection.
    // The `DatabaseConnection` type is imported from the SeaORM library.
    Extension(database): Extension<DatabaseConnection>, 
) -> Result<Json<Vec<ResponseTask>>, StatusCode> {
    // The function returns a `Result` with two possible outcomes:
    // - Ok(Json(Vec<ResponseTask>)) if the operation is successful and returns a vector of `ResponseTask` structs.
    // - Err(StatusCode) if an error occurs, and the error is represented by an HTTP status code.

    // The next lines perform the database query to fetch tasks and handle any errors that might occur.

    // Perform the database query using the SeaORM's `find` method on the `Tasks` entity.
    let query: Vec<ResponseTask> = Tasks::find()
        .all(&database) // Fetch all tasks from the database using the provided connection.
        .await // This is an asynchronous operation, so we need to await the result.
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)? // Handle any potential errors and return an internal server error status code if an error occurs.

    // The next lines map the fetched database tasks into `ResponseTask` structs and collect them into a vector.

    // Use `into_iter` to get an iterator from the query result.
    // Then, for each database task (represented by `db_task`), create a corresponding `ResponseTask` struct.
    // The `ResponseTask` struct is created by selecting the relevant fields from the `db_task`.
    // The `id`, `priority`, `title`, and `description` fields of `ResponseTask` are assigned the values from `db_task`.
    // The `priority` and `description` fields are wrapped in `Option` to account for the possibility of a task having no priority or description.
    .into_iter()
        .map(|db_task| ResponseTask { 
            id: db_task.id, 
            priority: db_task.priority, 
            title: db_task.title, 
            description: db_task.description})
        .collect(); // Collect the mapped `ResponseTask` structs into a vector.

    // Finally, wrap the vector of `ResponseTask` structs into a JSON response and return it as part of the `Ok` variant of the `Result`.
    Ok(Json(query))
}