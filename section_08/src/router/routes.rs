use tower_http::cors::{CorsLayer, Any};

// Import necessary items from the `axum` crate
use axum::{
    Router,
    routing::get,
    http::Method,
};

// Define the `app` function that returns a `Router`
pub fn app() -> Router {
    // Create a CORS layer with allowed methods and origins
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    // Create a new `Router` instance
    Router::new()
        // Define a route for the "/path" path using the `get` method
        .route("/path", get(string_extractor))
        .layer(cors)
}

// Extractor function that receives a `body` string and returns it as the response
pub async fn string_extractor(body: String) -> String {
    body
}

/* Inside the app function, we create a CorsLayer using CorsLayer::new(). 
The CorsLayer is configured to allow only the GET and POST methods and any origin. 
This layer will handle the Cross-Origin Resource Sharing (CORS) headers for the routes.

Next, we create a new Router instance using Router::new(). 
We define a route for the "/path" path using the get method and the string_extractor function as the handler. 
The string_extractor function is an asynchronous function that receives a body string and returns it as the response.

Finally, we apply the CorsLayer to the router using the layer method, which adds the CORS middleware to the router. */