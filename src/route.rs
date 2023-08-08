use std::sync::Arc;
use std::time::Duration;
use axum::error_handling::HandleErrorLayer;
use axum::http::{header, Method, StatusCode};
use axum::http::header::CONTENT_TYPE;
use axum::{middleware, Router};
use axum::routing::{get, post};
use tower::buffer::BufferLayer;
use tower::limit::RateLimitLayer;
use tower::load_shed::LoadShedLayer;
use tower::ServiceBuilder;
use tower::timeout::TimeoutLayer;
use tower_http::cors::{Any, CorsLayer};
use tower_http::{LatencyUnit, ServiceBuilderExt};
use tower_http::trace::{DefaultOnResponse, TraceLayer};
use tracing::Level;
use crate::{
    AppState,
    handler::{
        health_check_handler,
        upload_file_handler,
    },
};
use crate::handler::{download_file_handler, get_file_handler};


pub fn create_router(app_state: Arc<AppState>) -> Router {

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE]);

    Router::new()
        // .layer(DefaultBodyLimit::disable())
        // .layer(RequestBodyLimitLayer::new(
        //     250 * 1024 * 1024, /* 250mb */
        // ))
        // .layer(tower_http::trace::TraceLayer::new_for_http())
        .route("/health", get(health_check_handler))
        .route("/upload", post(upload_file_handler))
        .route("/meta/:hash", get(get_file_handler))
        .route("/download/:hash", get(download_file_handler))
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
        .with_state(app_state)
}
