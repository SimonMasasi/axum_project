use sea_orm::DatabaseConnection;

use crate::error::AppResult;

/// Fails if the database can't be reached.
pub async fn check(db: &DatabaseConnection) -> AppResult<()> {
    db.ping().await?;
    Ok(())
}
