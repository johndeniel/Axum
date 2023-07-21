use axum::Extension;
use sqlx::PgPool;

// Define a struct to represent the data you want to insert
#[derive(sqlx::FromRow, Debug)]
struct Task {
    priority: String,
    title: String,
    description: String,
}

pub async fn create_task(Extension(database): Extension<PgPool>) {
    // Sample data to insert (you can replace this with actual data)
    let new_task = Task {
        priority: "A".to_string(),
        title: "sqlx".to_string(),
        description: "sqlx".to_string(),
    };

    // Write the SQL query to perform the insert operation
    let query = r#"
        INSERT INTO tasks (priority, title, description)
        VALUES ($1, $2, $3)
        RETURNING priority, title, description
    "#;

    // Execute the query and handle the result with proper error handling
    match insert_task(&database, query, &new_task).await {
        Ok(task) => {
            // Successfully inserted, you can process the returned task here if needed
            println!("Inserted task: {:?}", task);
        }
        Err(e) => {
            // Handle the error appropriately
            eprintln!("Error inserting task: {:?}", e);
        }
    }
}

// Function to insert the task and return the result wrapped in a Result
async fn insert_task(database: &PgPool, query: &str, new_task: &Task) -> Result<Task, sqlx::Error> {
    let task = sqlx::query_as::<_, Task>(query)
        .bind(&new_task.priority)      // Bind the priority field
        .bind(&new_task.title)         // Bind the title field
        .bind(&new_task.description)   // Bind the description field
        .fetch_one(database)           // Execute the query and fetch the result as Task
        .await?;                       // Properly handle the query result or any errors

    Ok(task)                          // Return the inserted task
}