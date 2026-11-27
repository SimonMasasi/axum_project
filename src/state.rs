use s3::Bucket;
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    /// Handle to the RustFS bucket where uploaded files are stored.
    pub bucket: Box<Bucket>,
}
