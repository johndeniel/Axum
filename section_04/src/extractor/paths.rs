use axum::extract::Path;

// Define an asynchronous function named `id_extractor` that takes a `Path` parameter containing a `String`
// This function is expected to be used as a handler for extracting the `id` from the path variable
pub async fn id_extractor(Path(id): Path<String>) -> String {
    id
}

// Define an asynchronous function named `absolute_path` that returns a `String`
// This function is expected to be used as a handler for responding to requests targeting the "/path/14" path
pub async fn absolute_path() -> String {
    "14!".to_owned()
}

/* The id_extractor function takes a Path parameter containing a String. This function is intended to be used as 
a handler for extracting the id from the path variable. It returns the extracted id as a String.

The absolute_path function is an asynchronous function that returns a String. This function is expected to be used 
as a handler for responding to requests targeting the "/path/14" path. It returns the string "14!" as the response. */