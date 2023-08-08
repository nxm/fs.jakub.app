#![feature(result_option_inspect)]

mod handler;
mod route;
mod misc;
mod model;

use std::net::SocketAddr;
use tracing_subscriber::prelude::*;

use futures::SinkExt;
use sqlx::{postgres::{PgPoolOptions}, ConnectOptions, Postgres, Pool};
use tracing::{info, log};
use anyhow::Context;
use std::{str::FromStr, thread};
use std::sync::Arc;
use axum::response::IntoResponse;
use crate::misc::cleaner;
use crate::route::create_router;

pub struct AppState {
    db: Pool<Postgres>
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "fileshare=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    info!("Connecting to database...");
    let pool = PgPoolOptions::new()
        .max_connections(50)
        .connect(&database_url)
        .await
        .context("Could not connect to postgres!")?;
    info!("Connected to database!");


    tokio::spawn(cleaner(Arc::new(AppState { db: pool.clone() })));

    info!("Starting server...");
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let app = create_router(Arc::new(AppState { db: pool.clone() }));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();


    Ok(())
}


