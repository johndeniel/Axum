// Import required external crates and modules
use axum::{
    extract::Query, // Importing the Query extractor from the Axum framework
    http::StatusCode, // Importing the StatusCode enumeration for HTTP status codes
    Json, // Importing the Json type for serializing and deserializing JSON data
    Extension, // Importing the Extension type used for handling extensions in Axum
};
use serde::{Deserialize, Serialize}; // Importing serde for JSON (de)serialization
use sqlx::{
    postgres::PgPool, // Importing the PostgreSQL database pool for SQLx
    FromRow, // Importing the FromRow trait used to convert query results into structs
};

// Define a struct `ResponseTask` to represent a task retrieved from the database
#[derive(Serialize, FromRow)] // Implement the Serialize and FromRow traits for this struct
pub struct ResponseTask {
    id: i32, // Task ID
    title: String, // Task title
    priority: Option<String>, // Optional priority field for the task
    description: Option<String>, // Optional description field for the task
}

// Define a struct `GetTasksQueryParams` to represent query parameters for filtering tasks
#[derive(Deserialize)] // Implement the Deserialize trait for this struct
pub struct GetTasksQueryParams {
    priority: Option<String>, // Optional priority field for filtering tasks
}

// Define an async function `filtered_tasks` to handle the API endpoint for fetching filtered tasks
pub async fn filtered_tasks(
    query_params: Query<GetTasksQueryParams>, // Extract the query parameters from the request
    Extension(database): Extension<PgPool>, // Extract the database pool from the request extensions
) -> Result<Json<Vec<ResponseTask>>, StatusCode> {
    // The function returns a Result containing a Json response or a StatusCode for error handling

    // Extract the priority query parameter from the query parameters
    let priority: String = query_params.priority.clone().unwrap_or_else(|| "".to_string());

    // Define the SQL query based on whether the priority is specified or not
    let query: &str = if priority.is_empty() {
        "SELECT * FROM tasks WHERE priority IS NULL"
    } else {
        "SELECT * FROM tasks WHERE priority = $1"
    };

    // Fetch tasks from the database using the provided query and priority
    match fetch_all(&database, query, &priority).await {
        Ok(query_result) => {
            // If the query is successful, map the query results to ResponseTask structs and create a Vec<ResponseTask>
            let response_tasks: Vec<ResponseTask> = query_result
                .into_iter()
                .map(|db_task| ResponseTask {
                    id: db_task.id,
                    priority: db_task.priority,
                    title: db_task.title,
                    description: db_task.description,
                })
                .collect();

            // Return the Vec<ResponseTask> as a JSON response
            Ok(Json(response_tasks))
        }
        Err(_) => {
            // If there's an error during query execution, return an internal server error status code
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// Define an async function `fetch_all` to execute the SQL query and fetch tasks from the database
async fn fetch_all(
    database: &PgPool, // Database pool reference to execute the query
    query: &str, // The SQL query string to execute
    priority: &str, // The priority parameter to bind to the query (used in prepared statements)
) -> Result<Vec<ResponseTask>, sqlx::Error> {
    // The function returns a Result containing a Vec<ResponseTask> or an sqlx::Error for error handling

    // Execute the SQL query as a prepared statement and fetch all rows as ResponseTask structs
    match sqlx::query_as::<_, ResponseTask>(query)
        .bind(priority) // Bind the priority parameter to the query
        .fetch_all(database) // Execute the query using the provided database pool
        .await
    {
        Ok(row) => Ok(row), // If the query is successful, return the fetched rows as a Vec<ResponseTask>
        Err(_) => {
            // If there's an error during query execution, return a RowNotFound error
            Err(sqlx::Error::RowNotFound)
        }
    }
}