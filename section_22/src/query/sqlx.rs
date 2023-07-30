// Import necessary modules from external crates
use axum::{extract::Path, Extension, http::StatusCode}; // Import specific components from the "axum" crate
use sqlx::PgPool; // Import the "PgPool" type from the "sqlx" crate

// Define an asynchronous function named "hard_delete" that takes two parameters:
//   1. A "Path" object that extracts an integer "id" from the URL path
//   2. An "Extension" object that carries a "PgPool" instance to connect to a PostgreSQL database
// This function returns a Result with an empty value (()) or a StatusCode representing an error.
pub async fn hard_delete(
    Path(id): Path<i32>, // Extract the "id" from the URL path and bind it to the "id" variable
    Extension(database): Extension<PgPool>, // Extract the PostgreSQL database connection pool from the request and bind it to the "database" variable
) -> Result<(), StatusCode> {
    // Perform the delete operation using SQLx (an asynchronous SQL toolkit for Rust)
    // Execute the SQL query "DELETE FROM tasks WHERE id = $1" with the "id" parameter
    // The "$1" is a parameter placeholder that will be replaced with the actual value of "id"
    // The query will be executed using the PostgreSQL database connection provided by "database"
    // The result of the query execution is stored in the "query_result" variable
    let query_result = sqlx::query("DELETE FROM tasks WHERE id = $1")
        .bind(id) // Bind the "id" to the query as a parameter
        .execute(&database) // Execute the query with the database connection
        .await // Wait for the query execution to complete (since it's an asynchronous operation)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?; // Handle any errors during the query execution and map them to a 500 Internal Server Error

    // Check the number of rows affected (tasks deleted)
    if query_result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND); // If no rows were affected (no task was deleted), return a 404 Not Found error
    }

    Ok(()) // If the delete operation was successful and at least one row was affected, return Ok with an empty value (())
}