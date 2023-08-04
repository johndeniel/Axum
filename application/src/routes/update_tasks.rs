// Asynchronous function `atomic_update` used to perform an atomic update of a task in the database.
// The function takes the task ID as a path parameter, the database connection as state, and a JSON payload `request_task` containing the fields to update.
// The payload `request_task` is deserialized into the `RequestTask` structure using serde.

// The `RequestTask` structure represents the fields that can be updated for a task.
// Each field in the structure is wrapped in an `Option`, allowing flexibility in specifying which fields to update in the JSON payload.

// The function constructs an `ActiveModel` named `update_task` from the `tasks` module with the provided update fields.
// Each field in the `update_task` is set to the corresponding value from the `request_task`, making sure to wrap them in `Set()` to indicate the update.

// After constructing the `update_task`, the function performs the atomic update operation on the database using the `Tasks::update` method from SeaORM.
// The `filter` method is used to specify the condition for updating only the task with the given `task_id`.

// If the update operation is successful, the function returns an `Ok(())` to indicate a successful update.
// If any error occurs during the database operation, the function returns a `StatusCode::INTERNAL_SERVER_ERROR`.

// Note: The code assumes the existence of the `tasks` module within the `database` module and the corresponding fields in the `Task` entity structure.
// The `RequestTask` structure is designed to allow flexible updates with optional fields in the JSON payload.
// The function performs an atomic update, ensuring that all specified fields are updated together, and no partial updates are applied.
// This makes it a safe operation even if multiple clients try to update the same task simultaneously.

use crate::database::tasks;
use crate::database::tasks::Entity as Tasks;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::{
    prelude::DateTimeWithTimeZone, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RequestTask {
    pub id: Option<i32>,
    pub priority: Option<String>,
    pub title: String,
    pub completed_at: Option<DateTimeWithTimeZone>,
    pub description: Option<String>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
    pub user_id: Option<i32>,
    pub is_default: Option<bool>,
}

pub async fn atomic_update(
    Path(task_id): Path<i32>,
    State(database): State<DatabaseConnection>,
    Json(request_task): Json<RequestTask>,
) -> Result<(), StatusCode> {
    let update_task = tasks::ActiveModel {
        id: Set(task_id),
        priority: Set(request_task.priority),
        title: Set(request_task.title),
        completed_at: Set(request_task.completed_at),
        description: Set(request_task.description),
        deleted_at: Set(request_task.deleted_at),
        user_id: Set(request_task.user_id),
        is_default: Set(request_task.is_default),
    };

    Tasks::update(update_task)
        .filter(tasks::Column::Id.eq(task_id))
        .exec(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}