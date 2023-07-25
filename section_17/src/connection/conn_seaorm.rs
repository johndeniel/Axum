use sea_orm::{Database, DatabaseConnection};

use dotenvy::dotenv;                // Importing the "dotenvy" crate to load environment variables from a .env file.
use dotenvy_macro::dotenv;          // Importing the procedural macro for "dotenvy" to be used below.

// Define an asynchronous function named `connect` that takes a reference to a `database_uri` string
// and returns a `Database` connection wrapped in a `Result`.
pub async fn establish() -> Result<DatabaseConnection, sea_orm::DbErr> {
    // Load environment variables from the '.env' file if present
    dotenv().ok();

    // Get the value of the 'DATABASE_URL' environment variable defined in the '.env' file
    // Note: The 'dotenv!' macro is used to fetch the value of 'DATABASE_URL'.
    // It requires the 'dotenvy_macro' crate, which helps fetch environment variables at compile time.
    // The actual value will be stored in the 'database_uri' variable.
    let database_uri = dotenv!("DATABASE_URL");

    // Connect to the database using the provided `database_uri`.
    let database = Database::connect(database_uri).await.unwrap();

    Ok(database)
}