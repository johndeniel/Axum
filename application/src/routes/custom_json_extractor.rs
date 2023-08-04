// Module: custom_json_extractor
// Description: This Rust code defines a custom JSON extractor for a user data structure.
// The extractor is designed to parse and validate JSON payloads into instances of the `RequestUser` struct.
// The `RequestUser` struct represents user information with validation rules defined using the `validator` crate.
// Upon successful extraction and validation, the user data is available for further processing within the application.

// External Dependencies:
// - axum: A powerful web application framework for Rust.
// - async_trait: A procedural macro that allows defining asynchronous traits in Rust.
// - serde: A popular serialization/deserialization framework for Rust.
// - validator: A data validation library for Rust that provides declarative validation rules.

// Struct: RequestUser
// Description: A user data structure representing the information extracted from JSON requests.
// Details:
// - `username`: A string representing the user's email address, validated as a valid email using the `validator` crate.
// - `password`: A string representing the user's password, validated to have a minimum length of 8 characters using the `validator` crate.

// Implementation Details:
// - The `FromRequest` trait is implemented for the `RequestUser` struct, allowing it to be used as a custom extractor in Axum applications.
// - The trait implementation defines the `from_request` method, which performs the JSON extraction and validation process.
// - Within the `from_request` method, the incoming request body is parsed as JSON and deserialized into a `RequestUser` instance.
// - During deserialization, the validation rules defined in the `RequestUser` struct using the `validator` crate are applied.
// - If any validation rule fails, an appropriate `BAD_REQUEST` status code is returned along with the validation error message.
// - If the JSON payload is successfully parsed and validated, the extracted `RequestUser` instance is returned.
// - In case of parsing or deserialization errors, or if the validation fails, a rejection with an appropriate status code and error message is returned.

// Function: custom_json_extractor
// Description: A handler function that takes a `RequestUser` struct as an input parameter.
// Details:
// - This function is used as a part of the Axum application to handle user data extracted from incoming HTTP requests.
// - The extracted `RequestUser` instance is passed to this handler function for further processing or to be used in the application logic.
// - In this example implementation, the function simply prints the extracted user data using the `dbg!` macro.

// Note: The provided code showcases a custom JSON extractor for user data with validation.
// This kind of extraction and validation is a crucial step in ensuring the integrity and security of user input in web applications.
// By implementing a custom extractor and using the `validator` crate for validation, developers can create robust, safe, and efficient web services that handle user data effectively.

use axum::{
    async_trait,
    body::HttpBody,
    extract::FromRequest,
    http::{Request, StatusCode},
    BoxError, Json, RequestExt,
};
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct RequestUser {
    #[validate(email(message = "must be a valid email"))]
    pub username: String,
    #[validate(length(min = 8, message = "must have at least 8 characters"))]
    pub password: String,
}

#[async_trait]
impl<S, B> FromRequest<S, B> for RequestUser
where
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request(request: Request<B>, _state: &S) -> Result<Self, Self::Rejection> {
        let Json(user) = request
            .extract::<Json<RequestUser>, _>()
            .await
            .map_err(|error| (StatusCode::BAD_REQUEST, format!("{}", error)))?;

        if let Err(errors) = user.validate() {
            return Err((StatusCode::BAD_REQUEST, format!("{}", errors)));
        }

        Ok(user)
    }
}

pub async fn custom_json_extractor(user: RequestUser) {
    dbg!(user);
}