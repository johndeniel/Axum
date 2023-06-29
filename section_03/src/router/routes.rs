// Import the `message` function from the `extractor` module in the current crate
use crate::extractor::message;

// Import necessary items from the `axum` crate
use axum::{
    Router,
    routing::post,
};

// Define the `app` function that returns a `Router`
pub fn app() -> Router {
    // Create a new `Router` instance
    Router::new()
        // Define a route for the "/message" path using the `post` method
        .route("/message", post(message::string_extractor))
}