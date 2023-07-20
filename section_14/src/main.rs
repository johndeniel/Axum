// Importing the required external crates
use section_14::run;                 // Assuming this is a custom crate named "section_13" with a run function.
use dotenvy::dotenv;                // Importing the "dotenvy" crate to load environment variables from a .env file.
use dotenvy_macro::dotenv;          // Importing the procedural macro for "dotenvy" to be used below.

// The `#[tokio::main]` attribute macro marks this async function as the entry point of the program,
// and it sets up the asynchronous runtime provided by the `tokio` crate.
#[tokio::main]
async fn main() {
    // Load environment variables from the '.env' file if present
    dotenv().ok();

    // Get the value of the 'DATABASE_URL' environment variable defined in the '.env' file
    // Note: The 'dotenv!' macro is used to fetch the value of 'DATABASE_URL'.
    // It requires the 'dotenvy_macro' crate, which helps fetch environment variables at compile time.
    // The actual value will be stored in the 'database_uri' variable.
    let database_uri = dotenv!("DATABASE_URL");

    // Call the 'run' function from the 'section_13' module with the 'database_uri' as a parameter.
    // This function likely performs some operations using the provided database URI.
    // Since it is an asynchronous function (due to 'async' keyword), it is awaited using 'await'.
    run(database_uri).await;
}