// Import required modules and libraries.
// - `crate::database::tasks::{self, Entity as Tasks}`: Imports the `tasks` module with its `Entity` trait, which represents the task entity in the database.
// - `axum::extract::{Path, Query, State}, http::StatusCode, Extension, Json`: Imports various extractors, HTTP status code, and JSON response utility from the Axum web framework.
// - `chrono::{DateTime, FixedOffset}`: Imports `DateTime` and `FixedOffset` from the Chrono library for date and time handling.
// - `sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter}`: Imports traits and structures from the SeaORM library for database operations.
// - `serde::{Deserialize, Serialize}`: Imports traits for serialization and deserialization of data.

// Define a structure `ResponseTask` for serializing task data as JSON response.
// This structure includes fields representing the task's ID, title, priority, description, deleted_at, and user_id.

// Define an asynchronous function `get_one_task` to retrieve a single task by its ID.
// The function takes the task ID as a path parameter and the database connection as state.
// It uses SeaORM to query the database for the task with the provided ID, ensuring that the `deleted_at` field is null to avoid returning deleted tasks.
// If the task is found, it constructs a `Json` response containing the `ResponseTask` structure with the task's data and returns it.
// If the task is not found, it returns a `StatusCode::NOT_FOUND`.

// Define a structure `GetTasksQueryParams` for deserializing query parameters from the request.
// The structure includes an optional field `priority`, which represents the priority filter for fetching tasks.

// Define an asynchronous function `get_all_tasks` to retrieve all tasks from the database with optional filtering based on priority.
// The function takes the database connection as an extension and the query parameters as a `GetTasksQueryParams`.
// It constructs a filter condition based on the provided `priority` parameter (if any) using SeaORM's `Condition` struct.
// It then queries the database for tasks matching the filter condition and ensures that the `deleted_at` field is null.
// The function maps the fetched tasks into a `Vec` of `ResponseTask` structures and returns a `Json` response containing the data.

// Note: Both functions use the `async` keyword, indicating that they are asynchronous and can await for database operations.
// Additionally, the code assumes the existence of the `tasks` module within the `database` module and the corresponding fields in the `Task` entity structure.

use crate::database::tasks::{self, Entity as Tasks};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Extension, Json,
};
use chrono::{DateTime, FixedOffset};
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ResponseTask {
    id: i32,
    title: String,
    priority: Option<String>,
    description: Option<String>,
    deleted_at: Option<DateTime<FixedOffset>>,
    user_id: Option<i32>,
}

pub async fn get_one_task(
    Path(task_id): Path<i32>,
    State(database): State<DatabaseConnection>,
) -> Result<Json<ResponseTask>, StatusCode> {
    let task = Tasks::find_by_id(task_id)
        .filter(tasks::Column::DeletedAt.is_null())
        .one(&database)
        .await
        .unwrap();

    if let Some(task) = task {
        Ok(Json(ResponseTask {
            id: task.id,
            title: task.title,
            priority: task.priority,
            description: task.description,
            deleted_at: task.deleted_at,
            user_id: task.user_id,
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

#[derive(Deserialize)]
pub struct GetTasksQueryParams {
    priority: Option<String>,
}

pub async fn get_all_tasks(
    Extension(database): Extension<DatabaseConnection>,
    Query(query_params): Query<GetTasksQueryParams>,
) -> Result<Json<Vec<ResponseTask>>, StatusCode> {
    let mut priority_filter = Condition::all();
    if let Some(priority) = query_params.priority {
        priority_filter = if priority.is_empty() {
            priority_filter.add(tasks::Column::Priority.is_null())
        } else {
            priority_filter.add(tasks::Column::Priority.eq(priority))
        };
    }

    let tasks = Tasks::find()
        .filter(priority_filter)
        .filter(tasks::Column::DeletedAt.is_null())
        .all(&database)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .map(|db_task| ResponseTask {
            id: db_task.id,
            title: db_task.title,
            priority: db_task.priority,
            description: db_task.description,
            deleted_at: db_task.deleted_at,
            user_id: db_task.user_id,
        })
        .collect();

    Ok(Json(tasks))
}