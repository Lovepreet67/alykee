# Task Management API

A simple task management backend built with **Rust**, **Axum**, **SQLx**, and **PostgreSQL**.

## Database

This project uses **PostgreSQL** as its database.

Before running the application, ensure that:

* PostgreSQL is installed and running.
* The database specified in the environment variables exists.
* The required migrations have been applied.

## Configuration

A sample environment file is provided:

```text
.env.sample
```

Copy it to `.env` and update the values as needed.

## Running the Application

Start the server with:

```bash
cargo run
```

The server will read the configuration from `.env` and connect to the configured PostgreSQL database.

## API Testing

A Postman collection is included in the repository for testing the available APIs.

Location:

```text
alkyee.postman_collection.json
```

Replace the placeholder above with the actual relative path to the Postman collection file in the repository.
