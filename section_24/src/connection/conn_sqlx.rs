// Import necessary modules from the sqlx crate
use sqlx::{postgres::PgPoolOptions, PgPool};

// Function to establish a database connection pool
// This function returns a Result containing the connection pool or an error of type sqlx::Error
pub async fn establish() -> Result<PgPool, sqlx::Error> {
    // 1) Create a connection pool configuration with a maximum of 5 connections
    let pool = PgPoolOptions::new()
        .max_connections(5)

        // 2) Connect to the PostgreSQL database using the provided connection string
        // The connection string follows the format: "postgres://username:password@hostname/database_name"
        .connect("postgres://postgres:password@localhost/postgres")
        .await?; // The `await?` here will propagate any error that occurs during the connection process

    // 3) If the connection is successful, return the connection pool in the Ok variant of the Result
    Ok(pool)
}