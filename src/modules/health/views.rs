use axum::{Json, extract::State};
use serde::Serialize;
use utoipa::ToSchema;

use crate::{
    error::{AppResult, ErrorResponse},
    modules::health::services,
    state::AppState,
};

#[derive(Debug, Serialize, ToSchema)]
pub struct HealthResponse {
    #[schema(example = "ok")]
    pub status: String,
}

/// Check that the service and its database are reachable.
#[utoipa::path(
    get,
    path = "/health",
    tag = "health",
    responses(
        (status = 200, description = "Service is healthy", body = HealthResponse),
        (status = 500, description = "Database is unreachable", body = ErrorResponse),
    )
)]
pub async fn health(State(state): State<AppState>) -> AppResult<Json<HealthResponse>> {
    services::check(&state.db).await?;
    Ok(Json(HealthResponse {
        status: "ok".into(),
    }))
}
