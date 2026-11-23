use axum::{
    Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};

use crate::{
    error::{AppJson, AppResult, ErrorResponse},
    modules::auth::{
        dtos::users::{CreateUser, UpdateUser, UserResponse},
        services::users as user_service,
    },
    state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_users).post(create_user))
        .route(
            "/{id}",
            get(get_user).patch(update_user).delete(delete_user),
        )
}


/// List all users, ordered by id.
#[utoipa::path(
    get,
    path = "/users",
    tag = "users",
    responses(
        (status = 200, description = "All users", body = Vec<UserResponse>),
    )
)]
async fn list_users(State(state): State<AppState>) -> AppResult<AppJson<Vec<UserResponse>>> {
    let users = user_service::list(&state.db).await?;
    Ok(AppJson(users.into_iter().map(Into::into).collect()))
}

/// Get a single user by id.
#[utoipa::path(
    get,
    path = "/users/{id}",
    tag = "users",
    params(
        ("id" = i32, Path, description = "User id"),
    ),
    responses(
        (status = 200, description = "The user", body = UserResponse),
        (status = 404, description = "User not found", body = ErrorResponse),
    )
)]
async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> AppResult<AppJson<UserResponse>> {
    let user = user_service::get(&state.db, id).await?;
    Ok(AppJson(user.into()))
}

/// Create a user.
#[utoipa::path(
    post,
    path = "/users",
    tag = "users",
    request_body = CreateUser,
    responses(
        (status = 201, description = "User created", body = UserResponse),
        (status = 400, description = "Validation failed", body = ErrorResponse),
        (status = 409, description = "Username or email already taken", body = ErrorResponse),
        (status = 422, description = "Body is not valid JSON for this schema", body = ErrorResponse),
    )
)]
async fn create_user(
    State(state): State<AppState>,
    AppJson(input): AppJson<CreateUser>,
) -> AppResult<(StatusCode, AppJson<UserResponse>)> {
    let created = user_service::create(&state.db, input).await?;
    Ok((StatusCode::CREATED, AppJson(created.into())))
}

/// Update some or all fields of a user.
#[utoipa::path(
    patch,
    path = "/users/{id}",
    tag = "users",
    params(
        ("id" = i32, Path, description = "User id"),
    ),
    request_body = UpdateUser,
    responses(
        (status = 200, description = "Updated user", body = UserResponse),
        (status = 400, description = "Validation failed", body = ErrorResponse),
        (status = 404, description = "User not found", body = ErrorResponse),
        (status = 409, description = "Username or email already taken", body = ErrorResponse),
        (status = 422, description = "Body is not valid JSON for this schema", body = ErrorResponse),
    )
)]
async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    AppJson(input): AppJson<UpdateUser>,
) -> AppResult<AppJson<UserResponse>> {
    let updated = user_service::update(&state.db, id, input).await?;
    Ok(AppJson(updated.into()))
}

/// Delete a user.
#[utoipa::path(
    delete,
    path = "/users/{id}",
    tag = "users",
    params(
        ("id" = i32, Path, description = "User id"),
    ),
    responses(
        (status = 204, description = "User deleted"),
        (status = 404, description = "User not found", body = ErrorResponse),
    )
)]
async fn delete_user(State(state): State<AppState>, Path(id): Path<i32>) -> AppResult<StatusCode> {
    user_service::delete(&state.db, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
