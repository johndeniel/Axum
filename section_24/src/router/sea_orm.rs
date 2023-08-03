// Import the necessary modules and dependencies
use crate::query; // Import the `query` module from the current crate
use sea_orm::DatabaseConnection; // Import the `DatabaseConnection` type from the `sea_orm` crate
use axum::{Router, routing::post, Extension}; // Import required types and functions from the `axum` crate

// Define an asynchronous function named `app` that takes a `DatabaseConnection` parameter.
// The function returns a `Router`.
pub async fn app(database: DatabaseConnection) -> Router {
    // Create a new router instance
    Router::new()

        // Add a route for the path "/path" with the `PUT` method.
        // When a request is made to "/path/:id", the `create_task` function from the `query::sea_orm` module will be called.
        .route("/path", post(query::sea_orm::creating_account))

        // Attach the `database` as an extension to the router.
        // This makes the `database` available to all route handlers in the router.
        .layer(Extension(database))
}