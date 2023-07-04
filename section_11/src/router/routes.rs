// Import necessary modules and functions
use crate::status_code::always_error;
use crate::status_code::returns_201;

use axum::{
    Router,
    routing::{get, post},
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
        // Define a route for the "/returns_201" path using the `get` method
        .route("/returns_201", post(returns_201::returns_201))
        // Define a route for the "/always_error" path using the `get` method
        .route("/always_error", get(always_error::always_error))  
}