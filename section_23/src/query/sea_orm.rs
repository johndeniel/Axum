// Import necessary modules and items from other files
use crate::database::tasks::{
    self, 
    Entity as Tasks
};
use axum::{
    extract::{
        Path, // Extracts the path parameter from the request URL
        Query // Extracts the query parameter from the request URL
    },
    http::StatusCode, // Provides HTTP status codes
    Extension // Extracts shared state (DatabaseConnection) from the application state
};
use sea_orm::{
    ColumnTrait, // Trait for column operations in SeaORM entities
    DatabaseConnection, // Represents a connection to the database
    EntityTrait, // Trait for entity operations in SeaORM
    IntoActiveModel, // Trait for converting an entity into an active model
    QueryFilter, // Used to filter queries in SeaORM
    Set // Used to set values of columns in SeaORM entities
};
use serde::Deserialize; // For deserializing request query parameters


// Struct to hold the query parameters for the request
// It is derived from Deserialize to automatically parse incoming query parameters
#[derive(Deserialize)]
pub struct QueryParams {
    soft: bool, // A boolean flag indicating whether to perform a soft delete or not
}

// The main function that handles the soft delete or hard delete based on the query parameter
pub async fn soft_delete(
    Path(id): Path<i32>, // Extracts the "id" path parameter as an integer
    Extension(database): Extension<DatabaseConnection>, // Gets the shared DatabaseConnection from the application state
    Query(query_params): Query<QueryParams>, // Extracts the query parameters from the request and deserializes into QueryParams struct
) -> Result<(), StatusCode> {
    // Check if the soft delete flag is set
    if query_params.soft {
        // If it is a soft delete, find the task by its id in the database using SeaORM
        let mut task = if let Some(task) = Tasks::find_by_id(id)
            .one(&database)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)? // Return an internal server error if there is a database query error
        {
            task.into_active_model() // Convert the SeaORM task entity into an active model
        } else {
            return Err(StatusCode::NOT_FOUND); // If the task with the given id is not found, return a not found status code
        };

        // Update the "deleted_at" column of the task to the current timestamp (soft delete)
        let now = chrono::Utc::now();
        task.deleted_at = Set(Some(now.into()));

        // Perform the update operation in the database using SeaORM
        Tasks::update(task)
            .exec(&database)
            .await
            .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?; // Return an internal server error if there is a database update error
    } else {
        // If it is a hard delete, delete the task from the database using SeaORM
        Tasks::delete_many()
            .filter(tasks::Column::Id.eq(id)) // Specify the condition for deletion (matching the task id)
            .exec(&database)
            .await
            .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?; // Return an internal server error if there is a database delete error
    }

    Ok(()) // Return Ok(()) if the operation is successful
}