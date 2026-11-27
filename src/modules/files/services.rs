use s3::Bucket;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ConnectionTrait, EntityTrait, QueryOrder};

use crate::{
    error::{AppError, AppResult},
    modules::files::models as files,
};

/// Save a file to RustFS, then record it in the database.
pub async fn upload(
    db: &impl ConnectionTrait,
    bucket: &Bucket,
    file_name: String,
    content_type: String,
    data: &[u8],
) -> AppResult<files::Model> {
    // Use a random UUID as the stored name so two uploads called "cat.png"
    // don't overwrite each other. Keep the extension so it's easy to recognise.
    let key = match file_name.rsplit_once('.') {
        Some((_, ext)) => format!("{}.{}", uuid::Uuid::new_v4(), ext),
        None => uuid::Uuid::new_v4().to_string(),
    };

    // 1. Put the bytes in RustFS.
    bucket
        .put_object_with_content_type(&key, data, &content_type)
        .await?;

    // 2. Remember the file in the database.
    let new_file = files::ActiveModel {
        key: Set(key.clone()),
        file_name: Set(file_name),
        content_type: Set(content_type),
        size: Set(data.len() as i64),
        ..Default::default() // id and created_at are filled in by the database
    };

    match new_file.insert(db).await {
        Ok(file) => Ok(file),
        Err(err) => {
            // The database save failed, so remove the file from RustFS again
            // to avoid leaving a file nobody knows about.
            let _ = bucket.delete_object(&key).await;
            Err(err.into())
        }
    }
}

/// All uploaded files, newest first.
pub async fn list(db: &impl ConnectionTrait) -> AppResult<Vec<files::Model>> {
    let files = files::Entity::find()
        .order_by_desc(files::Column::Id)
        .all(db)
        .await?;

    Ok(files)
}

/// One file's database record.
pub async fn get(db: &impl ConnectionTrait, id: i32) -> AppResult<files::Model> {
    files::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("file {id} not found")))
}

/// Look the file up in the database, then fetch its bytes from RustFS.
pub async fn download(
    db: &impl ConnectionTrait,
    bucket: &Bucket,
    id: i32,
) -> AppResult<(files::Model, Vec<u8>)> {
    let file = get(db, id).await?;
    let response = bucket.get_object(&file.key).await?;

    Ok((file, response.to_vec()))
}
