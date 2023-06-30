use axum::{extract::Query, Json};
use serde::{Deserialize, Serialize};

// Define a struct named `QueryParams` using serde attributes for serialization and deserialization
#[derive(Serialize, Deserialize)]
pub struct QueryParams {
    message: String,
    id: u32,
}

// Define an asynchronous function named `query_params` that takes a `Query` parameter containing `QueryParams`
// This function is expected to be used as a handler for extracting and returning query parameters as JSON
pub async fn query_params(Query(query): Query<QueryParams>) -> Json<QueryParams> {
    // Wrap the extracted `query` parameter in a `Json` response and return it
    Json(query)
}