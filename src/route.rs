use std::sync::Arc;
use std::time::Duration;
use axum::error_handling::HandleErrorLayer;
use axum::http::{header, Method, Response, StatusCode};
use axum::http::header::CONTENT_TYPE;
use axum::{middleware, Router};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use tower::buffer::BufferLayer;
use tower::limit::RateLimitLayer;
use tower::load_shed::LoadShedLayer;
use tower::ServiceBuilder;
use tower::timeout::TimeoutLayer;
use tower_http::cors::{Any, CorsLayer};
use tower_http::{LatencyUnit, ServiceBuilderExt};
use tower_http::services::ServeDir;
use tower_http::trace::{DefaultOnResponse, TraceLayer};
use tracing::Level;
use crate::{
    AppState,
    handler::{
        health_check_handler,
        upload_file_handler,
    },
};
use crate::file::{get_file_details_html};
use crate::handler::{download_file_handler};
use crate::views::root::root;
use crate::views::upload::upload;


pub fn create_router(app_state: Arc<AppState>) -> Router {

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE]);


    let serve_dir = ServeDir::new("assets");

    Router::new()
        // .layer(DefaultBodyLimit::disable())
        // .layer(RequestBodyLimitLayer::new(
        //     250 * 1024 * 1024, /* 250mb */
        // ))
        // .layer(tower_http::trace::TraceLayer::new_for_http())
        .route("/", get(root))
        .route("/file/:hash", get(get_file_details_html))
        .route("/upload", get(upload))
        .route("/styles.css", get(styles))
        .route("/health", get(health_check_handler))
        .route("/api/upload", post(upload_file_handler))
        // .route("/meta/:hash", get(get_file_handler))
        .route("/api/download/:hash", get(download_file_handler))
        .layer(
            ServiceBuilder::new()
                .layer(
                    TraceLayer::new_for_http().on_response(
                        DefaultOnResponse::new()
                            .level(Level::INFO)
                            .latency_unit(LatencyUnit::Millis),
                    ),
                )
                .layer(CorsLayer::very_permissive())
        )
        .nest_service("/assets", serve_dir.clone())
        .with_state(app_state)
}

async fn styles() -> impl IntoResponse {
    return Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/css")
        .body(load_file::load_str!("../style/output.css").to_owned())
        .unwrap();
}