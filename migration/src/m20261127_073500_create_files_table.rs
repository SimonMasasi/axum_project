use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261127_073500_create_files_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Files::Table)
                    .if_not_exists()
                    .col(pk_auto(Files::Id))
                    // Name of the object in RustFS (unique, e.g. "<uuid>.png")
                    .col(string_uniq(Files::Key))
                    // Original name the user uploaded, e.g. "cat.png"
                    .col(string(Files::FileName))
                    .col(string(Files::ContentType))
                    // Size in bytes
                    .col(big_integer(Files::Size))
                    .col(timestamp(Files::CreatedAt).default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Files::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Files {
    Table,
    Id,
    Key,
    FileName,
    ContentType,
    Size,
    CreatedAt,
}
