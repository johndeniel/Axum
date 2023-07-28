// Import required modules and dependencies
use crate::database::tasks::{self, ActiveModel, Entity as Tasks}; // Import the 'tasks' module and its Entity alias
use axum::{ // Import the axum web framework
    extract::Path, // Import 'Query' extractor to get query parameters from requests
    Extension, // Import 'Extension' to extract an extension from the request
    Json, http::StatusCode, // Import 'Json' to serialize data as JSON in responses
};

use sea_orm::{prelude::DateTimeWithTimeZone, DatabaseConnection, Set, EntityTrait, QueryFilter, ColumnTrait};
use serde::Deserialize; // Import 'Deserialize' and 'Serialize' traits for data serialization/deserialization using serde

// Define a struct to represent the response format for tasks
#[derive(Deserialize)]
pub struct ResponseTask {
    pub id: Option<i32>, // An optional field for the task's ID
    pub priority: Option<String>, // An optional field for the task's priority
    pub title: String, // A required field for the task's title
    pub completed_at: Option<DateTimeWithTimeZone>, // An optional field for the task's completion timestamp with timezone
    pub description: Option<String>, // An optional field for the task's description
    pub deleted_at: Option<DateTimeWithTimeZone>, // An optional field for the task's deletion timestamp with timezone
    pub user_id: Option<i32>, // An optional field for the task's associated user ID
    pub is_default: Option<bool>, // An optional field indicating if the task is a default one
}

// The main function to perform an atomic update on a task and return the result as JSON
pub async fn atomic_update(
    Path(id): Path<i32>, // Extract the 'id' from the path of the incoming request
    Extension(database): Extension<DatabaseConnection>, // Extract the 'DatabaseConnection' extension from the request
    Json(response_task): Json<ResponseTask>, // Extract JSON data from the request and deserialize it into 'ResponseTask'
) -> Result<(), StatusCode> {
  
    // Create an 'ActiveModel' instance with the updated task properties received from the JSON data
    let update_task: ActiveModel = tasks::ActiveModel {
        id: Set(id), // Set the task's ID to the value extracted from the request path
        priority: Set(response_task.priority), // Set the task's priority based on the received JSON data (optional)
        title: Set(response_task.title), // Set the task's title based on the received JSON data (required)
        completed_at: Set(response_task.completed_at), // Set the task's completion timestamp based on the received JSON data (optional)
        description: Set(response_task.description), // Set the task's description based on the received JSON data (optional)
        deleted_at: Set(response_task.deleted_at), // Set the task's deletion timestamp based on the received JSON data (optional)
        user_id: Set(response_task.user_id), // Set the task's associated user ID based on the received JSON data (optional)
        is_default: Set(response_task.is_default), // Set the task's 'is_default' property based on the received JSON data (optional)
    };

    // Perform an atomic update on the task in the database
    Tasks::update(update_task)
        .filter(tasks::Column::Id.eq(id)) // Filter the task update to match the provided 'id'
        .exec(&database) // Execute the update operation using the database connection
        .await // Await the completion of the database operation
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?; // Handle any potential errors and map them to an internal server error status code

    Ok(()) // Return 'Ok' indicating that the update was successful with no content in the response
}