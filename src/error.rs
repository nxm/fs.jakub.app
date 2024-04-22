use std::{fmt::Display, sync::Arc};

pub type AppResult<T> = core::result::Result<T, AppError>;

#[derive(Debug, Clone)]
pub enum AppError {
    Internal(Arc<anyhow::Error>),
}