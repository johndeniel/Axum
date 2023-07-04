use axum::Extension;

// Define a custom struct `HeaderMessage` to hold the message string
#[derive(Clone)]
pub struct HeaderMesage(pub String);

// Define the `middleware_message` function that extracts the message from the extension
pub async fn middleware_message(Extension(message): Extension<HeaderMesage>) -> String {
    message.0    // Return the message string
}

/* The code above demonstrates the usage of a custom struct HeaderMessage to store the message string. 
The middleware_message function extracts the message from the extension and returns it. */