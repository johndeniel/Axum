use axum::{
    middleware::Next, 
    http::{Request, StatusCode}, 
    response::Response,
};

use crate::extractor::paths::HeaderMesage;

/// Middleware function to set a custom header in the request extensions.
///
/// This middleware extracts the "message" header from the incoming request and adds it
/// to the request extensions as a custom header.
///
/// # Parameters
///
/// - `request`: The incoming request.
/// - `next`: The next middleware or handler in the chain.
///
/// # Returns
///
/// Returns a `Result` containing the response or a `StatusCode` in case of an error.

pub async fn set_middleware_custom_header<B> (
    mut request: Request<B>, 
    next:Next<B>,
) -> Result<Response, StatusCode> {
    let headers = request.headers();

    // Get the "message" header from the request
    let message = headers
        .get("message")
        .ok_or_else(|| { StatusCode::BAD_REQUEST })?;

    // Convert the header value to a string
    let message = message.to_str().map_err(|_error| StatusCode::BAD_REQUEST)?.to_owned();
   
    // Insert the custom header into the request extensions
    let extension = request.extensions_mut();
    extension.insert(HeaderMesage(message));

    // Continue processing the request with the next middleware or handler
    Ok(next.run(request).await)
}