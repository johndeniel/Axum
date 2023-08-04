// The following code defines a simple asynchronous function to demonstrate how Axum's `Json` extractor and Serde work together to deserialize JSON data in a web application.

// Import required modules and libraries.
// - `axum::Json`: Imports the `Json` extractor from the Axum web framework, which is used to deserialize JSON data from HTTP request bodies.
// - `serde::Deserialize`: Imports the `Deserialize` trait from Serde, which is used to derive deserialization for the `RequestUser` structure.

// Define a structure `RequestUser` with fields for username and password.
// The `Deserialize` trait is derived for this structure, allowing automatic deserialization from JSON when using the `Json` extractor.

// Define an asynchronous function `validate_with_serde` to demonstrate the usage of `Json` extractor with Serde.
// The function takes a `Json<RequestUser>` parameter, which represents the JSON data of type `RequestUser` obtained from the HTTP request body.
// The function uses the `dbg!` macro to print the deserialized `user` parameter, demonstrating how the JSON data is automatically deserialized into the `RequestUser` structure.

// Note: The code assumes that Axum and Serde libraries are already imported and available in the project's dependencies.
// When this function is used as a request handler in an Axum web application, it will automatically deserialize JSON data from the request body into a `RequestUser` structure.
// If the JSON data in the request body matches the structure of `RequestUser`, the function will be called with the deserialized data.
// If the JSON data does not match the structure or contains any missing or extra fields, the deserialization will fail, and the function will not be called.
// The `Option` type is used for the `username` field, allowing the field to be optional in the JSON data, while the `password` field is required.
// This provides flexibility in handling optional fields in JSON data while ensuring the required fields are present.

use axum::Json;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RequestUser {
    pub username: Option<String>,
    pub password: String,
}

pub async fn validate_with_serde(Json(user): Json<RequestUser>) {
    dbg!(user);
}