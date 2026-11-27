use sea_orm::entity::prelude::DateTime;
use serde::Serialize;
use utoipa::ToSchema;

use crate::modules::files::models as files;

/// Details about an uploaded file.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct FileResponse {
    pub id: i32,
    /// The name the file is stored under in RustFS.
    #[schema(example = "6f1c2c1e-6a51-4d3b-9b9e-1f0a2d6a7b11.png")]
    pub key: String,
    /// The original file name sent by the client.
    #[schema(example = "cat.png")]
    pub file_name: String,
    #[schema(example = "image/png")]
    pub content_type: String,
    /// Size in bytes.
    pub size: i64,
    /// Where to GET the file's contents from.
    #[schema(example = "/files/1/download")]
    pub url: String,
    pub created_at: DateTime,
}

impl From<files::Model> for FileResponse {
    fn from(value: files::Model) -> Self {
        Self {
            url: format!("/files/{}/download", value.id),
            id: value.id,
            key: value.key,
            file_name: value.file_name,
            content_type: value.content_type,
            size: value.size,
            created_at: value.created_at,
        }
    }
}

/// Only used to describe the upload form in the API docs.
#[derive(ToSchema)]
#[allow(dead_code)]
pub struct UploadFileForm {
    /// The file to upload.
    #[schema(value_type = String, format = Binary)]
    pub file: Vec<u8>,
}
