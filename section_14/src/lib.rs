// Import the `router` module from the `router.rs` file (assumed to be in the same directory).
mod router;
// Import the necessary items from the `sea_orm` crate.
use sea_orm::Database;

// Define an asynchronous function named `run` that takes a reference to a `database_uri` string.

pub async fn run(database_uri: &str) {
    // Connect to the database using the provided `database_uri` and store the `Database` connection.
    // This operation is asynchronous, so it awaits the connection.
    let database = Database::connect(database_uri).await.unwrap();

    // Initialize the application routes using the `router::routes::app` function, passing the `Database` connection.
    // This operation is asynchronous, so it awaits the result.
    let app = router::routes::app(database).await;

    // Bind the server to the address "0.0.0.0:3000" and start serving the application.
    // The `app` is converted into an axum `Service` using `into_make_service()`.
    // This operation is asynchronous, so it awaits the result.
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}