use axum::http::HeaderMap;

// Define an asynchronous function named `custom_headers` that takes a `HeaderMap` parameter
// This function is expected to be used as a handler for extracting and returning a custom header value as a string
pub async fn custom_headers(header: HeaderMap) -> String {
    // Extract the value of the "User-Agent" header from the `header` map using the `get` method
    let user_agent_header = header.get("User-Agent").unwrap();

    // Convert the header value to a string and return it
    user_agent_header.to_str().unwrap().to_owned()
}

/* The custom_headers function is an asynchronous function that takes a HeaderMap parameter. 
This function is intended to be used as a handler for extracting and returning a custom header value as a string.

The get method is used to retrieve the value of the "User-Agent" header from the header map. 
The method returns an Option since the header may or may not exist in the map. In this code, we use unwrap assuming the "User-Agent" header is present.

Then, the extracted header value is converted to a string using the to_str() method. 
If the conversion is successful, the string value is returned. Otherwise, an error is propagated using unwrap. */