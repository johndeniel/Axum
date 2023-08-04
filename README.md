# Rust Axum 

This repository is dedicated to showcasing the power and versatility of the Axum framework in Rust programming language. Axum is a high-performance web framework designed for building scalable and asynchronous applications. This comprehensive introduction to the Axum framework is thoughtfully crafted by [Brooks Builds](https://github.com/brooks-builds/full-stack-todo-rust-course). The content within this lesson is made available under the [MIT License](https://opensource.org/licenses/mit/), reflecting our commitment to promoting open-source collaboration and knowledge sharing.

## Repository Contents

- **section_01**: axum-api
- **section_02**: post-body
- **section_03**: extract-json
- **section_04**: path-variable
- **section_05**: query-params
- **section_06**: std-header
- **section_07**: custom-header
- **section_08**: cors-middleware
- **section_09**: shared-middleware
- **section_10**: custom-middleware
- **section_11**: http-statuscode
- **section_12**: returning-json
- **section_13**: conn-sqlx
- **section_14**: sea-orm
- **section_15**: insert-default
- **section_16**: insert-w/json
- **section_17**: query-w/id
- **section_18**: query-all
- **section_19**: filtered-query
- **section_20**: atomic-update
- **section_21**: partial-update
- **section_22**: hard-delete 
- **section_23**: soft-delete
- **section_24**: creating-account
- **section_25**: hash-password

## Getting Started

```toml
# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
axum = { version = "0.6.18", features = ["headers"] }
tokio-postgres = "0.7"
serde = { version = "1.0.164", features = ["derive"] }
sqlx = { version = "0.7.1", features = ["runtime-tokio-rustls", "postgres"] }
tokio = { version = "1.28.2", features = ["macros", "rt-multi-thread"] }
tower-http = { version = "0.4.1", features = ["cors"] }
```
<br>

# JSON Web Token (JWT)

<div align="justify"> JSON Web Tokens (JWT) is an open standard (RFC 7519) that defines a compact and self-contained way of securely transmitting information between parties as a JSON object. It is commonly used for authentication and authorization in web applications and APIs. JWTs are digitally signed, ensuring their integrity, and can be verified and trusted by parties that share a secret or a public/private key pair. </div>

   - ## Structure
     A JWT consists of three parts separated by periods: `header.payload.signature`

   - ### Header
     The header typically consists of two parts: the token type (JWT) and the signing algorithm used, such as HMAC SHA256 or RSA. It is JSON data encoded in Base64Url.
     
     <b> Example Header: </b>
     
     ```json
     {
         "alg": "HS256",
         "typ": "JWT"
     }

     ```
     
   - ### Payload
     The payload contains the claims, which are statements about an entity (user, device, etc.) and additional data. There are three types of claims: registered, public, and private claims. Registered claims include standard fields like `iss` (issuer), `exp` (expiration time), `sub` (subject), and more. Public claims are defined by the application, and private claims are custom to entities. The payload is also Base64Url encoded.
     
     <b> Example Payload: </b>
     
     ```json
     {
         "iss": "example.com",
         "sub": "user123",
         "exp": 1671713600
     }

     ```
          
     
     
   - ### Signature
     The signature is created by combining the encoded header and payload along with a secret or private key using the specified algorithm. This signature ensures that the token hasn't been tampered with during transmission and can be verified by the recipient. It helps to maintain the token's integrity and authenticity.
     
     <b> Example Signature (HMAC SHA256): </b>
     
     ```bash
     HMACSHA256(
         base64UrlEncode(header) + "." + base64UrlEncode(payload),
         secret
     )

     ```     
     
<br>     

# JWT Flow

### Step 1: Authentication:
<div align="justify"> When a user attempts to log in, the server verifies the credentials. Upon successful authentication, the server generates a JWT and returns it to the client. </div>

### Step 2: Authorization:
<div align="justify"> The client includes the JWT in the request header for each subsequent API call. The server, before processing the request, verifies the token's signature and decodes the payload to retrieve relevant information about the user and their permissions. </div>

### Step 3: Expiration:
<div align="justify"> JWTs can have an expiration time (exp) in the payload. The server checks the token's validity by verifying if the current time is before the expiration time. If the token has expired, the server denies access. </div>

### Step 4: Revocation:
<div align="justify"> JWTs are stateless, meaning they are not stored on the server. If a user logs out or needs their access revoked for any reason, the server must maintain a separate blacklist or use short-lived tokens to handle revocation. </div>

# Advantages

- **Stateless:** Servers do not need to store token information, reducing database queries and scalability issues.

- **Self-contained:** All the necessary information is contained within the token, reducing the need for additional database lookups.

- **Easy to Use:** JWTs are widely supported, and libraries are available in multiple programming languages, making implementation straightforward.

- **Secure:** Digital signatures ensure that the token data remains tamper-proof and trustworthy.

# Considerations

- **Token Size:** Since JWTs are self-contained, including too much information can lead to larger token sizes, affecting network performance.

- **Sensitive Data:** Avoid storing sensitive information in the token as it can be decoded by anyone with access to the token.

- **Expiration:** Carefully consider token expiration times to balance security and user experience.

<br>

# Docker Compose: PostgreSQL Database

<div align="justify"> This repository contains a Docker Compose file that sets up a PostgreSQL database service using the latest PostgreSQL image. It's designed to provide an easy and standardized way to spin up a PostgreSQL database for your application. </div>

## Prerequisites

Before using this Docker Compose setup, you need to have the following installed on your system:

- Docker: [Install Docker](https://docs.docker.com/get-docker/)

## Description

The `docker-compose.yaml` file in this repository defines a single service named `database`, which runs a PostgreSQL database using the latest PostgreSQL image available on Docker Hub.

### Service Configuration

The PostgreSQL database service is configured with the following settings:

- **Container Name**: The PostgreSQL container is named `my-postgres-db`, making it easy to reference and manage.

- **Image**: The service uses the latest PostgreSQL image available on Docker Hub.

- **Volumes**: The service uses two volumes for data persistence:
  - `db-data`: A named volume that persists the PostgreSQL database data in the container's `/var/lib/postgresql/data` directory.
  - `./init.sql`: A bind mount that allows an `init.sql` script located in the `root` directory of the host to be executed during container initialization. You can modify this script to set up the initial database schema and data.

- **Environment Variables**: The `POSTGRES_PASSWORD` environment variable is set to `password`, providing the password for the PostgreSQL user `postgres`. You can change this to a more secure password as needed.

- **Ports**: The service maps port `5432` on the host to port `5432` in the container, allowing access to the PostgreSQL database from the host machine.

- **Healthcheck**: The service performs a health check using `pg_isready` to verify the availability of the PostgreSQL database. It retries the check 5 times at 10-second intervals during the start-up process.


## Usage

1. Start the PostgreSQL database service, run the following command:
```bash
docker-compose up -d
```

2. Access the terminal of the active container using the following command:
```bash
docker-compose exec database /bin/bash
```

3. Access the PostgreSQL database using the following command:
```bash
psql -U postgres -d postgres
```

4. You can now easily create and execute SQL commands.
```sql
SELECT * FROM users;
```

5. To stop the services and remove the containers, use the following command:
```bash
docker-compose down
```

<br>

# SQL Schema and Sample Data

<div align="justify"> This overview covers the SQL schema and sample data for two tables: 'users' and 'tasks'. The 'users' table stores user information with a unique ID, username, password, and optional fields for soft deletion and a token. The 'tasks' table contains task details like ID, priority, title, completion timestamp, description, soft deletion timestamp, user ID, and a boolean flag for default tasks. Sample data includes a user with the username 'deleteduser' and three tasks with distinct properties. </div>

## Table: users

The 'users' table stores information about users and has the following columns:

- **id**: An auto-incrementing unique ID for each user (Primary Key).
- **username**: The username of the user, which must be unique and not null.
- **password**: The password for the user, which is not null.
- **deleted_at**: A timestamp for soft deletion (when a user is deleted), defaults to null.
- **token**: A token field (not used in this script), defaults to null.

## Table: tasks

The 'tasks' table stores information about tasks and has the following columns:

- **id**: An auto-incrementing unique ID for each task (Primary Key).
- **priority**: The priority of the task, which can be null.
- **title**: The title of the task, which must not be null.
- **completed_at**: A timestamp for task completion (when a task is completed), defaults to null.
- **description**: A description of the task, which can be null.
- **deleted_at**: A timestamp for soft deletion (when a task is deleted), defaults to null.
- **user_id**: A foreign key referencing the user ID associated with the task.
- **is_default**: A boolean field indicating if the task is a default task, defaults to false.

The 'user_id' column in the 'tasks' table is a foreign key that references the 'id' column in the 'users' table, establishing a relationship between the two tables.

## Sample Data

### users Table

A sample user is inserted into the 'users' table with the following details:

- **username**: 'deleteduser'
- **password**: (hashed password)

### tasks Table

Three sample tasks are inserted into the 'tasks' table with the following details:

1. Task: 'my deleted task'
   - **deleted_at**: Current timestamp for soft deletion
   - **user_id**: ID of the 'deleteduser' from the 'users' table

2. Task: 'I am a task, you can complete me by checking the box'
   - **priority**: 'A'
   - **description**: 'This is my description'
   - **is_default**: true

3. Task: 'See my details for by clicking me'
   - **priority**: 'B'
   - **description**: 'My description can be changed'
   - **is_default**: true

These sample data entries provide an illustration of how data can be added to the tables and how they can be associated with each other using the foreign key constraint.


<br>




# Connecting to PostgreSQL using `sqlx` in Rust

In Rust, `sqlx` is a powerful library that provides a safe and efficient way to interact with databases using SQL queries. This markdown file will guide you through the process of connecting to a SQL database using `sqlx` in a Rust project.

## Prerequisites

Before you start, ensure that you have the following prerequisites installed:

1. Rust programming language - Visit [Rust's official website](https://www.rust-lang.org/learn/get-started) to install Rust.
2. Cargo - The package manager for Rust that comes with the Rust installation.

## Adding `sqlx` to your project

To use `sqlx`, you need to add it as a dependency to your Rust project. Follow these steps:

1. Open your project's `Cargo.toml` file.
2. Under the `[dependencies]` section, add the following lines:

```toml
sqlx = { version = "0.7.1", features = ["runtime-tokio-rustls", "postgres"] }
tokio = { version = "1.28.2", features = ["macros", "rt-multi-thread"] }
```


## Setting up your SQL Database
For this example, let's assume you have a PostgreSQL database running locally. Make sure you have the database URL available, which typically looks like this:

```bash
postgres://username:password@localhost:5432/database_name
```
## Writing Rust Code
Create a new Rust file, such as main.rs, in your project's source folder. We will define a simple program to connect to the database using sqlx and execute a query.

```rust
// main.rs

use sqlx::postgres::PgPoolOptions; // Change this based on your database (e.g., mysql::MySqlPoolOptions for MySQL)
use sqlx::prelude::*;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Replace the DATABASE_URL with your actual database URL
    let pool = PgPoolOptions::new()
        .max_connections(5) // Set your desired max connections count
        .connect("postgres://username:password@localhost:5432/database_name")
        .await?;

    // Example query: select all rows from a table
    let query_result = sqlx::query!("SELECT * FROM your_table_name")
        .fetch_all(&pool)
        .await?;

    // Process the query result
    for row in query_result {
        let column_name: DataType = row.get("column_name");
        // Do something with the data...
    }

    // Perform other database operations as needed...

    Ok(())
}
```

<br>

# SeaORM Overview

SeaORM is a Rust library that provides a simple and convenient way to interact with relational databases. It aims to make database access easy and intuitive while following Rust's safe and expressive design principles.

## Features

### 1. Type-Safe Queries

SeaORM uses the type system to ensure that your queries are valid at compile-time. This approach eliminates the possibility of runtime errors related to database queries, making your code more reliable and maintainable.

### 2. Code Generation

SeaORM provides a CLI tool that generates Rust code based on your database schema. This code generation simplifies the process of creating entities and queries, reducing boilerplate code and saving development time.

<br>

This will download and install the latest version of the `SeaORM CLI` from the crates.io repository.

```bash
cargo install sea-orm-cli
```

<br>


The `sea-orm-cli -h` command provides general help and displays information about the available commands and options.

```bash
sea-orm-cli -h
```

<br>

The `sea-orm-cli generate -h` command displays help specifically for the generate subcommand. The generate command is used to generate code for various components, such as entities and migrations.

```bash
sea-orm-cli generate -h
```

<br>

The `sea-orm-cli generate entity -h` command provides help for generating entity code. Entities represent tables in the database and are central to using SeaORM.

```bash
sea-orm-cli generate entity -h
```

<br>

The `sea-orm-cli generate entity -o src/database` command generates entity code and places it in the specified output directory (src/database in this case). This will create Rust files for each entity in the database, which can be later used with the SeaORM library.

```bash
sea-orm-cli generate entity -o src/database
```

<br>

With this command, you'll have entity code generated for your database tables, allowing you to start using SeaORM with your application.

### 3. Entity-Oriented API

The core concept in SeaORM is the Entity, which represents a table in the database. Entities are defined as Rust structs, and SeaORM automatically generates the necessary database schema based on these definitions. This approach keeps the codebase organized and easy to manage.

### 4. Query Builder

With SeaORM's query builder, you can construct complex queries in a builder pattern style. This allows you to build queries in a fluent and readable manner, making it easier to understand and modify them.

### 5. Associations

SeaORM supports defining relationships between entities, such as one-to-one, one-to-many, and many-to-many associations. This simplifies the process of fetching related data and ensures data integrity in your database.

### 6. Transactions

SeaORM allows you to work with database transactions, ensuring that multiple operations are atomic and consistent. This is crucial for maintaining data integrity and recovering from failures.

### 7. Database-Agnostic

SeaORM is designed to be database-agnostic, meaning it can work with various relational databases, including PostgreSQL, MySQL, SQLite, and more. This flexibility allows you to switch between databases easily without rewriting your codebase.

### 8. Asynchronous Support

SeaORM supports async/await, allowing you to perform asynchronous database operations, making your applications more scalable and efficient.

<br>

# Dotenvy (.env)

The `.env` file is a configuration file commonly used in software projects to store environment-specific settings and sensitive information. It is a simple text file that contains key-value pairs in the format of `KEY=VALUE`. The `.env` file is typically located in the root directory of the project and is read by the application to load environment variables during runtime.

## Purpose

The primary purpose of the `.env` file is to decouple configuration settings from the codebase. It allows developers to store sensitive data (like API keys, passwords, and database credentials) outside the version control system, reducing the risk of accidentally exposing this information.

## Usage

1. **Create the `.env` file**: Begin by creating a new file named `.env` in the root directory of your project.

2. **Add Environment Variables**: Inside the `.env` file, add the environment variables in the format of `KEY=VALUE`, one per line. For example:

```.env
DB_HOST=localhost
DB_USER=myusername
DB_PASSWORD=mypassword
SECRET_KEY=mysecretkey
```


3. **Loading Environment Variables**: To access the environment variables in your application, you need to load them during runtime. Many programming languages and frameworks have libraries or built-in functions for loading `.env` files and setting the environment variables.

   - In Node.js, you can use packages like `dotenv` to load the `.env` file.
   - In Python, you can use `python-dotenv` to load environment variables from the `.env` file.
   - In Ruby on Rails, the `.env` file is automatically loaded by default.

4. **Accessing Environment Variables**: Once the `.env` file is loaded, you can access the environment variables in your code using the appropriate methods provided by your programming language or framework. For example:

   - In Node.js:
     ```javascript
     const DB_HOST = process.env.DB_HOST;
     const SECRET_KEY = process.env.SECRET_KEY;
     ```

   - In Python:
     ```python
     import os

     db_host = os.environ.get('DB_HOST')
     secret_key = os.environ.get('SECRET_KEY')
     ```

5. **Git Ignore**: To ensure that sensitive information in the `.env` file is not committed to version control, it is essential to add `.env` to your `.gitignore` file. This prevents accidental exposure of sensitive data to your code repository.

## Security Considerations

- Never commit the `.env` file to version control. Always keep it local to your development environment.
- Limit access to the `.env` file and ensure it is not accessible to unauthorized users.
- Do not include any comments or whitespace lines in the `.env` file, as it may lead to unexpected behavior when loading environment variables.

Remember, the `.env` file is intended for development and local environments. In production or deployment environments, use the appropriate mechanism provided by your hosting service or platform to manage environment variables securely.

Using `.env` files is a widely accepted practice in the software development community to manage configuration and secrets safely. However, always handle sensitive data with caution and follow security best practices to protect your application and users.