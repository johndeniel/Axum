// Import necessary libraries and modules
use axum::{Json, Extension, http::StatusCode}; // Import required Axum traits and types
use serde::{Deserialize, Serialize}; // Import serde traits for JSON serialization/deserialization
use sqlx::PgPool; // Import SQLx Postgres pool for database connection

// Define a struct to represent the incoming JSON request for creating a user account
#[derive(Deserialize)]
pub struct RequestUser {
    username: String,
    password: String,
}

// Define a struct to represent the JSON response of a newly created user account
#[derive(Serialize)]
pub struct ResponseUser {
    username: String,
    id: i32,
    token: String,
}

// Define a struct to represent a user model fetched from the database using SQLx
#[derive(sqlx::FromRow)]
pub struct UserModel {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub token: Option<String>,
}

use crate::query::hash;

// Async function to handle creating a new user account
pub async fn creating_account(
    Extension(database): Extension<PgPool>, // Get the Postgres database pool from the request context
    Json(request_user): Json<RequestUser>, // Deserialize the JSON request into RequestUser struct
) -> Result<Json<ResponseUser>, StatusCode> {
    // Create a new user in the database using SQLx and retrieve the newly created user
    let new_user = sqlx::query_as::<_, UserModel>(
        "INSERT INTO users (username, password, token) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(request_user.username) // Bind the username from the request
    .bind(hash::hash_password(request_user.password)?) // Bind the password from the request
    .bind(Some("n2093487gjlaufrseh".to_owned())) // Set a default token value and bind it to the query
    .fetch_one(&database) // Execute the query and retrieve the newly created user
    .await // Await the query result
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?; // Handle any database errors and return internal server error if they occur

    // Return a JSON response containing the newly created user's details
    Ok(Json(ResponseUser {
        username: new_user.username,
        id: new_user.id,
        token: new_user.token.unwrap_or_else(|| "".to_owned()), // Use the user's token or an empty string if it's None
    }))
}