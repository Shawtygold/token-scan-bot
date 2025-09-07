use serde::Deserialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("{} | Bad Request: status: {}, {}", error_data.source, error_data.status_code, error_data.message)]
    BadRequest { error_data: ApiErrorData },
    #[error("{} | Unauthorized: status: {}, {}", error_data.source, error_data.status_code, error_data.message)]
    Unauthorized { error_data: ApiErrorData },
    #[error("{} | Forbidden: status: {}, {}", error_data.source, error_data.status_code, error_data.message)]
    Forbidden { error_data: ApiErrorData },
    #[error("{} | TooManyRequests: status: {}, {}", error_data.source, error_data.status_code, error_data.message)]
    TooManyRequests { error_data: ApiErrorData },
    #[error("{} | InternalServerError: status: {}, {}", error_data.source, error_data.status_code, error_data.message)]
    InternalServerError { error_data: ApiErrorData },
    #[error("{} | Service Unavailable: status: {}, {}", error_data.source, error_data.status_code, error_data.message)]
    ServiceUnavailable { error_data: ApiErrorData },
    #[error("{} | Access Denied: status: {}, {}", error_data.source, error_data.status_code, error_data.message)]
    AccessDenied { error_data: ApiErrorData },
    #[error("{} | Missing API Key: status: {}, {}", error_data.source, error_data.status_code, error_data.message)]
    MissingAPIKey { error_data: ApiErrorData },
    #[error("{} | Invalid API Key: status: {}, {}", error_data.source, error_data.status_code,error_data.message)]
    InvalidAPIKey { error_data: ApiErrorData },
    #[error("{} | Not Found: status: {}, {}", error_data.source, error_data.status_code, error_data.message)]
    NotFound { error_data: ApiErrorData },
    #[error("{} | Bad Gateway: status: {}, {}", error_data.source, error_data.status_code, error_data.message)]
    BadGateway { error_data: ApiErrorData },
    #[error("{} | Gateway Timeout: status: {}, {}", error_data.source, error_data.status_code, error_data.message)]
    GatewayTimeout { error_data: ApiErrorData },
    #[error("{} | Unknown: {}, {}", error_data.source, error_data.status_code, error_data.message)]
    Unknown { error_data: ApiErrorData },
}

#[derive(Debug, Clone)]
pub struct ApiErrorData {
    pub source: String,
    pub status_code: u16,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct MoralisApiErrorData {
    #[serde(rename = "statusCode")]
    pub status_code: u16,
    pub message: String
}

#[derive(Debug, Deserialize)]
pub struct JupiterApiErrorData {
    pub error: String,
}
