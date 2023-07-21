// Import required modules and dependencies
use axum::{Extension, Json}; // Import the `Extension` type from the `axum` crate
use sea_orm::{DatabaseConnection, Set, ActiveModelTrait}; // Import required types from the `sea_orm` crate
use crate::database::tasks; // Import the `tasks` module from the `database` module of the current crate
use serde::Deserialize; // Import the `Deserialize` traits from the `serde` crate for deserialization

// Define a struct to represent the data you want to insert
#[derive(Deserialize)]
pub struct RequestTask{
    priority: Option<String>,
    title: String,
    description: Option<String>,
}

// Define an asynchronous function named `create_task` that takes a `DatabaseConnection` as an extension.
// The function is responsible for creating a new task in the database.
pub async fn create_task(Extension(database): Extension<DatabaseConnection>, Json(request_task): Json<RequestTask>) {
    // Define the properties of the new task as an `ActiveModel` with default values.
    // In this example, we set the priority, title, and description fields of the task.
    let new_task = tasks::ActiveModel {
        priority: Set(request_task.priority), // Set the priority field with a value of "B"
        title: Set(request_task.title), // Set the title field with a value of "SeaORM"
        description: Set(request_task.description), // Set the description field with a value of "SeaORM"
        ..Default::default() // Use default values for any other fields not explicitly set
    };

    // Save the new task to the database using the provided `DatabaseConnection`.
    // The operation is asynchronous, so it is awaited here.
    // If an error occurs during the save operation, it will be unwrapped with `.unwrap()`.
    // In production code, it is recommended to handle errors in a more robust way.
    let result = new_task.save(&database).await.unwrap();

    // Print the result (usually a `Model` representing the newly created task) using `dbg!`.
    // `dbg!` is a macro that prints the debug representation of its argument along with the file and line number.
    // This is helpful for debugging and inspecting values during development.
    dbg!(result);
}