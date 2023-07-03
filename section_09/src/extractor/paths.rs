use axum::Extension;
use crate::router::routes::SharedData;

/// Middleware function to handle the "/path" route and retrieve the message from shared data.
/// It takes an `Extension` of `SharedData` as a parameter and returns the message string.
pub async fn middleware_message(Extension(shared_data): Extension<SharedData>) -> String {
    shared_data.message
}

/* This code defines a middleware function called middleware_message that is responsible for handling 
the "/path" route and retrieving the message from the shared data. The function takes an Extension of 
SharedData as a parameter, which allows access to the shared data within the function.

The function is asynchronous (async) because it awaits the retrieval of the message. It then returns the 
message string.

By adding this middleware to a route, you can ensure that the shared data is accessible and utilized 
within that specific route. */