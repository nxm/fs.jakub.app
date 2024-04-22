use std::sync::Arc;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::{Html, IntoResponse};
use leptos::ssr::render_to_string;
use leptos::view;
use leptos::IntoAttribute;
use leptos::IntoView;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::chrono::NaiveDateTime;
use tracing::info;
use crate::AppState;
use crate::components::meta::Metadata;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct File {
    pub id: i32,
    pub hash: Option<String>,
    pub name: Option<String>,
    pub md5: Option<String>,
    pub content_type: Option<String>,
    pub size: Option<i32>,
    #[serde(rename = "expiresAt")]
    pub expires_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<NaiveDateTime>,
}

pub async fn get_file_details_html(
    Path(hash): Path<String>,
    State(data): State<Arc<AppState>>,
) -> Result<Html<String>, (StatusCode, Json<serde_json::Value>)> {
    info!("Getting file with hash: {:?}", hash);

    let file = sqlx::query_as!(
            File,
            "SELECT * FROM files WHERE hash = $1",
            hash
        ).fetch_one(&data.db).await;

    match file {
        Ok(file) => { let html = render_to_string(move |cx| view! {
        cx,
        <Metadata />
        <main class="min-h-screen bg-neutral-900 text-black flex flex-col gap-4 items-center">
            <div class="w-full p-4 shadow flex justify-center">
                <div class="max-w-4xl w-full flex flex-col gap-4 mb-0">
                    <label class="text-xl font-medium text-white" for="input_url">
                        "Hash:"
                    </label>
                    <input class="px-6 py-4 rounded-lg shadow"
                        type="text"
                        name="text"
                        value={file.hash.clone().unwrap_or_default()}
                        size="30"
                        disabled
                     />

                    <label class="text-xl font-medium text-white" for="input_url">
                        "File name:"
                    </label>
                    <input class="px-6 py-4 rounded-lg shadow"
                        type="text"
                        name="text"
                        value={file.name.clone().unwrap_or_default()}
                        size="30"
                        disabled
                     />

                    <label class="text-xl font-medium text-white" for="input_url">
                        "MD5:"
                    </label>
                    <input class="px-6 py-4 rounded-lg shadow"
                        type="text"
                        name="text"
                        value={file.md5.clone().unwrap_or_default()}
                        size="30"
                        disabled
                     />


                    <label class="text-xl font-medium text-white" for="input_url">
                        "Size:"
                    </label>
                    <input class="px-6 py-4 rounded-lg shadow"
                        type="text"
                        name="text"
                        value={file.size.clone().unwrap_or_default()}
                        size="30"
                        disabled
                     />

                    <a href={ format!("/api/download/{}", file.hash.clone().unwrap_or_default())}>
                    <div class="flex px-6 py-4 rounded-lg shadow bg-red-800 hover:bg-red-600 transition duration-150 text-center ease-in-out text-white font-medium">
                        "Download"
                    </div>
                    </a>
                </div>
            </div>
        </main>
            });
            Ok(Html(html))
        }
        Err(_) => Err((StatusCode::NOT_FOUND, Json(serde_json::json!({
            "status": "error",
            "message": "File not found"
        }))))
    }
}

