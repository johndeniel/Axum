// Import necessary modules and functions
use crate::extractor::paths;
use crate::middleware::custom_header;

use axum::{
    Router,
    routing::get, middleware,
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
        // Define a route for the "/path" path using the `get` method
        .route("/path", get(paths::middleware_message))
        // Apply the custom middleware to the route layer
        .route_layer(middleware::from_fn(custom_header::set_middleware_custom_header))
}