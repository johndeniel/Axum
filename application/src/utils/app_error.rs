/**
 * Custom Error Handling in Axum
 *
 * This module defines a custom error type, `AppError`, and implements the `IntoResponse` trait for it. The `AppError` struct
 * is used to represent application-specific errors, containing an HTTP status code and an error message. By implementing the
 * `IntoResponse` trait, instances of `AppError` can be converted into HTTP responses using Axum's response machinery.
 *
 * # Example
 *
 * ```rust
 * use axum::{http::StatusCode, response::IntoResponse, Json};
 * use serde::Serialize;
 *
 * pub struct AppError {
 *     code: StatusCode,
 *     message: String,
 * }
 *
 * impl AppError {
 *     pub fn new(code: StatusCode, message: impl Into<String>) -> Self {
 *         Self {
 *             code,
 *             message: message.into(),
 *         }
 *     }
 * }
 *
 * impl IntoResponse for AppError {
 *     fn into_response(self) -> axum::response::Response {
 *         (
 *             self.code,
 *             Json(ResponseMessage {
 *                 message: self.message,
 *             }),
 *         )
 *             .into_response()
 *     }
 * }
 *
 * #[derive(Serialize)]
 * struct ResponseMessage {
 *     message: String,
 * }
 * ```
 *
 * In this example, you can create an instance of `AppError` using `AppError::new()`, passing the desired `StatusCode` and an error
 * message as arguments. When an instance of `AppError` is returned from an Axum handler, it will automatically be converted to an
 * HTTP response with the specified status code and a JSON body containing the error message.
 *
 * Custom error handling like this allows applications to provide more informative and consistent error responses to clients,
 * enhancing the overall user experience.
*/

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

pub struct AppError {
    code: StatusCode,
    message: String,
}

impl AppError {
    pub fn new(code: StatusCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (
            self.code,
            Json(ResponseMessage {
                message: self.message,
            }),
        )
            .into_response()
    }
}

#[derive(Serialize)]
struct ResponseMessage {
    message: String,
}
