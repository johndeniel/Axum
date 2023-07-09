// Import the `run` function from the `section_12` module or crate
use section_12::run;

// Define the entry point of the program and set up the asynchronous runtime provided by Tokio
#[tokio::main]
async fn main() {
    // Call the `run` function, which executes the logic defined in the `run` function
    // The `await` keyword suspends the execution of the `main` function until `run` completes
    run().await;
}