use thiserror::Error;
use crate::api::errors::ApiError;

#[derive(Debug, Error)]
pub enum AppError {
	#[error("Api Error {0}")]
	Api(#[from] ApiError),
	#[error("Token Pair Error {0}")]
	TokenPair(#[from] TokenPairError)
}

#[derive(Debug, Error)]
pub enum TokenPairError {
    #[error("Active pair not found {}", token_address)]
    ActivePairNotFound {
		token_address: String
	},
}