// This Rust code provides JWT-related functionality for an Axum-based application. JSON Web Tokens
// (JWT) are utilized for user authentication and session management. The code includes two functions:
// `create_jwt` and `is_valid`.

// The `Claims` struct represents the payload data contained in the JWT. It includes an expiration
// (`exp`) and an issued-at timestamp (`iat`). The `Claims` struct is derived for serialization and
// deserialization using serde attributes.

// The `create_jwt` function generates a new JWT based on the current time. It sets the expiration
// time (`exp`) to 30 seconds from the current timestamp (`iat`). The JWT is then created using the
// `jsonwebtoken` crate with the provided `JWT_SECRET`. If successful, the JWT string is returned. If
// an error occurs during token creation, it returns an `INTERNAL_SERVER_ERROR` status code.

// The `is_valid` function verifies the validity of a given JWT string. It uses the `jsonwebtoken`
// crate to decode the token with the provided `JWT_SECRET`. If the token is valid and not expired,
// it returns `Ok(true)`. If the token is invalid or expired, it returns an `AppError` containing the
// appropriate error message and the corresponding status code. The `AppError` is likely a custom error
// type specific to this application.

// Both functions access the `JWT_SECRET` environment variable using `dotenv!` macro from the
// `dotenvy_macro` crate. This ensures that the secret key is not hard-coded in the code and is instead
// provided securely through an environment variable.

// It's essential to ensure that the JWT_SECRET environment variable is correctly set during runtime,
// and the `jsonwebtoken` crate's validation settings are appropriate for your application's security
// requirements. Additionally, consider using appropriate JWT token lifetimes and refresh mechanisms
// based on your application's use case.

// Overall, this code showcases a standard approach to implementing JWT generation and validation for
// an Axum-based application, enhancing security and enabling session management with JSON Web Tokens.

use axum::http::StatusCode;
use chrono::{Duration, Utc};
use dotenvy_macro::dotenv;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use super::app_error::AppError;

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    exp: usize,
    iat: usize,
}

pub fn create_jwt() -> Result<String, StatusCode> {
    let mut now = Utc::now();
    let iat = now.timestamp() as usize;
    let expires_in = Duration::seconds(30);
    now += expires_in;
    let exp = now.timestamp() as usize;
    let claim = Claims { exp, iat };
    let secret: &'static str = dotenv!("JWT_SECRET");
    let key = EncodingKey::from_secret(secret.as_bytes());
    encode(&Header::default(), &claim, &key).map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn is_valid(token: &str) -> Result<bool, AppError> {
    let secret: &'static str = dotenv!("JWT_SECRET");
    let key = DecodingKey::from_secret(secret.as_bytes());
    decode::<Claims>(token, &key, &Validation::new(Algorithm::HS256)).map_err(
        |error| match error.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => AppError::new(
                StatusCode::UNAUTHORIZED,
                "Your session has expired, please login again",
            ),
            _ => AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong, please try again",
            ),
        },
    )?;
    Ok(true)
}