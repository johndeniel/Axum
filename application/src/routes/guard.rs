// Asynchronous function `guard` that acts as a middleware for authentication and authorization in an Axum web application.
// The middleware performs the following tasks:
// 1. Retrieves the database connection from the shared application state.
// 2. Extracts the JWT (JSON Web Token) from the `Authorization` header and stores it in a local variable `token`.
// 3. Queries the database to find a user based on the JWT token.
// 4. If a user is found, it indicates that the token is valid, and the user object is stored in a local variable `user`.
// 5. The token is also validated separately using a custom `is_valid` function to further obfuscate whether the token is wrong or expired.
// 6. If the token is invalid, an `AppError` is raised with an internal server error status.
// 7. If the user is not found, indicating the token is invalid or the user associated with the token does not exist, an `AppError` is raised with an unauthorized status.
// 8. If the token and user are valid, the user object is added to the request extensions, making it available to downstream handlers.
// 9. The middleware then proceeds to execute the next middleware or the main request handler (`next.run(request).await`) and returns its response.

// The function takes several parameters:
// - `State(database)`: The database connection is retrieved from the shared application state, allowing access to the database within the middleware.
// - `TypedHeader(token)`: The JWT token is extracted from the `Authorization` header and stored in the `token` variable.
// - `request: Request<T>`: The HTTP request object passed to the middleware. The user object will be stored in its extensions for downstream handlers to use.
// - `next: Next<T>`: The next middleware or the main request handler that will be executed after this middleware.

// The function is generic over the type `T`, allowing it to be used with different types of request bodies.

// Note: The code assumes the existence of the `users` module within the `database` module and the corresponding fields in the `User` entity structure.
// It also assumes the presence of utility functions for handling custom errors (`AppError`) and JWT validation (`is_valid`).
// The middleware checks the validity of the token both by querying the database and by calling the `is_valid` function, enhancing security.
// The function returns a `Result<Response, AppError>`, where `Response` is the HTTP response type and `AppError` represents custom application-specific errors.

use crate::{
    database::users::{self, Entity as Users},
    utils::{app_error::AppError, jwt::is_valid},
};
use axum::{
    extract::State,
    headers::{authorization::Bearer, Authorization},
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    TypedHeader,
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

pub async fn guard<T>(
    State(database): State<DatabaseConnection>,
    TypedHeader(token): TypedHeader<Authorization<Bearer>>,
    mut request: Request<T>,
    next: Next<T>,
) -> Result<Response, AppError> {
    let token = token.token().to_owned();
    let user = Users::find()
        .filter(users::Column::Token.eq(Some(token.clone())))
        .one(&database)
        .await
        .map_err(|_error| {
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
        })?;
    is_valid(&token)?; // Validating token after getting from the database to obsfucate that the token is wrong. Feel free to move up if you are not worried about that.

    let Some(user) = user else {return Err(AppError::new(StatusCode::UNAUTHORIZED, "You are not authorized, please log in or create account"))};

    request.extensions_mut().insert(user);

    Ok(next.run(request).await)
}