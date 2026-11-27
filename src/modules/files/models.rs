use sea_orm::entity::prelude::*;

/// One row per uploaded file. The file's bytes live in RustFS under `key`;
/// this table just remembers what was uploaded.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "files")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    #[sea_orm(unique)]
    pub key: String,

    pub file_name: String,

    pub content_type: String,

    pub size: i64,

    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
