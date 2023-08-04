// The following code sets up the routing for an Axum web application with various endpoints to handle tasks and users.
// It also demonstrates how to use middleware for authentication (`guard`) and custom JSON extractors.

// Import required modules and libraries.
// - `create_task`, `custom_json_extractor`, `delete_task`, `get_tasks`, `guard`, `hello_world`, `partial_update_task`, `update_tasks`, `users`, `validate_with_serde`: Import the modules containing the respective endpoint handlers for different routes.
// - `axum`: Import the `axum` web framework and its various components for handling HTTP requests and routing.
// - `sea_orm::DatabaseConnection`: Import the `DatabaseConnection` type from the SeaORM library to hold the connection to the database.

// Define a structure `AppState` that holds the application's state, including the database connection.
// The structure implements the `FromRef` trait, allowing it to be extracted from request extensions.

// Define an asynchronous function `create_routes` to create and configure the router for the Axum application.
// The function takes a `DatabaseConnection` as a parameter, which will be used to create the application state.
// The function starts by creating an `AppState` from the provided database connection.

// The router is created using the `Router::new()` constructor.

// The various endpoints for different routes are defined using the `route()` method of the router.
// Each route specifies a URL path and associates it with a corresponding handler function.

// Some routes use different HTTP methods (`get`, `post`, `put`, `patch`, `delete`) to handle different types of requests.

// Middleware is applied using the `route_layer()` method, which inserts the `guard` function as middleware to protect certain routes.
// The `guard` function is responsible for authentication and ensures that only authenticated users can access protected routes.
// The `app_state.clone()` is used to provide the `AppState` to the `guard` middleware.

// The application also includes a custom JSON extractor route (`/custom_json_extractor`) that demonstrates how to use a custom JSON extractor.
// The custom extractor is implemented in the `custom_json_extractor` module.

// Each route is associated with a specific handler function (defined in separate modules).
// For example, `post(create_task)` associates the `/tasks` route with the `create_task` function from the `create_task` module.

// The router is returned as the result of the `create_routes` function.

// Note: The code assumes that the various endpoint handler functions (`create_task`, `get_all_tasks`, `get_one_task`, etc.) are implemented in their respective modules (`create_task`, `get_tasks`, etc.).
// It also assumes that the `AppState` structure is defined in the same module and the necessary modules are imported and available in the project's dependencies.
// The `AppState` is used to hold the database connection and can be accessed by middleware and handler functions using request extensions.

mod create_task;
mod custom_json_extractor;
mod delete_task;
mod get_tasks;
mod guard;
mod hello_world;
mod partial_update_task;
mod update_tasks;
mod users;
mod validate_with_serde;

use axum::{
    extract::FromRef,
    middleware,
    routing::{delete, get, patch, post, put},
    Router,
};
use create_task::create_task;
use custom_json_extractor::custom_json_extractor;
use delete_task::delete_task;
use get_tasks::{get_all_tasks, get_one_task};
use guard::guard;
use partial_update_task::partial_update;
use sea_orm::DatabaseConnection;
use update_tasks::atomic_update;
use users::{create_user, login, logout};
use validate_with_serde::validate_with_serde;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub database: DatabaseConnection,
}

pub async fn create_routes(database: DatabaseConnection) -> Router {
    let app_state = AppState { database };
    Router::new()
        .route("/users/logout", post(logout))
        .route("/hello_world", get(hello_world::hello_world))
        .route_layer(middleware::from_fn_with_state(app_state.clone(), guard))
        .route("/validate_data", post(validate_with_serde))
        .route("/custom_json_extractor", post(custom_json_extractor))
        .route("/tasks", post(create_task))
        .route("/tasks", get(get_all_tasks))
        .route("/tasks/:task_id", get(get_one_task))
        .route("/tasks/:task_id", put(atomic_update))
        .route("/tasks/:task_id", patch(partial_update))
        .route("/tasks/:task_id", delete(delete_task))
        .route("/users", post(create_user))
        .route("/users/login", post(login))
        .with_state(app_state)
}
