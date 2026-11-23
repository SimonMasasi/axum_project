use sea_orm::prelude::DateTime;
use serde::{Deserialize , Serialize};
use utoipa::ToSchema;


use  crate::{error::AppError, modules::auth::{models::roles}};


#[derive(Debug , Serialize , ToSchema)]
#[serde(rename_all = "camelCase")]
pub  struct RolesResponse{
    pub id: i32,
    pub name:String,
    pub description: Option<String>,
    pub  is_active:bool,
    pub created_at:DateTime
}


impl  From<roles::Model> for RolesResponse {

    fn from(value: roles::Model) -> Self {
        Self {
            id:value.id,
            name:value.name,
            description:value.description,
            is_active:value.is_active,
            created_at:value.created_at
        }
    }
    
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct  CreateRoleInput {

    #[schema(example="super user" , min_length=4 , max_length = 40)]
    pub name:String,

    #[schema(example = "role for super user")]
    pub description:String

}


impl  CreateRoleInput {

    pub  fn validate(self) -> Result<Self , AppError> {
        let name = validate_role_name(&self.name)?;
        let description = self.description;

        Ok(Self { name , description})
    }
    
}


fn validate_role_name(raw: &str) ->Result<String , AppError> {
    let  role_name = raw.trim();
    if role_name.len() < 4 || role_name.len() > 40{
        return  Err(AppError::BadRequest(
         "role name must be between 4 and 40 characters".into(),
        ))
    }
    Ok(role_name.to_string())
}
