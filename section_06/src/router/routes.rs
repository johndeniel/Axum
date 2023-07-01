// Import the `message` function from the `extractor` module in the current crate
use crate::extractor::paths;

// Import necessary items from the `axum` crate
use axum::{
    Router,
    routing::get,
};

// Define the `app` function that returns a `Router`
pub fn app() -> Router {
    // Create a new `Router` instance
    Router::new()
        // Define a route for the "/path" path using the `get` method
        .route("/path", get(paths::standard_headers))
}