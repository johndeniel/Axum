// Import required external crates and modules.
use axum::{extract::Path, Extension, Json, http::StatusCode};
use sqlx::PgPool;
use serde::Deserialize;

// Define a struct `ResponseTask` that represents the data received in the HTTP request body.
// This struct is deserialized from JSON in the request body and contains optional fields.
#[derive(Deserialize)]
pub struct ResponseTask {
    pub id: Option<i32>,                // The 'id' field is an optional integer, used for potential updates.
    pub priority: Option<String>,       // The 'priority' field is an optional string for task priority.
    pub title: String,                  // The 'title' field is a required string for the task title.
    pub description: Option<String>,    // The 'description' field is an optional string for the task description.
    pub is_default: Option<bool>,       // The 'is_default' field is an optional boolean for a default task indicator.
}

// The `atomic_update` function is an asynchronous function that handles task updates.
// It takes three parameters:
// - `Path(id)`: An Axum extractor that retrieves the task ID from the request path.
// - `Extension(database)`: An Axum extractor that retrieves a PostgreSQL connection pool from request extensions.
// - `Json(response_task)`: An Axum extractor that deserializes JSON data from the request body into the `ResponseTask` struct.
// - It returns a `Result<(), StatusCode>`, where `()` indicates success and `StatusCode` represents the HTTP status code if an error occurs.
pub async fn atomic_update(
    Path(id): Path<i32>,                 // Extract the 'id' from the request URL path, such as "/tasks/1".
    Extension(database): Extension<PgPool>, // Extract the PostgreSQL connection pool from the request extensions.
    Json(response_task): Json<ResponseTask>, // Extract and deserialize JSON data from the request body into the `ResponseTask` struct.
) -> Result<(), StatusCode> {

    // Define the SQL query as a raw string. This query updates the 'tasks' table in the database.
    let query: &str = r#"
        UPDATE tasks 
            SET priority = $1, 
            title = $2, 
            description = $3, 
            is_default = $4 
         WHERE id = $5
    "#;

    // Prepare the SQL query using the `sqlx::query()` method.
    // The `query()` method takes the raw SQL query as an argument and returns a query builder.
    // We'll bind the values from the `response_task` struct and the `id` extracted from the URL path to the query builder.
    let update: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> = sqlx::query(query)
        .bind(response_task.priority)       // Bind the 'priority' field from 'response_task' to the first parameter ($1) in the SQL query.
        .bind(response_task.title)          // Bind the 'title' field from 'response_task' to the second parameter ($2) in the SQL query.
        .bind(response_task.description)    // Bind the 'description' field from 'response_task' to the third parameter ($3) in the SQL query.
        .bind(response_task.is_default)     // Bind the 'is_default' field from 'response_task' to the fourth parameter ($4) in the SQL query.
        .bind(id);                          // Bind the 'id' extracted from the URL path to the fifth parameter ($5) in the SQL query.

    // Execute the prepared SQL query using the PostgreSQL connection pool (`database`).
    // We use the `execute()` method to perform the update operation, which returns the number of rows affected.
    // If the update is successful (no errors), the function returns an `Ok(())`, indicating success.
    // If there is an error during the update operation, the function returns an `Err()` with a corresponding `StatusCode` for internal server error (HTTP 500).
    match update.execute(&database).await {
        Ok(_) => Ok(()),                          // The update was successful, return an `Ok(())`.
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR), // An error occurred, return an `Err()` with an HTTP 500 status code for internal server error.
    }
}