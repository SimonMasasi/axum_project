use anyhow::Context;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use crate::config::Config;

pub async fn connect(config: &Config) -> anyhow::Result<DatabaseConnection> {
    let mut opts = ConnectOptions::new(&config.database_url);
    opts.sqlx_logging(false);

    let db = Database::connect(opts)
        .await
        .with_context(|| format!("failed to connect to {}", config.database_url))?;

    if config.run_migrations {
        Migrator::up(&db, None)
            .await
            .context("failed to run database migrations")?;
        tracing::info!("database migrations are up to date");
    }

    Ok(db)
}
