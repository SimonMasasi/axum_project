//! Connection to RustFS, our file storage.
//!
//! RustFS speaks the same API as Amazon S3, so we use the `rust-s3` crate to
//! talk to it. Files ("objects") live inside a "bucket", which is like a
//! top-level folder.

use anyhow::Context;
use s3::{Bucket, BucketConfiguration, Region, creds::Credentials};

use crate::config::Config;

pub async fn connect(config: &Config) -> anyhow::Result<Box<Bucket>> {
    // RustFS isn't on AWS, so we give it a "custom" region pointing at our server.
    let region = Region::Custom {
        region: "us-east-1".to_string(), // any value works for RustFS
        endpoint: config.s3_endpoint.clone(),
    };

    let credentials = Credentials::new(
        Some(&config.s3_access_key),
        Some(&config.s3_secret_key),
        None,
        None,
        None,
    )
    .context("invalid S3 credentials")?;

    // "Path style" means URLs look like http://host:9000/bucket/file
    // (instead of http://bucket.host:9000/file), which is what RustFS expects.
    let bucket = Bucket::new(&config.s3_bucket, region.clone(), credentials.clone())?
        .with_path_style();

    // Create the bucket the first time we run.
    let exists = bucket
        .exists()
        .await
        .with_context(|| format!("failed to reach RustFS at {}", config.s3_endpoint))?;

    if !exists {
        Bucket::create_with_path_style(
            &config.s3_bucket,
            region,
            credentials,
            BucketConfiguration::default(),
        )
        .await
        .context("failed to create bucket")?;
        tracing::info!(bucket = %config.s3_bucket, "created storage bucket");
    }

    Ok(bucket)
}
