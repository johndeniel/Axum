// This Rust code sets up an Axum-based web server, integrating the `sea_orm` crate for database
// connectivity and management. The code is organized into three modules: `database`, `routes`, and
// `utils`, each serving a distinct purpose.

// The `run` function is the entry point for the application. It takes a `database_uri` parameter,
// which represents the connection string for the database. Within the function, it establishes a
// connection to the specified database using `Database::connect` from the `sea_orm` crate. The
// connection is awaited, and the application panics if the connection attempt fails.

// After the successful database connection, the code proceeds to create the application's routes
// using the `create_routes` function from the `routes` module. This function is likely responsible
// for defining and configuring the various routes and their associated handlers.

// The application is then bound to listen on the address "0.0.0.0:3000" using `axum::Server::bind`.
// `axum` is a powerful web framework for Rust. The application is served by converting it into a
// `Service` using the `into_make_service` method.

// The whole process is executed asynchronously, thanks to the `async` and `await` keywords used in
// the function. If the server fails to start, the application panics and terminates with an error.

// It's important to ensure that the database connection details are correctly provided in the
// `database_uri`, and the `create_routes` function handles route configuration properly. Additionally,
// considering the asynchronous nature of the application, it's crucial to handle errors gracefully
// and avoid blocking operations to maintain responsiveness.

// Overall, this code showcases a structured approach to building an asynchronous web server with Axum,
// utilizing `sea_orm` for database interactions, and employing modularity to organize the codebase.

mod database;
mod routes;
mod utils;
use sea_orm::Database;

pub async fn run(database_uri: &str) {
    let database = Database::connect(database_uri).await.unwrap();
    let app = routes::create_routes(database).await;

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}