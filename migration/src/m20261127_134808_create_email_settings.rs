use std::arch::aarch64::int32x2_t;
use sea_orm_migration::{prelude::*, schema::*};
use crate::Value::String;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261127_134808_create_email_settings"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        _manager
            .create_table(
                Table::create()
                    .table(EmailSettings::Table)
                    .if_not_exists()
                    .col(pk_auto(EmailSettings::Id))
                    .col(string_uniq(EmailSettings::SmtpHost))
                    .col(string(EmailSettings::Password))
                    .col(integer(EmailSettings::SmtpPort))
                    .col(string(EmailSettings::Username))
                    .col(string(EmailSettings::DefaultEmailFrom))
                    .col(boolean(EmailSettings::IsActive).default(true))
                    .col(timestamp(EmailSettings::CreatedAt).default(Expr::current_timestamp()))
                    .to_owned(),

            ).await
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        _manager
            .drop_table(Table::drop().table(EmailSettings::Table).to_owned())
            .await
    }
}


#[derive(DeriveIden)]
enum EmailSettings {
    Table,
    Id,
    SmtpHost,
    Password,
    SmtpPort,
    Username,
    DefaultEmailFrom,
    IsActive,
    CreatedAt
}
