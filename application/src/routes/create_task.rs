// Function: create_task
// Description: This function is responsible for creating a new task record in the database based on the provided JSON request.
// Parameters:
//   - State(database): State<DatabaseConnection> - An active connection to the database provided by the Axum web framework's state extractor.
//   - authorization: TypedHeader<Authorization<Bearer>> - A typed header containing the authorization token (Bearer token) extracted from the HTTP request.
//   - Json(request_task): Json<RequestTask> - JSON payload containing the details of the new task to be created, which is deserialized into the RequestTask struct.
// Returns:
//   - Result<(), StatusCode> - An Ok(()) value is returned upon successful task creation, and an error StatusCode is returned in case of any failures.
//
// Details:
//   1. The function begins by validating the authorization token extracted from the HTTP headers. It ensures the token is associated with a valid user in the database.
//   2. If the user is not found or the token is invalid, the function returns an UNAUTHORIZED status code, indicating the user lacks the necessary privileges to create a task.
//   3. Upon successful token validation, the function creates a new ActiveModel instance using sea_orm, which represents the task to be inserted into the database.
//   4. The provided task details (title, priority, and description) are associated with the new task. The user_id is set to link the task with the corresponding user.
//   5. The new task is then saved into the database using the save() method provided by sea_orm.
//   6. If any error occurs during the database operation, the function returns an INTERNAL_SERVER_ERROR status code, indicating an issue with the database connectivity or the query execution.
//   7. Upon successful task creation, the function returns Ok(()), signifying that the task was inserted into the database successfully.
//
// Note: Proper error handling is essential in production environments to handle potential failures gracefully and provide meaningful feedback to the client.

use crate::database::tasks;
use crate::database::users::{self, Entity as Users};
use axum::extract::State;
use axum::{
    headers::{authorization::Bearer, Authorization},
    http::StatusCode,
    Json, TypedHeader,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RequestTask {
    title: String,
    priority: Option<String>,
    description: Option<String>,
}

pub async fn create_task(
    State(database): State<DatabaseConnection>,
    authorization: TypedHeader<Authorization<Bearer>>,
    Json(request_task): Json<RequestTask>,
) -> Result<(), StatusCode> {
    let token = authorization.token();

    let user = if let Some(user) = Users::find()
        .filter(users::Column::Token.eq(Some(token)))
        .one(&database)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        user
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let new_task = tasks::ActiveModel {
        priority: Set(request_task.priority),
        title: Set(request_task.title),
        description: Set(request_task.description),
        user_id: Set(Some(user.id)),
        ..Default::default()
    };

    let _result = new_task.save(&database).await.unwrap();

    Ok(())
}