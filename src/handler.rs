use std::io::{Read};
use std::sync::Arc;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use axum::body::{StreamBody};
use axum::extract::{Multipart, Path, State};
use axum::http::{header};
use crate::{
    AppState,
    misc::get_random_hash,
    model::FileModel,
};
use tokio::fs::File;
use tokio::io::{AsyncWriteExt};
use tracing::info;
use tokio_util::io::ReaderStream;

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
        let path = std::path::Path::new("store").join(f.hash.unwrap());
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

pub async fn upload_file_handler(
    State(data): State<Arc<AppState>>,
    mut files: Multipart,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    while let Some(f) = files.next_field().await.unwrap() {
        let file_hash = get_random_hash();
        let path = std::path::Path::new("store").join(file_hash.clone());

        let file_name = f.file_name().unwrap().to_string();
        let content_type = f.content_type().unwrap().to_string();
        let content = f.bytes().await.unwrap();
        let hash_md5 = md5::compute(&content);

        if !path.exists() {
            tokio::fs::create_dir_all("store").await.unwrap();
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
                "INSERT INTO files (hash, name, md5, content_type, size) VALUES ($1, $2, $3, $4, $5) RETURNING *",
                file_hash,
                file_name,
                format!("{:x}", hash_md5),
                content_type,
                content.len() as i32
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
