use axum::http::StatusCode;

/// An asynchronous function that always returns an error with the status code "I'm a teapot".
///
/// This function is used for testing purposes or to simulate an endpoint that intentionally
/// produces an error response.
///
/// # Returns
///
/// Returns a `Result` indicating the error with the status code.
pub async fn always_error() -> Result<(), StatusCode> {
    Err(StatusCode::IM_A_TEAPOT)
}