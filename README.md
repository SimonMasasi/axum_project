# axum_project

A small REST API built with Axum 0.8 and SeaORM 2 (SQLite).

## Run

```sh
cp .env.example .env   # once
cargo run              # applies pending migrations, then starts on 127.0.0.1:3000
cargo test             # integration tests against an in-memory database
```

Adding a feature (e.g. posts): migration -> `entities/post.rs` -> `dto/post.rs`
-> `routes/posts.rs` -> `.nest("/posts", posts::router())` in `routes/mod.rs`.

## Migrations

```sh
sea-orm-cli migrate generate create_posts_table   # new file in migration/src
# then add it to the vec in migration/src/lib.rs

cargo run -p migration -- up       # apply pending
cargo run -p migration -- down     # roll back the last one
cargo run -p migration -- status
cargo run -p migration -- fresh    # drop everything and re-apply (dev only)
```

Rules:
- Once a migration has run anywhere shared, **never edit it**. Add a new one.
- Always write `down` so you can roll back.
- Keep `src/entities` in sync with the schema (or regenerate with
  `sea-orm-cli generate entity -o src/entities`).
- The server auto-applies migrations on startup (`RUN_MIGRATIONS=true`). In
  production you may prefer `RUN_MIGRATIONS=false` and run
  `cargo run -p migration -- up` as a separate deploy step.

## Errors

Handlers return `AppResult<T>` and use `?`. `AppError` maps to a status code and a
consistent body:

```json
{ "error": { "code": "not_found", "message": "user 99 not found" } }
```

| Variant                     | Status |
|-----------------------------|--------|
| `NotFound`                  | 404    |
| `BadRequest` (validation)   | 400    |
| `Conflict` / unique violation | 409  |
| `InvalidJson` (bad body)    | 400/415/422 |
| `Database` (anything else)  | 500, real error logged, not sent |

## API

| Method | Path          | Body                                  |
|--------|---------------|---------------------------------------|
| GET    | /health       |                                       |
| GET    | /users        |                                       |
| POST   | /users        | `{ "username", "email" }`             |
| GET    | /users/{id}   |                                       |
| PATCH  | /users/{id}   | any of `username`, `email`, `isActive` |
| DELETE | /users/{id}   |                                       |
