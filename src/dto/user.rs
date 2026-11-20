use sea_orm::prelude::DateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{entities::user, error::AppError};

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub is_active: bool,
    pub created_at: DateTime,
}

impl From<user::Model> for UserResponse {
    fn from(m: user::Model) -> Self {
        Self {
            id: m.id,
            username: m.username,
            email: m.email,
            is_active: m.is_active,
            created_at: m.created_at,
        }
    }
}

/// Body of `POST /users`.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateUser {
    /// 3–32 characters; surrounding whitespace is trimmed.
    #[schema(example = "simon", min_length = 3, max_length = 32)]
    pub username: String,
    /// Stored lower-cased.
    #[schema(example = "simon@example.com")]
    pub email: String,
}

impl CreateUser {
    pub fn validate(self) -> Result<Self, AppError> {
        let username = validate_username(&self.username)?;
        let email = validate_email(&self.email)?;
        Ok(Self { username, email })
    }
}

/// Body of `PATCH /users/{id}`. Omitted fields are left unchanged.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUser {
    pub username: Option<String>,
    pub email: Option<String>,
    pub is_active: Option<bool>,
}

impl UpdateUser {
    pub fn validate(self) -> Result<Self, AppError> {
        Ok(Self {
            username: self.username.as_deref().map(validate_username).transpose()?,
            email: self.email.as_deref().map(validate_email).transpose()?,
            is_active: self.is_active,
        })
    }
}

fn validate_username(raw: &str) -> Result<String, AppError> {
    let username = raw.trim();
    if username.len() < 3 || username.len() > 32 {
        return Err(AppError::BadRequest(
            "username must be between 3 and 32 characters".into(),
        ));
    }
    Ok(username.to_string())
}

fn validate_email(raw: &str) -> Result<String, AppError> {
    let email = raw.trim().to_lowercase();
    let valid = email
        .split_once('@')
        .is_some_and(|(local, domain)| !local.is_empty() && domain.contains('.'));
    if !valid {
        return Err(AppError::BadRequest("email is not valid".into()));
    }
    Ok(email)
}
