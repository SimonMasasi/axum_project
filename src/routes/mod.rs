use axum::{Json, Router, routing::get};
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;

use crate::{
    modules::{auth::views::users,
              files::views as files,
              health::views as health,
              auth::views::roles,
              settings::views::email_settings
    },
    state::AppState,
};
use crate::modules::settings;

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
        roles::list_all_roles,
        roles::find_role_by_id,
        roles::create_role,
        roles::update_role,
        roles::delete_role,
        email_settings::list_all_email_settings,
        email_settings::find_email_settings,
        files::upload_file,
        files::list_files,
        files::get_file,
        files::download_file,
    ),
    tags(
        (name = "health", description = "Service health"),
        (name = "users", description = "Create, read, update and delete users"),
        (name = "roles", description = "Create, read, update and delete Roles"),
        (name = "files", description = "Upload and download files (stored in RustFS)"),

    )
)]
pub struct ApiDoc;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health::health))
        .nest("/users", users::router())
        .nest("/roles", roles::router())
        .nest("/email_settings", email_settings::email_settings_router())
        .nest("/files", files::router())
        .route(OPENAPI_PATH, get(|| async { Json(ApiDoc::openapi()) }))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
