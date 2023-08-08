use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::chrono::NaiveDateTime;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct FileModel {
    pub id: i32,
    pub hash: Option<String>,
    pub name: Option<String>,
    pub md5: Option<String>,
    pub content_type: Option<String>,
    pub size: Option<i32>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<NaiveDateTime>,
}
