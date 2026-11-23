//! User business logic. Nothing here knows about HTTP: functions take a
//! connection plus plain input, and return models or an `AppError`.
//!
//! `db` is any `ConnectionTrait`, so these work with the pool or inside a
//! transaction.

use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ConnectionTrait, EntityTrait, IntoActiveModel, QueryOrder,
};

use crate::{
    error::{AppError, AppResult},
    modules::auth::{
        dtos::users::{CreateUser, UpdateUser},
        models::users,
    },
};

/// All users, ordered by id.
pub async fn list(db: &impl ConnectionTrait) -> AppResult<Vec<users::Model>> {
    let users = users::Entity::find()
        .order_by_asc(users::Column::Id)
        .all(db)
        .await?; // DbErr -> AppError::Database automatically
    Ok(users)
}

/// Loads a user or returns `AppError::NotFound`.
pub async fn get(db: &impl ConnectionTrait, id: i32) -> AppResult<users::Model> {
    users::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| not_found(id))
}

/// Validates the input and inserts a user.
pub async fn create(db: &impl ConnectionTrait, input: CreateUser) -> AppResult<users::Model> {
    let input = input.validate()?;

    let new_user = users::ActiveModel {
        username: Set(input.username),
        email: Set(input.email),
        // id, is_active and created_at are filled in by the database defaults
        ..Default::default()
    };
    // A duplicate username/email becomes a 409 Conflict (see error.rs).
    Ok(new_user.insert(db).await?)
}

/// Validates the input and updates the fields that are present.
pub async fn update(
    db: &impl ConnectionTrait,
    id: i32,
    input: UpdateUser,
) -> AppResult<users::Model> {
    let input = input.validate()?;
    let mut user = get(db, id).await?.into_active_model();

    if let Some(username) = input.username {
        user.username = Set(username);
    }
    if let Some(email) = input.email {
        user.email = Set(email);
    }
    if let Some(is_active) = input.is_active {
        user.is_active = Set(is_active);
    }

    Ok(user.update(db).await?)
}

/// Deletes a user or returns `AppError::NotFound`.
pub async fn delete(db: &impl ConnectionTrait, id: i32) -> AppResult<()> {
    let result = users::Entity::delete_by_id(id).exec(db).await?;
    if result.rows_affected == 0 {
        return Err(not_found(id));
    }
    Ok(())
}

fn not_found(id: i32) -> AppError {
    AppError::NotFound(format!("user {id} not found"))
}
