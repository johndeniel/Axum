// This Rust code sets up an asynchronous runtime using Tokio and loads environment variables
// using the `dotenv` crate. The `dotenv` macro is used to retrieve the value of the `DATABASE_URL`
// environment variable. The retrieved database URI is then passed as a parameter to the `run`
// function from the `application` module, which likely handles database-related operations.

// Since this is an asynchronous application, it is important to ensure that the code within the
// `run` function and other asynchronous tasks properly handle errors and avoid blocking operations
// to maintain responsiveness. Additionally, it's essential to handle any potential failures when
// loading environment variables using `dotenv` to prevent unexpected behavior during runtime.

// Overall, this code demonstrates a standard approach for loading environment variables and
// setting up an asynchronous runtime in a Rust application.

use application::run;
use dotenvy::dotenv;
use dotenvy_macro::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_uri = dotenv!("DATABASE_URL");
    run(database_uri).await;
}