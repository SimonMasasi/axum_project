use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ConnectionTrait, EntityTrait, IntoActiveModel, QueryOrder,
};
use crate::{
    error::{AppError, AppResult},
    modules::settings::{
        dtos::email_settings::{EmailSettingsInputDto},
        models::email_settings,
    },
};


pub  async  fn list(db: &impl  ConnectionTrait) -> AppResult<Vec<email_settings::Model>> {
    let email_settings = email_settings::Entity::find()
        .order_by_asc(email_settings::Column::Id)
        .all(db)
        .await?;

    Ok(email_settings)
}

pub  async  fn get(db: &impl  ConnectionTrait, id: i32) -> AppResult<email_settings::Model> {
    email_settings::Entity::find_by_id(id)
    .one(db)
        .await?
    .ok_or_else(|| AppError::NotFound(format!("{} not found", id)))
}

pub async  fn create(db : &impl ConnectionTrait , input: EmailSettingsInputDto) -> AppResult<email_settings::Model> {

    let  input = input.validate()?;

    let new_email_setting = email_settings::ActiveModel{
        smtp_port:Set(input.smtp_port),
        smtp_host:Set(input.smtp_host),
        password:Set(input.password),
        default_email:Set(input.default_email),
        user_name:Set(input.user_name),
        ..Default::default()

    };

    Ok(new_email_setting.insert(db).await?)
}


pub async  fn update(
    db: &impl ConnectionTrait,
    id: i32,
    input: EmailSettingsInputDto
) -> AppResult<email_settings::Model> {

    let  input = input.validate()?;
    let mut email_setting_to_update = get(db, id).await?.into_active_model();

    email_setting_to_update.user_name = Set(input.user_name);
    email_setting_to_update.smtp_host = Set(input.smtp_host);
    email_setting_to_update.smtp_port = Set(input.smtp_port);
    email_setting_to_update.default_email = Set(input.default_email);

    Ok(email_setting_to_update.update(db).await?)

}


pub async fn delete(db: &impl ConnectionTrait, id: i32) -> AppResult<()> {
    let delete_email =  email_settings::Entity::delete_by_id(id).exec(db).await?;

    if delete_email.rows_affected == 0{
        return Err(AppError::NotFound(format!("Email {} already exists", id)))
    }

    Ok(())
}