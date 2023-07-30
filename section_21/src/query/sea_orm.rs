// Import necessary modules and types.
// The `crate` keyword refers to the current crate, and we are importing specific items from the `database` module.
use crate::database::{tasks, tasks::Entity as Tasks};
// The `axum` crate is a lightweight and flexible web framework for Rust.
use axum::{
    extract::Path, // The `extract` module provides extractors to retrieve data from the request.
    http::StatusCode, // The `http` module provides HTTP status codes for responses.
    Extension, // The `Extension` type is used to store and retrieve state data during request handling.
    Json, // The `Json` extractor is used to parse JSON payloads from the request.
};
// The `sea_orm` crate is an ORM (Object-Relational Mapping) library for Rust, used to interact with databases.
use sea_orm::{
    prelude::DateTimeWithTimeZone, // The `prelude` module provides useful traits and types for working with SeaORM.
    ColumnTrait, // The `ColumnTrait` trait defines common methods for columns in a database table.
    DatabaseConnection, // The `DatabaseConnection` type represents a connection to the database.
    EntityTrait, // The `EntityTrait` trait provides common methods for interacting with entities (database tables).
    IntoActiveModel, // The `IntoActiveModel` trait converts a database entity into an active model for updates.
    QueryFilter, // The `QueryFilter` trait is used to build query filters for database queries.
    Set, // The `Set` type is used to set the value of a database column in an active model.
};
// The `serde` crate is used for serializing and deserializing Rust data structures.
use serde::Deserialize;

// Define a request struct used for deserialization of incoming JSON data.
// The `RequestTask` struct represents the data fields that can be partially updated for a task.
#[derive(Deserialize)]
pub struct RequestTask {
    // The `priority` field is represented as an `Option<Option<String>>` to support partial updates.
    // The inner `Option` is used to differentiate between the cases when the field is present in the JSON with a value and when it is not present.
    // The `serde_with::rust::double_option` attribute is used to handle deserialization correctly.
    pub priority: Option<Option<String>>,
    pub title: Option<String>,
    // Similar to `priority`, the `completed_at`, `description`, and `deleted_at` fields are also represented as `Option<Option<T>>` for partial updates.
    pub completed_at: Option<Option<DateTimeWithTimeZone>>,
    pub description: Option<Option<String>>,
    pub deleted_at: Option<Option<DateTimeWithTimeZone>>,
}

// The main function that will handle partial updates to a task based on its ID.
pub async fn partial_update(
    Path(task_id): Path<i32>, // The `Path` extractor is used to get the `task_id` from the request's URL path.
    Extension(database): Extension<DatabaseConnection>, // The `Extension` extractor is used to retrieve the database connection from the request's state.
    Json(request_task): Json<RequestTask>, // The `Json` extractor is used to parse the request's JSON payload into the `RequestTask` struct.
) -> Result<(), StatusCode> {
    // Attempt to find the task with the provided `task_id` in the database.
    // The `Tasks::find_by_id()` method is provided by the SeaORM library and allows querying the database for a single task by its ID.
    // The `one()` method is used to execute the query and retrieve the result, but it returns a `Result<Option<Tasks>, Error>` where `Error` is the database error type.
    // In this case, the `await` keyword is used because the method returns a `Future`, and `await` is used to wait for the result of the asynchronous operation.
    // The `map_err()` method is used to handle potential errors by converting them into a 500 INTERNAL SERVER ERROR status code if there is a database-related error.
    let mut db_task = if let Some(task) = Tasks::find_by_id(task_id)
        .one(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        // Convert the found `task` into an active model using `into_active_model()`.
        // The `task` here is of type `sea_orm::ActiveModel<Tasks>`, which is a representation of the database table entry.
        // The `into_active_model()` method is used to convert this representation into an active model that can be updated and saved back to the database.
        task.into_active_model()
    } else {
        // If the task with the given `task_id` is not found in the database, return a 404 NOT FOUND status code.
        return Err(StatusCode::NOT_FOUND);
    };

    // Check if the `priority` field is present in the request payload.
    // If so, update the `db_task.priority` field with the value from the request.
    // The `Set()` method is provided by the SeaORM library and is used to set the value of a field in the active model.
    if let Some(priority) = request_task.priority {
        db_task.priority = Set(priority);
    }

    // Similar to `priority`, update the other fields if they are present in the request payload.
    if let Some(title) = request_task.title {
        db_task.title = Set(title);
    }

    if let Some(completed_at) = request_task.completed_at {
        db_task.completed_at = Set(completed_at);
    }

    if let Some(description) = request_task.description {
        db_task.description = Set(description);
    }

    if let Some(deleted_at) = request_task.deleted_at {
        db_task.deleted_at = Set(deleted_at);
    }

    // Perform the update operation on the `Tasks` table using the updated `db_task` object.
    // The `Tasks::update()` method is provided by the SeaORM library and is used to create an update query on the `Tasks` table.
    // The `filter()` method is used to specify the condition for which task to update. In this case, we are filtering by the `task_id`.
    // The `exec()` method is used to execute the update query on the database, and `await` is used to wait for the result of the asynchronous operation.
    // If there is an error during the update operation, the `map_err()` method is used to convert it into a 500 INTERNAL SERVER ERROR status code.
    Tasks::update(db_task)
        .filter(tasks::Column::Id.eq(task_id))
        .exec(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Return a successful response with an empty body.
    // The function returns an `Ok(())`, where `()` is an empty tuple, indicating that the response has no body content.
    Ok(())
}