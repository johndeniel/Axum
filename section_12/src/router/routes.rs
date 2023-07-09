// Import necessary modules and functions
use crate::json::return_json;

use axum::{
    Router,
    routing::get,
};

/// Creates and configures the Axum application.
///
/// This function sets up the routes and middleware for the application.
///
/// # Returns
///
/// Returns a configured `Router` instance.

pub fn app() -> Router {
    Router::new()
        // Define a route for the "/get_jason" path using the `get` method
        .route("/path", get(return_json::get_jason))
       
}