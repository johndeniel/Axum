// Import necessary modules and functions
use axum::{
    Router,
    routing::get, Extension, 
};

use sea_orm::DatabaseConnection;


/// Creates and configures the Axum application.
///
/// This function sets up the routes and middleware for the application.
///
/// # Returns
///
/// Returns a configured `Router` instance.

pub async fn app(database: DatabaseConnection) -> Router {
 
    Router::new()
        .route("/path", get(message))
        .layer(Extension(database))
}


pub async fn message() -> String {
    "Hi guys".to_owned()
}