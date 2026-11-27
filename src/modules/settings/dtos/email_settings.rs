use sea_orm::entity::prelude::DateTime;
use serde::{Deserialize , Serialize};
use utoipa::ToSchema;


use crate::{error::AppError , modules::settings::{models::email_settings}};


#[derive(Debug, Serialize , ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct EmailSettingsResponse {
    pub id: i32,
    pub smtp_host: String,
    pub user_name: String,
    pub smtp_port: i32,
    pub default_email: String,
    pub is_active:bool,
    pub created_at:DateTime
}


impl From<email_settings::Model> for EmailSettingsResponse {

    fn from(value: email_settings::Model) -> Self {
        Self {
            id:value.id,
            smtp_host : value.smtp_host,
            user_name : value.user_name,
            smtp_port : value.smtp_port,
            default_email : value.default_email,
            is_active: value.is_active,
            created_at:value.created_at,
        }
    }

}


#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct EmailSettingsInputDto {

    #[schema(example = "smtp_host" , min_length=4 , max_length=128 )]
    pub smtp_host: String,

    pub user_name: String,

    pub smtp_port: i32,

    pub default_email: String,

    pub password: String,
}


impl EmailSettingsInputDto {

    pub fn validate(self) -> Result<Self, AppError> {
        let smtp_host  = validate_email_host(&self.smtp_host)?;
        let user_name = self.user_name;
        let smtp_port = self.smtp_port;
        let default_email = self.default_email;
        let password = validate_password(&self.password)?;

        Ok(Self{
            smtp_host,
            user_name,
            smtp_port,
            default_email,
            password
        })

    }
}


fn validate_email_host(raw: &String) -> Result<String, AppError> {

    let smtp_host = raw.to_string();
    if smtp_host.is_empty() {
        return  Err(AppError::BadRequest("smtp_host is empty".to_string()))
    }
    if smtp_host.len() < 3 {
        return  Err(AppError::BadRequest("smtp_host is too short".to_string()))
    }
    if smtp_host.len() > 40 {
        return  Err(AppError::BadRequest("smtp_host is too long".to_string()))
    }

    Ok(smtp_host.to_string())
}


fn validate_password(raw: &String) -> Result<String, AppError> {
    let password = raw.trim();

    if password.is_empty() {
        return  Err(AppError::BadRequest("password is empty".to_string()))
    }

    if password.len() < 8 {
        return  Err(AppError::BadRequest("password is too short".to_string()))
    }

    Ok(password.to_string())
}