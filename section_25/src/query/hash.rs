// Description: This function takes a user-provided password as input, hashes it using bcrypt with a cost factor of 4, and returns the hashed password as a String.
// Parameters:
// - password: A String containing the user-provided password that needs to be hashed.
// Returns:
// - Result<String, StatusCode>: Returns a Result containing the hashed password as a String if successful, or a StatusCode indicating an internal server error if hashing fails.

use axum::http::StatusCode;

pub fn hash_password(password: String) -> Result<String, StatusCode> {
    // bcrypt::hash is a function provided by the 'bcrypt' crate to securely hash passwords.
    // It takes the password and the cost factor as input and returns a Result containing the hashed password or an error.
    bcrypt::hash(password, 4) // The cost factor of 4 determines the computational cost of the hashing process. Higher values increase security but take longer to compute.
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR) // If an error occurs during hashing, the 'map_err' function maps the error to an internal server error StatusCode.
}