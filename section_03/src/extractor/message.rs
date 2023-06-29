// Import the `Json` type from the `axum` crate for handling JSON requests and responses
use axum::Json;
// Import the `Deserialize` and `Serialize` traits from the `serde` crate for serialization and deserialization
use serde::{Deserialize, Serialize};

// Define a struct named `Request` and derive `Serialize` and `Deserialize` traits for JSON serialization and deserialization
#[derive(Serialize, Deserialize)]
pub struct Request {
    message_json: String,
}

// Define a struct named `Response` and derive `Serialize` and `Deserialize` traits for JSON serialization and deserialization
#[derive(Serialize, Deserialize)]
pub struct Response {
    message_json: String,
    response_json: String,
}

// Define an asynchronous function named `string_extractor` that takes a `Json` parameter containing a deserialized `Request` struct
// This function is expected to be used as an endpoint for handling JSON requests
pub async fn string_extractor(Json(body): Json<Request>) -> Json<Response> {
    // Create a new `Response` struct and return it as a JSON response
    Json(Response {
        message_json: body.message_json,
        response_json: "Hello from server".to_owned(),
    })
}