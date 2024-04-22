mod handler;
mod route;
mod misc;
mod components;
mod views;
mod error;
mod file;

use std::net::SocketAddr;
use tracing_subscriber::prelude::*;

use futures::SinkExt;
use sqlx::{postgres::{PgPoolOptions}, ConnectOptions, Postgres, Pool};
use tracing::{info, log};
use anyhow::Context;
use std::{env, str::FromStr, thread};
use std::sync::Arc;
use std::time::Duration;
use axum::response::IntoResponse;
use sqlx::postgres::PgConnectOptions;
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

    let pg_url = env::var("DATABASE_URL").expect("Invalid DATABASE_URL env var");
    let pool = PgPoolOptions::new()
        .min_connections(2)
        .max_connections(100)
        .acquire_timeout(Duration::from_secs(120))
        .connect_with(
            PgConnectOptions::from_str(&pg_url)
                .context("Could not create pg otions from given url")?,
        )
        .await
        .context("Could not connect to postgres!")?;
    info!("Connected to database!");


    tokio::spawn(cleaner(Arc::new(AppState { db: pool.clone() })));

    info!("Starting server...");
    let addr = SocketAddr::from(([0, 0, 0, 0], 2115));
    let app = create_router(Arc::new(AppState { db: pool.clone() }));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();


    Ok(())
}


