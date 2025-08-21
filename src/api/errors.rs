use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiErrors {
    #[error("Missing token data: {0}")]
    MissingData(String),
}
