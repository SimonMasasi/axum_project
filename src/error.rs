use axum::{
    Json,
    extract::{FromRequest, multipart::MultipartError, rejection::JsonRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use s3::error::S3Error;
use sea_orm::{DbErr, SqlErr};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("{0}")]
    NotFound(String),

    #[error("{0}")]
    BadRequest(String),

    #[error("{0}")]
    Conflict(String),

    /// The request body wasn't valid JSON or didn't match the expected shape.
    #[error(transparent)]
    InvalidJson(#[from] JsonRejection),

    #[error(transparent)]
    Database(#[from] DbErr),

    /// The upload form was malformed or the file was too big.
    #[error(transparent)]
    Multipart(#[from] MultipartError),

    /// Something went wrong talking to RustFS (file storage).
    #[error(transparent)]
    Storage(#[from] S3Error),
}

pub type AppResult<T> = Result<T, AppError>;

/// JSON body returned for every error response.
#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse {
    pub error: ErrorDetail,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorDetail {
    /// Machine-readable error code, e.g. `not_found`, `bad_request`, `conflict`.
    #[schema(example = "not_found")]
    pub code: String,
    /// Human-readable description of the problem.
    #[schema(example = "user 42 not found")]
    pub message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message) = match &self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, "not_found", msg.clone()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, "bad_request", msg.clone()),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, "conflict", msg.clone()),
            AppError::InvalidJson(rejection) => {
                (rejection.status(), "invalid_json", rejection.body_text())
            }
            AppError::Database(err) => {

                if let Some(SqlErr::UniqueConstraintViolation(_)) = err.sql_err() {
                    (
                        StatusCode::CONFLICT,
                        "conflict",
                        "a record with these values already exists".to_string(),
                    )
                } else {

                    tracing::error!(error = ?err, "database error");
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "internal_error",
                        "something went wrong".to_string(),
                    )
                }
            }
            AppError::Multipart(err) => (err.status(), "bad_request", err.body_text()),
            // RustFS answers 404 when the file doesn't exist.
            AppError::Storage(S3Error::HttpFailWithBody(404, _)) => (
                StatusCode::NOT_FOUND,
                "not_found",
                "file not found".to_string(),
            ),
            AppError::Storage(err) => {
                tracing::error!(error = ?err, "storage error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal_error",
                    "something went wrong".to_string(),
                )
            }
        };

        let body = Json(ErrorResponse {
            error: ErrorDetail {
                code: code.to_string(),
                message,
            },
        });
        (status, body).into_response()
    }
}

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(AppError))]
pub struct AppJson<T>(pub T);

impl<T: Serialize> IntoResponse for AppJson<T> {
    fn into_response(self) -> Response {
        Json(self.0).into_response()
    }
}
