//! Integration tests: build the real router against an in-memory SQLite
//! database and send requests to it directly (no network needed).
//!
//! Run with: cargo test

use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode, header},
};
use axum_project::{routes, state::AppState};
use http_body_util::BodyExt;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database};
use serde_json::{Value, json};
use tower::ServiceExt; // for `oneshot`

async fn test_app() -> Router {
    // Each in-memory SQLite connection is its own empty database, so keep the
    // pool at exactly one connection.
    let mut opts = ConnectOptions::new("sqlite::memory:");
    opts.max_connections(1).min_connections(1).sqlx_logging(false);
    let db = Database::connect(opts).await.unwrap();
    Migrator::up(&db, None).await.unwrap();
    routes::router(AppState { db })
}

async fn send(app: &Router, method: &str, uri: &str, body: Option<Value>) -> (StatusCode, Value) {
    let mut req = Request::builder().method(method).uri(uri);
    let body = match body {
        Some(json) => {
            req = req.header(header::CONTENT_TYPE, "application/json");
            Body::from(json.to_string())
        }
        None => Body::empty(),
    };
    let res = app.clone().oneshot(req.body(body).unwrap()).await.unwrap();
    let status = res.status();
    let bytes = res.into_body().collect().await.unwrap().to_bytes();
    let json = serde_json::from_slice(&bytes).unwrap_or(Value::Null);
    (status, json)
}

#[tokio::test]
async fn health_is_ok() {
    let app = test_app().await;
    let (status, body) = send(&app, "GET", "/health", None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["status"], "ok");
}

#[tokio::test]
async fn user_crud_flow() {
    let app = test_app().await;

    let (status, created) = send(
        &app,
        "POST",
        "/users",
        Some(json!({ "username": "simon", "email": "Simon@Example.com" })),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(created["email"], "simon@example.com"); // normalised
    assert_eq!(created["isActive"], true); // DB default
    let id = created["id"].as_i64().unwrap();

    let (status, list) = send(&app, "GET", "/users", None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(list.as_array().unwrap().len(), 1);

    let (status, updated) = send(
        &app,
        "PATCH",
        &format!("/users/{id}"),
        Some(json!({ "isActive": false })),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(updated["isActive"], false);

    let (status, _) = send(&app, "DELETE", &format!("/users/{id}"), None).await;
    assert_eq!(status, StatusCode::NO_CONTENT);

    let (status, body) = send(&app, "GET", &format!("/users/{id}"), None).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body["error"]["code"], "not_found");
}

#[tokio::test]
async fn duplicate_email_is_409() {
    let app = test_app().await;
    let user = json!({ "username": "simon", "email": "simon@example.com" });
    send(&app, "POST", "/users", Some(user)).await;

    let (status, body) = send(
        &app,
        "POST",
        "/users",
        Some(json!({ "username": "other", "email": "simon@example.com" })),
    )
    .await;
    assert_eq!(status, StatusCode::CONFLICT);
    assert_eq!(body["error"]["code"], "conflict");
}

#[tokio::test]
async fn validation_and_bad_json_return_json_errors() {
    let app = test_app().await;

    let (status, body) = send(
        &app,
        "POST",
        "/users",
        Some(json!({ "username": "ab", "email": "simon@example.com" })),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["error"]["code"], "bad_request");

    // Missing `email` field: axum's rejection is converted into our JSON shape.
    let (status, body) = send(&app, "POST", "/users", Some(json!({ "username": "simon" }))).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(body["error"]["code"], "invalid_json");
}

#[tokio::test]
async fn openapi_spec_is_served() {
    let app = test_app().await;
    let (status, spec) = send(&app, "GET", routes::OPENAPI_PATH, None).await;
    assert_eq!(status, StatusCode::OK);
    assert!(spec["openapi"].as_str().unwrap().starts_with("3."));

    // Documented paths line up with the real routes.
    for path in ["/health", "/users", "/users/{id}"] {
        assert!(spec["paths"][path].is_object(), "missing path {path}");
    }
    for method in ["get", "patch", "delete"] {
        assert!(spec["paths"]["/users/{id}"][method].is_object(), "missing {method} /users/{{id}}");
    }

    let schemas = &spec["components"]["schemas"];
    for name in ["UserResponse", "CreateUser", "UpdateUser", "ErrorResponse"] {
        assert!(schemas[name].is_object(), "missing schema {name}");
    }
    // serde's camelCase renaming is reflected in the schema.
    assert!(schemas["UserResponse"]["properties"]["isActive"].is_object());
}
