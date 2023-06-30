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
        // Define a route for the "/path/:id" path using the `get` method
        .route("/path/:id", get(paths::id_extractor))
        // Define a route for the "/path/14" path using the `get` method
        .route("/path/14", get(paths::absolute_path))
}

/* The first route is defined for the "/path/:id" path, where ":id" 
represents a dynamic segment in the path. It uses the get method 
and specifies the id_extractor function from the paths module as 
the handler for this route. This route is designed to extract the 
id value from the path variable.

The second route is defined for the "/path/14" path. 
It also uses the get method and specifies the absolute_path function 
from the paths module as the handler for this route. This route is 
meant to handle requests specifically targeting the "/path/14" absolute path. */