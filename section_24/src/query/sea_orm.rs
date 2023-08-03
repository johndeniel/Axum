// Import necessary modules from external crates
use axum::{
    http::StatusCode, 
    Extension, 
    Json,
};
use sea_orm::{
    DatabaseConnection, 
    Set, 
    ActiveModelTrait,
};
use serde::{Deserialize, Serialize};

// Import the 'users' module from the 'database' module of the crate
use crate::database::users;

// Define a request structure using serde for deserialization
#[derive(Deserialize)]
pub struct RequestUser {
    username: String,
    password: String,
}

// Define a response structure using serde for serialization
#[derive(Serialize)]
pub struct ResponseUser {
    username: String,
    id: i32,
    token: String,
}

// Define an asynchronous function called 'creating_account'
// This function is an HTTP handler for creating a new user account
pub async fn creating_account(
    // The function takes two parameters:
    // 1. 'database' - an extension of type 'DatabaseConnection'
    //    This parameter will be provided by the Axum framework using its middleware system.
    //    It enables the function to access the database connection throughout the request's lifetime.
    Extension(database): Extension<DatabaseConnection>, 
    // 2. 'request_user' - a JSON payload received from the client and automatically deserialized
    //    into the 'RequestUser' structure using the 'Json' extractor provided by Axum.
    Json(request_user): Json<RequestUser>,
) -> Result<Json<ResponseUser>, StatusCode> {

    // Create a new 'ActiveModel' instance using the 'users' module's 'ActiveModel' struct.
    // This represents a new user record to be saved in the database.
    let new_user = users::ActiveModel {
        // Set the 'username' field with the value provided in the request payload.
        username: Set(request_user.username),
        // Set the 'password' field with the value provided in the request payload.
        password: Set(request_user.password),
        // Set the 'token' field with a default value of "n2093487gjlaufrseh" wrapped in an Option.
        token: Set(Some("n2093487gjlaufrseh".to_owned())),
        // Use '..Default::default()' to set any remaining fields with their default values.
        ..Default::default()
    }
    // Save the new user record to the database using the provided database connection.
    // This is an asynchronous operation, so it awaits the result using '.await'.
    // If there's an error during saving, the closure after 'map_err' returns an 'INTERNAL_SERVER_ERROR' status code.
    .save(&database)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // If the user was successfully saved, construct the response with the newly created user's data.
    // The 'unwrap' method is used here because we expect these fields to be present in the newly created user.
    Ok(Json(ResponseUser {
        // Extract the 'username' field from the saved user record and set it in the response.
        username: new_user.username.unwrap(),
        // Extract the 'id' field from the saved user record and set it in the response.
        id: new_user.id.unwrap(),
        // Extract the 'token' field from the saved user record and unwrap its inner value (String) to set it in the response.
        token: new_user.token.unwrap().unwrap(),
    }))
}