use  sea_orm::entity::prelude::*;


#[derive(Clone ,Debug , PartialEq , DeriveEntityModel )]
#[sea_orm(table_name="email_settings")]
pub struct  Model{

    #[sea_orm(primary_key)]
    pub id : i32,

    pub smtp_host: String,

    pub password: String,

    pub user_name: String,

    pub smtp_port: i32,

    pub default_email: String,

    pub is_active:bool,

    pub created_at: DateTime,

}

#[derive(Copy , Clone , Debug , EnumIter , DeriveRelation)]
pub enum Relation {}


impl  ActiveModelBehavior  for ActiveModel {}


