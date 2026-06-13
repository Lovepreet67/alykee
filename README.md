# Task Management API

A task management backend built with **Rust**, **Axum**, **SQLx**, **PostgreSQL**, **Redis**, and **Mailpit**.

The API supports admin-created users and tasks, JWT authentication, and email-based OTP verification.

## Prerequisites

- Rust
- Docker and Docker Compose
- SQLx CLI for running migrations

Install SQLx CLI if needed:

```bash
cargo install sqlx-cli --no-default-features --features postgres
```

## Configuration

A sample environment file is provided:

```text
.env.sample
```

Copy it to `.env` and update the values as needed.

```bash
cp .env.sample .env
```

Required environment variables:

```text
DATABASE_URL=postgres://root:rootpass@localhost:5332/assignement
JWT_SECRET=your_jwt_secret
ROOT_USER_PASSWORD=root
ROOT_USER_EMAIL=root@gmail.com
REDIS_HOST_NAME=localhost:6379
REDIS_USER_NAME=appuser
REDIS_PASSWORD=apppassword
```

On startup, the application creates a root admin user using `ROOT_USER_EMAIL` and `ROOT_USER_PASSWORD` if it does not already exist.

## Services

The application needs:

- **PostgreSQL** for users and tasks
- **Redis** for OTP storage
- **Mailpit** for receiving local OTP emails

A `docker-compose.yml` file is included for all local services.

```bash
docker compose up -d
```

PostgreSQL runs on:

```text
localhost:5332
```

The Compose file creates the database configured in `.env.sample`:

```text
POSTGRES_USER=root
POSTGRES_PASSWORD=rootpass
POSTGRES_DB=assignement
```

Mailpit web UI is available at:

```text
http://localhost:8025
```

SMTP runs on:

```text
localhost:1025
```

Note: the current Docker Compose file mounts `./redis.conf`. Make sure `redis.conf` exists before starting Redis with Docker Compose.

## Database

After starting Docker Compose, run migrations:

```bash
sqlx migrate run
```

The migration creates:

- `users`
- `task`
- `user_role`
- `task_status`

## Running the Application

Start the local services, run migrations, then start the server:

```bash
docker compose up -d
sqlx migrate run
cargo run
```

If the services are already running, start only the server with:

```bash
cargo run
```

The server starts on:

```text
http://127.0.0.1:3000
```

## Health Check

Check database and Redis connectivity:

```http
GET /health_check
```

Example response:

```json
{
  "redis": "Ok",
  "db": "Ok"
}
```

If a dependency is unavailable, its status returns an error message.

## Authentication Flow

Login is OTP-based:

1. Call `POST /login` with email and password.
2. The API sends an OTP to the registered email through Mailpit.
3. Call `POST /verify-otp` with email and OTP.
4. The API returns a JWT.

Protected routes expect the JWT in the `Auth` header.

## API Routes

Public routes:

```http
POST /login
POST /verify-otp
GET /health_check
```

Authenticated user routes:

```http
GET /task
```

Admin routes:

```http
POST /user
POST /task
```

## Testing

Run tests with:

```bash
cargo test
```

This project uses SQLx compile-time query checking, so tests and builds need access to the configured PostgreSQL database unless SQLx offline metadata is added.

Run Clippy with:

```bash
cargo clippy --all-targets -- -D warnings
```

## API Testing

A Postman collection is included:

```text
alkyee.postman_collection.json
```
