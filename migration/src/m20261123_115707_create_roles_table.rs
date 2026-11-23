use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261123_115707_create_roles_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        _manager
        .create_table(
            Table::create()
            .table(Roles::Table)
            .if_not_exists()
            .col(pk_auto(Roles::Id))
            .col(string_uniq(Roles::Name))
            .col(string(Roles::Description))
            .col(boolean(Roles::IsActive).default(true))
            .col(timestamp(Roles::CreatedAt).default(Expr::current_timestamp()))
            .to_owned(),
        ).await
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        _manager
        .drop_table(Table::drop().table(Roles::Table).to_owned())
        .await
    }
}


#[derive(DeriveIden)]
enum  Roles {
    Table,
    Id,
    Name,
    Description,
    IsActive,
    CreatedAt,
}