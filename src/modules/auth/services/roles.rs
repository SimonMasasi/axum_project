use sea_orm::{
    ActiveModelTrait , ActiveValue::Set , ConnectionTrait , EntityTrait , IntoActiveModel , QueryOrder
};

use crate::{
    error::{AppError , AppResult},
    modules::auth::{
        dtos::roles::{CreateRoleInput},
        models::roles,
    },
};


pub  async  fn  get_all_roles(db:&impl ConnectionTrait) -> AppResult<Vec<roles::Model>>{

    let  roles = roles::Entity::find()
    .order_by_asc(roles::Column::Id)
    .all(db)
    .await?;

    Ok(roles)
}


pub async  fn get_role_by_id(db:&impl ConnectionTrait , id:i32) -> AppResult<roles::Model>{
    roles::Entity::find_by_id(id)
    .one(db)
    .await?
    .ok_or_else(|| not_found(id))
}

pub async  fn create_role (db: &impl  ConnectionTrait , input:CreateRoleInput) ->AppResult<roles::Model>{

    let input =  input.validate()?;

    let new_role = roles::ActiveModel{
        name:Set(input.name),
        description:Set(Some(input.description)),
        ..Default::default()
    };

    Ok(new_role.insert(db).await?)
}


pub async fn  update_role(db: &impl  ConnectionTrait , id:i32 , input:CreateRoleInput) -> AppResult<roles::Model>{

    let  input = input.validate()?;

    let mut role = get_role_by_id(db, id).await?.into_active_model();

    role.name = Set(input.name);
    role.description = Set(Some(input.description));

    Ok(role.update(db).await?)

}


fn not_found (id:i32) -> AppError{
    AppError::NotFound(format!("role {id} not found"))
}