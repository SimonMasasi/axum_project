use axum::{
    Router,
    extract::{DefaultBodyLimit, Multipart, Path, State},
    http::{StatusCode, header},
    response::IntoResponse,
    routing::get,
};

use crate::{
    error::{AppError, AppJson, AppResult, ErrorResponse},
    modules::files::{
        dtos::{FileResponse, UploadFileForm},
        services as files_service,
    },
    state::AppState,
};

/// Biggest file we accept (axum's default is only 2 MB).
const MAX_UPLOAD_SIZE: usize = 10 * 1024 * 1024; // 10 MB

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_files).post(upload_file))
        .route("/{id}", get(get_file))
        .route("/{id}/download", get(download_file))
        .layer(DefaultBodyLimit::max(MAX_UPLOAD_SIZE))
}

/// Upload a file. Send it as `multipart/form-data` in a field called `file`.
#[utoipa::path(
    post,
    path = "/files",
    tag = "files",
    request_body(content = UploadFileForm, content_type = "multipart/form-data"),
    responses(
        (status = 201, description = "File uploaded", body = FileResponse),
        (status = 400, description = "No file was sent", body = ErrorResponse),
    )
)]
pub async fn upload_file(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> AppResult<(StatusCode, AppJson<FileResponse>)> {
    // A multipart form can have many fields; look for the one named "file".
    while let Some(field) = multipart.next_field().await? {
        if field.name() != Some("file") {
            continue;
        }

        let file_name = field.file_name().unwrap_or("file").to_string();
        let content_type = field
            .content_type()
            .unwrap_or("application/octet-stream")
            .to_string();
        let data = field.bytes().await?;

        if data.is_empty() {
            return Err(AppError::BadRequest("the uploaded file is empty".into()));
        }

        let file =
            files_service::upload(&state.db, &state.bucket, file_name, content_type, &data).await?;
        return Ok((StatusCode::CREATED, AppJson(file.into())));
    }

    Err(AppError::BadRequest(
        "no file found: send it in a form field named `file`".into(),
    ))
}

/// List every uploaded file (newest first).
#[utoipa::path(
    get,
    path = "/files",
    tag = "files",
    responses(
        (status = 200, description = "All uploaded files", body = Vec<FileResponse>),
    )
)]
pub async fn list_files(State(state): State<AppState>) -> AppResult<AppJson<Vec<FileResponse>>> {
    let files = files_service::list(&state.db).await?;
    Ok(AppJson(files.into_iter().map(Into::into).collect()))
}

/// Get one file's details (not its contents).
#[utoipa::path(
    get,
    path = "/files/{id}",
    tag = "files",
    params(("id" = i32, Path, description = "File id")),
    responses(
        (status = 200, description = "File details", body = FileResponse),
        (status = 404, description = "File not found", body = ErrorResponse),
    )
)]
pub async fn get_file(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> AppResult<AppJson<FileResponse>> {
    let file = files_service::get(&state.db, id).await?;
    Ok(AppJson(file.into()))
}

/// Download a file's contents.
#[utoipa::path(
    get,
    path = "/files/{id}/download",
    tag = "files",
    params(("id" = i32, Path, description = "File id")),
    responses(
        (status = 200, description = "The file's contents"),
        (status = 404, description = "File not found", body = ErrorResponse),
    )
)]
pub async fn download_file(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> AppResult<impl IntoResponse> {
    let (file, bytes) = files_service::download(&state.db, &state.bucket, id).await?;

    // Setting Content-Type lets the browser show images/PDFs directly.
    Ok(([(header::CONTENT_TYPE, file.content_type)], bytes))
}
