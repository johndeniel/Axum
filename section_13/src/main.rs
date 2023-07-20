/* Defines a struct User to represent user data retrieved from the database.
Defines an async function create_tables to create two tables (users and tasks) if they don't already exist.
Defines an async function get_all_users to fetch all users from the users table and return them as a Vec<User>.

In the main function:
Creates a connection pool to the PostgreSQL database.
Calls create_tables to ensure the required tables are available.
Calls get_all_users to fetch all users from the database and prints them. */


// The line below allows silencing unused warnings for this exploratory code.
// The `#![allow(unused)]` attribute is used for this purpose.
#![allow(unused)] 

// Import required dependencies from the `sqlx` library.
use sqlx::postgres::{PgPoolOptions, PgPool};
use sqlx::{Row, Error};

// Define a struct `User` to represent user data.
#[derive(Debug)]
struct User {
    id: i32,
    username: String,
    password: String,
    deleted_at: Option<String>,
    token: Option<String>,
}


// Define an async function `create_tables` to create necessary tables if they don't exist.
async fn create_tables(pool: &PgPool) -> Result<(), Error> {
	// Create the `users` table if it doesn't exist.
    sqlx::query(
        r#"
            CREATE TABLE IF NOT EXISTS users (
                id          SERIAL PRIMARY KEY,
                username    VARCHAR(64) NOT NULL UNIQUE,
                password    VARCHAR(64) NOT NULL,
                deleted_at  TIMESTAMPTZ DEFAULT NULL,
                token       TEXT DEFAULT NULL
            );
        "#,
    )
    .execute(pool)
    .await?;

	// Create the `tasks` table if it doesn't exist.
    sqlx::query(
        r#"
            CREATE TABLE IF NOT EXISTS tasks (
                id            SERIAL PRIMARY KEY,
                priority      VARCHAR(4) DEFAULT NULL,
                title         VARCHAR(255) NOT NULL,
                completed_at  TIMESTAMPTZ DEFAULT NULL,
                description   TEXT DEFAULT NULL,
                deleted_at    TIMESTAMPTZ DEFAULT NULL,
                user_id       INTEGER DEFAULT NULL, 
                is_default    BOOLEAN DEFAULT FALSE,
                CONSTRAINT fk_users FOREIGN KEY (user_id) REFERENCES users(id)
            );
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())  // Return `Ok` to indicate success without a value.
}


// Define an async function `get_all_users` to fetch all users from the `users` table.
async fn get_all_users(pool: &PgPool) -> Result<Vec<User>, Error> {
	// Fetch all rows from the `users` table.s
    let rows = sqlx::query("SELECT * FROM users").fetch_all(pool).await?;

	// Convert rows into a Vec<User>.
    let users: Vec<User> = rows
        .into_iter()
        .map(|row| User {
            id: row.get("id"),
            username: row.get("username"),
            password: row.get("password"),
            deleted_at: row.get("deleted_at"),
            token: row.get("token"),
        })
        .collect();

    Ok(users) // Return the Vec<User> as a successful result.
}


// The main function, where the program execution starts.
#[tokio::main]
async fn main() -> Result<(), Error> {
    // 1) Create a connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost/postgres")
        .await?;

    // 2) Create tables if not exist yet
    create_tables(&pool).await?;

    // 3) Fetch and print all users.
    let users = get_all_users(&pool).await?;
    for user in users {
        println!("{:?}", user);
    }

    Ok(()) // Return `Ok` to indicate successful program execution.
}