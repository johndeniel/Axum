// Import the `run` function from the `section_15` module (assumed to be in the same crate).
use section_16::run;

// Define the `main` function, which is the entry point of the program.
// The `main` function is asynchronous and uses the `tokio::main` attribute to run the async code.
#[tokio::main]
async fn main() {
    // Call the `run` function.
    // The `run` function is expected to be asynchronous, and we await its completion here.
    // This ensures that the async code within `run` is fully executed before the program exits.
    run().await;
}