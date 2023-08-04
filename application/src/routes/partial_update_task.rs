// Asynchronous function `partial_update` used to update a task partially in the database.
// The function takes the task ID as a path parameter, the database connection as state, and a JSON payload `request_task` containing the fields to update.
// The payload `request_task` is deserialized into the `RequestTask` structure using serde.

// The `RequestTask` structure represents the partial updates that can be applied to a task.
// Each field in the structure is wrapped in an `Option`, allowing partial updates by setting only the fields provided in the JSON payload.
// The `serde` annotations with `skip_serializing_if = "Option::is_none"` ensure that the `None` values are not serialized, making it possible to selectively update fields.

// The function starts by querying the database to find the task with the given `task_id`.
// If the task is found, it converts it to an active model using `into_active_model()` from SeaORM.
// If the task is not found, the function returns a `StatusCode::NOT_FOUND` to indicate that the task does not exist.

// The function then checks each field in the `request_task` and updates the corresponding field in the `db_task` if a new value is provided.
// The `Set` method from SeaORM is used to set the new value in the active model.

// After applying the updates to `db_task`, the function performs the update operation on the database using the `Tasks::update` method from SeaORM.
// The `filter` method is used to specify the condition for updating only the task with the given `task_id`.

// If the update operation is successful, the function returns an `Ok(())` to indicate a successful update.
// If any error occurs during the database operation, the function returns a `StatusCode::INTERNAL_SERVER_ERROR`.

// Note: The code assumes the existence of the `tasks` module within the `database` module and the corresponding fields in the `Task` entity structure.
// Additionally, the `RequestTask` structure is designed to allow partial updates with flexible field selection in the JSON payload.

use crate::database::tasks;
use crate::database::tasks::Entity as Tasks;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::{
    prelude::DateTimeWithTimeZone, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel,
    QueryFilter, Set,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RequestTask {
    #[serde(
        default,                                    // <- important for deserialization
        skip_serializing_if = "Option::is_none",    // <- important for serialization
        with = "::serde_with::rust::double_option",
    )]
    pub priority: Option<Option<String>>,
    pub title: Option<String>,
    #[serde(
        default,                                    // <- important for deserialization
        skip_serializing_if = "Option::is_none",    // <- important for serialization
        with = "::serde_with::rust::double_option",
    )]
    pub completed_at: Option<Option<DateTimeWithTimeZone>>,
    #[serde(
        default,                                    // <- important for deserialization
        skip_serializing_if = "Option::is_none",    // <- important for serialization
        with = "::serde_with::rust::double_option",
    )]
    pub description: Option<Option<String>>,
    #[serde(
        default,                                    // <- important for deserialization
        skip_serializing_if = "Option::is_none",    // <- important for serialization
        with = "::serde_with::rust::double_option",
    )]
    pub deleted_at: Option<Option<DateTimeWithTimeZone>>,
}

pub async fn partial_update(
    Path(task_id): Path<i32>,
    State(database): State<DatabaseConnection>,
    Json(request_task): Json<RequestTask>,
) -> Result<(), StatusCode> {
    let mut db_task = if let Some(task) = Tasks::find_by_id(task_id)
        .one(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        task.into_active_model()
    } else {
        return Err(StatusCode::NOT_FOUND);
    };

    if let Some(priority) = request_task.priority {
        db_task.priority = Set(priority);
    }

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

    Tasks::update(db_task)
        .filter(tasks::Column::Id.eq(task_id))
        .exec(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}