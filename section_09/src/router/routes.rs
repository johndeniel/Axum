// Import the `message` function from the `extractor` module in the current crate
use crate::extractor::paths;

// Import necessary items from the `axum` crate
use axum::{
    Router,
    routing::get,
    Extension,
};

// Define a struct to hold shared data
#[derive(Clone)]
pub struct SharedData {
    pub message: String,
}

// Define the `app` function that returns a `Router`
pub fn app() -> Router {
    // Create an instance of the shared data
    let shared_data = SharedData {
        message: "Hi Guys!".to_owned(),
    };

    // Create a new `Router` instance
    Router::new()
        // Define a route for the "/path" path using the `get` method
        .route("/path", get(paths::middleware_message))
        // Add the shared data as a layer to the router
        .layer(Extension(shared_data))
}