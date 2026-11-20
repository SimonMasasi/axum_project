use axum::{Json, Router, routing::get};
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;

use crate::state::AppState;

mod health;
mod users;

/// Path the generated OpenAPI document is served at.
pub const OPENAPI_PATH: &str = "/api-docs/openapi.json";

/// OpenAPI document for this service. Add new handlers to `paths(...)` after
/// annotating them with `#[utoipa::path]`; schemas they reference are picked
/// up automatically.
#[derive(OpenApi)]
#[openapi(
    info(title = "axum_project API", description = "User management API"),
    paths(
        health::health,
        users::list_users,
        users::get_user,
        users::create_user,
        users::update_user,
        users::delete_user,
    ),
    tags(
        (name = "health", description = "Service health"),
        (name = "users", description = "Create, read, update and delete users"),
    )
)]
pub struct ApiDoc;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health::health))
        .nest("/users", users::router())
        .route(OPENAPI_PATH, get(|| async { Json(ApiDoc::openapi()) }))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
