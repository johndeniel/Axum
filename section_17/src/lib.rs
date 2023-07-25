// Import the `router` module from the `router.rs` file (assumed to be in the same directory).
mod router;
mod connection;
mod query;
mod database;
use tokio::signal::ctrl_c;

// Define an asynchronous function named `run` that takes a reference to a `database_uri` string.
pub async fn run() {
    // Create the SQLX database connection
    let conn_sqlx: sqlx::Pool<sqlx::Postgres> = connection::conn_sqlx::establish()
        .await
        .expect("Failed to establish the database connection");

    // Create the SeaORM database connection
    let conn_sea_orm = connection::conn_seaorm::establish()
        .await
        .expect("Failed to establish the database connection");

    // Initialize the application routes using the `router::routes::app` function,
    // passing the appropriate database connections.
    // These operations are asynchronous, so they await the results.
    let app1 = router::sqlx::app(conn_sqlx).await;
    let app2 = router::sea_orm::app(conn_sea_orm).await;

    // Define the addresses for both servers
    let addr1: std::net::SocketAddr = "0.0.0.0:3000".parse().unwrap();
    let addr2: std::net::SocketAddr = "0.0.0.0:3001".parse().unwrap();

    // Spawn two tasks to run both servers concurrently
    let server1 = axum::Server::bind(&addr1).serve(app1.into_make_service());
    let server2 = axum::Server::bind(&addr2).serve(app2.into_make_service());

    // Await both servers concurrently using `tokio::select!`.
    // This allows the program to run both servers at the same time.
    tokio::select! {
        _ = server1 => {},
        _ = server2 => {},
        _ = ctrl_c() => println!("Ctrl-C received, shutting down servers..."),
    }
}