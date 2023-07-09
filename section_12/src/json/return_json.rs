use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Data {
    name: String,
    age: i8,
    birth_day: String,
}

/// Asynchronous function that returns a JSON response containing the serialized `Data` struct.
///
/// This function demonstrates how to use the `Json` extractor from Axum to automatically serialize
/// the `Data` struct into a JSON response.
///
/// # Returns
///
/// Returns a `Json<Data>` object containing the serialized `Data` struct.

pub async fn get_jason() -> Json<Data>{
    let data = Data {
        name: "John Deniel Dela Peña".to_owned(),
        age: 14,
        birth_day: "October 29".to_owned(),
    };

    Json(data)
}

/* The code above includes a comment describing the purpose and behavior of the get_json function. 
It explains that this function returns a JSON response containing the serialized Data struct. 
The comment also mentions the usage of the Json extractor from Axum, which automatically serializes 
the Data struct into a JSON response. */