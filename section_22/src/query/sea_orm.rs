// Import necessary modules and libraries
use axum::{extract::Path, Extension, http::StatusCode};
use sea_orm::{DatabaseConnection, EntityTrait, IntoActiveModel};
use crate::database::tasks::Entity as Tasks;

// Function to perform an atomic update on a task and return the result as JSON
pub async fn hard_delete(
    Path(id): Path<i32>,                           // Extract the 'id' from the path of the incoming request
    Extension(database): Extension<DatabaseConnection>,  // Extract the 'DatabaseConnection' extension from the request
) -> Result<(), StatusCode> {

    // Attempt to find a task in the database with the specified 'id'
    let task = if let Some(task) = Tasks::find_by_id(id)  // Find a task by its id
        .one(&database)                              // Execute the query using the provided database connection
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)? {  // Handle potential errors and map them to an appropriate HTTP status code
            task.into_active_model()               // Convert the found task entity into an active model that can be used for updating
        } else {
            return Err(StatusCode::NOT_FOUND);  // If no task is found with the given 'id', return a 'NOT_FOUND' status code
        };

    // Delete the found task from the database
    Tasks::delete(task)
        .exec(&database)                           // Execute the delete operation using the provided database connection
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;  // Handle potential errors and map them to an appropriate HTTP status code

    Ok(())  // Return success, indicating that the update was performed successfully
}