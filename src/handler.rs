use std::io::{Read};
use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, Json, debug_handler};
use axum::body::{StreamBody};
use axum::extract::{Multipart, Path, Query, State};
use axum::http::{header};
use chrono::Duration;
use lazy_static::lazy_static;
use serde::Deserialize;
use crate::{
    AppState,
    misc::get_random_hash,
    model::FileModel,
};
use tokio::fs::File;
use tokio::io::{AsyncWriteExt};
use tracing::info;
use tokio_util::io::ReaderStream;
use crate::misc::parse_duration;

lazy_static! {
    pub static ref STORE_DIR: String = std::env::var("STORE_DIR").expect("STORE_DIR must be set");
}

pub async fn health_check_handler() -> impl IntoResponse {
    let json_response = serde_json::json!({
        "status": "success",
        "message": "ok"
    });

    Json(json_response)
}

pub async fn download_file_handler(
    Path(hash): Path<String>,
    State(data): State<Arc<AppState>>,
) -> impl IntoResponse {

    let query_result = sqlx::query_as!(
            FileModel,
            "SELECT * FROM files WHERE hash = $1",
            hash
        ).fetch_one(&data.db).await;

    if let Ok(f) = query_result {
        let path = std::path::Path::new(&*STORE_DIR).join(f.hash.unwrap());
        let file = match tokio::fs::File::open(path).await {
            Ok(file) => file,
            Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err))),
        };
        let stream = ReaderStream::new(file);
        let body = StreamBody::new(stream);

        let headers = [
            (header::CONTENT_TYPE, format!("{}; charset=utf-8", f.content_type.unwrap())),
            (
                header::CONTENT_DISPOSITION,
                format!("attachment; filename=\"{}\"", f.name.unwrap()),
            ),
        ];

        Ok((headers, body))
    } else {
        Err((StatusCode::NOT_FOUND, format!("File not found")))
    }
}

pub async fn get_file_handler(
    Path(hash): Path<String>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    info!("Getting file with hash: {:?}", hash);

    let query_result = sqlx::query_as!(
            FileModel,
            "SELECT * FROM files WHERE hash = $1",
            hash
        ).fetch_one(&data.db).await;

    if let Ok(file) = query_result {
        let json_response = serde_json::json!(file);
        return Ok(Json(json_response))
    }

    return Err((StatusCode::NOT_FOUND, Json(serde_json::json!({
        "status": "error",
        "message": "File not found"
    }))))
}

#[derive(Deserialize)]
pub struct UploadQuery {
    pub expires_in: String,
}

impl Default for UploadQuery {
    fn default() -> Self {
        Self {
            expires_in: "7d".to_string(),
        }
    }
}

#[debug_handler]
pub async fn upload_file_handler(
    State(data): State<Arc<AppState>>,
    opts: Option<Query<UploadQuery>>,
    mut files: Multipart,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    while let Some(f) = files.next_field().await.unwrap() {
        let expires_in = parse_duration(opts.unwrap_or_default().expires_in.as_str()).unwrap();

        if expires_in > Duration::days(30) {
            return Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({
                "status": "error",
                "message": "File expiration time cannot be longer than 30 days"
            }))))
        }

        let file_hash = get_random_hash(15);
        let path = std::path::Path::new(&*STORE_DIR).join(file_hash.clone());

        let file_name = f.file_name().unwrap().to_string();
        let content_type = f.content_type().unwrap().to_string();
        let content = f.bytes().await.unwrap();
        let hash_md5 = md5::compute(&content);

        if !path.exists() {
            tokio::fs::create_dir_all(&*STORE_DIR).await.unwrap();
        }

        info!("Saving file to {:?}", path);
        info!("File name: {:?}", file_name);
        info!("File hash: {:?}", file_hash);
        info!("File md5: {:?}", format!("{:x}", hash_md5));
        info!("File content type: {:?}", content_type);
        info!("File size: {:?}", content.len());

        let mut file = File::create(path).await.unwrap();
        file.write_all(&content).await.unwrap();

        let query_result = sqlx::query_as!(
                FileModel,
                "INSERT INTO files (hash, name, md5, content_type, size, expires_at) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
                file_hash,
                file_name,
                format!("{:x}", hash_md5),
                content_type,
                content.len() as i32,
                chrono::Utc::now().naive_utc() + expires_in
            ).fetch_one(&data.db).await;

        return if let Ok(f) = query_result {
            let json_response = serde_json::json!(f);
            Ok(Json(json_response))
        } else {
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "status": "error",
                "message": "Could not save file"
            }))))
        }

    }

    return Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({
        "status": "error",
        "message": "No file provided"
    }))))
}
