use axum::{
    response::{Response, IntoResponse}, 
    http::StatusCode
};

/// An asynchronous function that returns a response with the status code 201 (Created).
///
/// This function demonstrates how to create a response with a custom status code and body.
///
/// # Returns
///
/// Returns a `Response` object with the status code 201 and a body containing the message "This is a 201".
pub async fn returns_201() -> Response {
    (
        StatusCode::CREATED,
        "This is a 201".to_owned()
    ).into_response()
}