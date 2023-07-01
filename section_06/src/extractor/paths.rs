use axum::{TypedHeader, headers::UserAgent};

// Define an asynchronous function named `id_extractor` that takes a `TypedHeader` parameter containing `UserAgent`
// This function is expected to be used as a handler for extracting and returning the user agent as a string
pub async fn standard_headers (TypedHeader(user_agent): TypedHeader<UserAgent>) -> String {
    // Convert the `user_agent` to a string and return it
    user_agent.to_string()
}

/* The standard_headers function is an asynchronous function that takes a TypedHeader parameter containing a UserAgent. 
This function is intended to be used as a handler for extracting and returning the user agent as a string.

The TypedHeader macro is used to extract the UserAgent header from the request. 
The extracted UserAgent value is then converted to a string using the to_string() method and returned. */