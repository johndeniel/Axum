// Import the necessary modules and dependencies
use crate::query; // Import the `query` module from the current crate
use sqlx::postgres::PgPool; // Import the `PgPool` type from the `sqlx` crate
use axum::{ Router, routing::delete, Extension, }; // Import required types and functions from the `axum` crate

// Define an asynchronous function named `app` that takes a `DatabaseConnection` parameter.
// The function returns a `Router`.
pub async fn app(database: PgPool) -> Router {
    // Create a new router instance
    Router::new()

        // Add a route for the path "/path" with the `GET` method.
        // When a request is made to "/path", the `create_task` function from the `query::sea_orm` module will be called.
        .route("/path/:id", delete(query::sqlx::soft_delete))

        // Attach the `database` as an extension to the router.
        // This makes the `database` available to all route handlers in the router.
        .layer(Extension(database))
}