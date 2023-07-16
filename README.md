# Rust Axum 

<div align="justify"> This repository is dedicated to showcasing the power and versatility of the Axum framework in Rust programming language. Axum is a high-performance web framework designed for building scalable and asynchronous applications. </div>

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

## Getting Started

```toml
# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
axum = { version = "0.6.18", features = ["headers"] }
serde = { version = "1.0.164", features = ["derive"] }
tokio = { version = "1.28.2", features = ["macros", "rt-multi-thread"] }
tower-http = { version = "0.4.1", features = ["cors"] }
```

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