// The following code defines several asynchronous functions to handle user-related operations in an Axum web application with SeaORM as the ORM library.
// The user-related operations include creating a new user, logging in, and logging out.

// The code uses the Axum web framework to handle HTTP requests and SeaORM to interact with the database.

// Import required modules and libraries.
// - `crate::database::users::{Entity as Users, Model}`: Imports the `users` module with its `Entity` trait and `Model` structure, representing the user entity in the database.
// - `utils::jwt::create_jwt`: Imports a function to create JSON Web Tokens (JWT) for user authentication.
// - `axum::{extract::State, http::StatusCode, Extension, Json}`: Imports various extractors, HTTP status code, and JSON response utility from the Axum web framework.
// - `sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, Set}`: Imports traits and structures from the SeaORM library for database operations.
// - `serde::{Deserialize, Serialize}`: Imports traits for serialization and deserialization of data.
// - `bcrypt`: Imports the bcrypt library for password hashing and verification.

// Define a structure `RequestUser` for deserializing incoming JSON data representing user credentials (username and password).
// Define another structure `ResponseUser` for serializing user data (username, ID, and JWT token) into JSON response.

// Define an asynchronous function `create_user` to create a new user in the database.
// The function takes the database connection as state and the JSON payload `request_user` containing the user credentials.
// It first creates a new JWT token using the `create_jwt` function and then hashes the user password using bcrypt.
// The hashed password and the JWT token are set in a new `ActiveModel` representing the user to be saved in the database.
// The `save` method from SeaORM is used to persist the new user data.
// If the user creation is successful, the function returns a JSON response containing the created user's data.
// If any error occurs during the database operation, the function returns a `StatusCode::INTERNAL_SERVER_ERROR`.

// Define an asynchronous function `login` to handle user login.
// The function takes the database connection as state and the JSON payload `request_user` containing the user credentials.
// It queries the database to find a user with the given username.
// If a user is found, it verifies the password by comparing the provided password with the hashed password stored in the database using bcrypt.
// If the password is valid, a new JWT token is created, and the user's token in the database is updated.
// The updated user data is saved in the database using SeaORM.
// If the login is successful, the function returns a JSON response containing the user's data with the updated JWT token.
// If the user is not found or the password is invalid, appropriate error responses (UNAUTHORIZED or NOT_FOUND) are returned.

// Define an asynchronous function `logout` to handle user logout.
// The function takes the database connection as state and an `Extension` containing the user's data obtained from the request extensions.
// It converts the user data from the extension into an `ActiveModel`, sets the user's token to `None` to log them out, and saves the updated user data in the database.
// If the logout is successful, the function returns an empty response.
// If any error occurs during the database operation, the function returns a `StatusCode::INTERNAL_SERVER_ERROR`.

// Two utility functions, `hash_password` and `verify_password`, are defined to handle password hashing and verification using bcrypt.
// These functions wrap the bcrypt library's functionality and return a hashed password or a boolean indicating password verification status.

// Note: The code assumes the existence of the `users` module within the `database` module and the corresponding fields in the `User` entity structure.
// The code also assumes the presence of a utility function for creating JWT tokens (`create_jwt`) and the bcrypt library for password hashing and verification.

use crate::{
    database::users::{Entity as Users, Model},
    utils::jwt::create_jwt,
};
use axum::{extract::State, http::StatusCode, Extension, Json};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    Set,
};
use serde::{Deserialize, Serialize};

use crate::database::users;

#[derive(Deserialize)]
pub struct RequestUser {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct ResponseUser {
    username: String,
    id: i32,
    token: String,
}

pub async fn create_user(
    State(database): State<DatabaseConnection>,
    Json(request_user): Json<RequestUser>,
) -> Result<Json<ResponseUser>, StatusCode> {
    let jwt = create_jwt()?;
    let new_user = users::ActiveModel {
        username: Set(request_user.username),
        password: Set(hash_password(request_user.password)?),
        token: Set(Some(jwt)),
        ..Default::default()
    }
    .save(&database)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ResponseUser {
        username: new_user.username.unwrap(),
        id: new_user.id.unwrap(),
        token: new_user.token.unwrap().unwrap(),
    }))
}

pub async fn login(
    State(database): State<DatabaseConnection>,
    Json(request_user): Json<RequestUser>,
) -> Result<Json<ResponseUser>, StatusCode> {
    let db_user = Users::find()
        .filter(users::Column::Username.eq(request_user.username))
        .one(&database)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(db_user) = db_user {
        if !verify_password(request_user.password, &db_user.password)? {
            return Err(StatusCode::UNAUTHORIZED);
        }

        let new_token = create_jwt()?;
        let mut user = db_user.into_active_model();

        user.token = Set(Some(new_token));

        let saved_user = user
            .save(&database)
            .await
            .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(Json(ResponseUser {
            username: saved_user.username.unwrap(),
            id: saved_user.id.unwrap(),
            token: saved_user.token.unwrap().unwrap(),
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn logout(
    State(database): State<DatabaseConnection>,
    Extension(user): Extension<Model>,
) -> Result<(), StatusCode> {
    let mut user = user.into_active_model();

    user.token = Set(None);

    user.save(&database)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}

fn hash_password(password: String) -> Result<String, StatusCode> {
    bcrypt::hash(password, 14).map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)
}

fn verify_password(password: String, hash: &str) -> Result<bool, StatusCode> {
    bcrypt::verify(password, hash).map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)
}