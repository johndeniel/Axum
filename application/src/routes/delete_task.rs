// Asynchronous function to delete a task from the database.
// The function takes a task ID as a path parameter and a `QueryParams` structure as a query parameter.
// The `Path` and `State` extractors from Axum are used to obtain the task ID and the database connection, respectively.
// The `Query` extractor is used to get the optional `soft` parameter from the request's query string.

// The `QueryParams` structure is derived from `serde::Deserialize` to allow parsing of query parameters from the request.

// The function first checks if the `soft` parameter is true. If true, it performs a soft delete by updating the task's `deleted_at` field with the current UTC time.
// The SeaORM library is used to find the task by its ID and perform the update operation.
// If the task with the given ID is not found, the function returns a `StatusCode::NOT_FOUND`.

// If the `soft` parameter is false, a hard delete is performed by using SeaORM's `delete_many` method to remove the task from the database.
// The `filter` method is used to specify the deletion condition based on the task ID.

// The function returns an `Ok(())` upon successful deletion and a `StatusCode::INTERNAL_SERVER_ERROR` if any error occurs during the database operation.

// Note: The code assumes that the `tasks` module with the `Entity` trait and the `Column` enumeration exists in the `database` module.
// Additionally, the `Task` struct must have a `deleted_at` field to support soft deletion.

use crate::database::tasks::{self, Entity as Tasks};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, Set};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct QueryParams {
    soft: bool,
}

pub async fn delete_task(
    Path(task_id): Path<i32>,
    State(database): State<DatabaseConnection>,
    Query(query_params): Query<QueryParams>,
) -> Result<(), StatusCode> {

    if query_params.soft {
        let mut task = if let Some(task) = Tasks::find_by_id(task_id)
            .one(&database)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        {
            task.into_active_model()
        } else {
            return Err(StatusCode::NOT_FOUND);
        };

        let now = chrono::Utc::now();

        task.deleted_at = Set(Some(now.into()));
        Tasks::update(task)
            .exec(&database)
            .await
            .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;
    } else {
        Tasks::delete_many()
            .filter(tasks::Column::Id.eq(task_id))
            .exec(&database)
            .await
            .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    Ok(())
}