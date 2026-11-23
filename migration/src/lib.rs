pub use sea_orm_migration::prelude::*;

mod m20261119_080453_create_users_table;
mod m20261123_115707_create_roles_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {

    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20261119_080453_create_users_table::Migration),
            Box::new(m20261123_115707_create_roles_table::Migration),
        ]
    }
}
