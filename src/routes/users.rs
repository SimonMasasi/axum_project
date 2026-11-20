use axum::{
    Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait, IntoActiveModel, QueryOrder};

use crate::{
    dto::user::{CreateUser, UpdateUser, UserResponse},
    entities::user,
    error::{AppError, AppJson, AppResult, ErrorResponse},
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
    let users = user::Entity::find()
        .order_by_asc(user::Column::Id)
        .all(&state.db)
        .await?; // DbErr -> AppError::Database automatically

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
    let user = find_user(&state, id).await?;
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
    let input = input.validate()?;

    let new_user = user::ActiveModel {
        username: Set(input.username),
        email: Set(input.email),
        // id, is_active and created_at are filled in by the database defaults
        ..Default::default()
    };
    // A duplicate username/email becomes a 409 Conflict (see error.rs).
    let created = new_user.insert(&state.db).await?;

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
    let input = input.validate()?;
    let mut user = find_user(&state, id).await?.into_active_model();

    if let Some(username) = input.username {
        user.username = Set(username);
    }
    if let Some(email) = input.email {
        user.email = Set(email);
    }
    if let Some(is_active) = input.is_active {
        user.is_active = Set(is_active);
    }

    let updated = user.update(&state.db).await?;
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
    let result = user::Entity::delete_by_id(id).exec(&state.db).await?;
    if result.rows_affected == 0 {
        return Err(user_not_found(id));
    }
    Ok(StatusCode::NO_CONTENT)
}

/// Loads a user or returns a 404 `AppError`.
async fn find_user(state: &AppState, id: i32) -> AppResult<user::Model> {
    user::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| user_not_found(id))
}

fn user_not_found(id: i32) -> AppError {
    AppError::NotFound(format!("user {id} not found"))
}
