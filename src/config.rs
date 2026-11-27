use anyhow::Context;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub addr: String,
    pub run_migrations: bool,
    // RustFS (S3-compatible) storage settings
    pub s3_endpoint: String,
    pub s3_access_key: String,
    pub s3_secret_key: String,
    pub s3_bucket: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "sqlite://db.sqlite?mode=rwc".to_string());

        let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port: u16 = std::env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .context("PORT must be a number between 0 and 65535")?;

        let run_migrations = std::env::var("RUN_MIGRATIONS")
            .map(|v| v != "false" && v != "0")
            .unwrap_or(true);

        // Small helper: read an env var or fall back to a default value.
        let env_or = |key: &str, default: &str| std::env::var(key).unwrap_or_else(|_| default.to_string());

        Ok(Self {
            database_url,
            addr: format!("{host}:{port}"),
            run_migrations,
            s3_endpoint: env_or("S3_ENDPOINT", "http://127.0.0.1:9000"),
            s3_access_key: env_or("S3_ACCESS_KEY", "admin"),
            s3_secret_key: env_or("S3_SECRET_KEY", "rustfadmin"),
            s3_bucket: env_or("S3_BUCKET", "uploads"),
        })
    }
}
