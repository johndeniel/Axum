// Import required modules and dependencies
use crate::database::tasks::{self, Entity as Tasks}; // Import the 'tasks' module and its Entity alias
use axum::{ // Import the axum web framework
    extract::Query, // Import 'Query' extractor to get query parameters from requests
    http::StatusCode, // Import 'StatusCode' to handle HTTP status codes
    Extension, // Import 'Extension' to extract an extension from the request
    Json, // Import 'Json' to serialize data as JSON in responses
};

use sea_orm::{ // Import the sea_orm ORM library
    ColumnTrait, // Import traits related to database columns
    Condition, // Import 'Condition' to build query filters
    DatabaseConnection, // Import 'DatabaseConnection' to interact with the database
    EntityTrait, // Import traits related to database entities
    QueryFilter, // Import 'QueryFilter' to build advanced queries
};
use serde::{Deserialize, Serialize}; // Import 'Deserialize' and 'Serialize' traits for data serialization/deserialization using serde

// Define a struct to represent the response format for tasks
#[derive(Serialize)]
pub struct ResponseTask {
    id: i32,
    title: String,
    priority: Option<String>,
    description: Option<String>,
}

// Define a struct to represent the query parameters for filtering tasks
#[derive(Deserialize)]
pub struct GetTasksQueryParams {
    priority: Option<String>, // An optional parameter to filter tasks by priority
}

// The main function to get all tasks and return them as JSON
pub async fn filtered_tasks(
    Extension(database): Extension<DatabaseConnection>, // Extract the 'DatabaseConnection' extension from the request
    Query(query_params): Query<GetTasksQueryParams>, // Extract the 'GetTasksQueryParams' from the request query
) -> Result<Json<Vec<ResponseTask>>, StatusCode> {
    // The function returns a Result containing the JSON response or a StatusCode if an error occurs.

    // Create a filter for the 'priority' column in the database based on the query parameter
    let mut priority_filter = Condition::all();

    // Check if the 'priority' query parameter exists
    if let Some(priority) = query_params.priority {
        // If the 'priority' parameter is present, modify the 'priority_filter' accordingly.

        // If 'priority' is an empty string, filter tasks with a NULL priority in the database
        // Otherwise, filter tasks with a matching 'priority' in the database
        priority_filter = if priority.is_empty() {
            priority_filter.add(tasks::Column::Priority.is_null())
        } else {
            priority_filter.add(tasks::Column::Priority.eq(priority))
        };
    }

    // Query the 'Tasks' entity in the database using the constructed filter
    let tasks = Tasks::find()
        .filter(priority_filter)
        .all(&database) // Execute the query using the provided database connection
        .await // Await the asynchronous query operation
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?; // Handle any potential errors and return an internal server error status code if needed

    // Convert the retrieved database tasks to 'ResponseTask' format and collect them into a vector
    let tasks_response = tasks
        .into_iter()
        .map(|db_task| ResponseTask {
            id: db_task.id,
            title: db_task.title,
            priority: db_task.priority,
            description: db_task.description,
        })
        .collect();

    // Return the tasks_response as a JSON response wrapped in the 'Json' type
    Ok(Json(tasks_response))
}