use super::models::TokenData;
use crate::api::errors::{ApiError, ApiErrorData, JupiterApiErrorData};
use anyhow::Error;
use reqwest::{Client, Method, Response, StatusCode};
use serde_json::from_str;

pub struct JupiterApiClient {
    client: Client,
    base_url: String,
}

impl JupiterApiClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: String::from("https://lite-api.jup.ag/tokens/v2"),
        }
    }

    pub async fn fetch_token_info(&self, token_address: &str) -> Result<TokenData, Error> {
        let url = format!("{}/search?query={}", &self.base_url, token_address);

        let mut response = self.client.request(Method::GET, url).send().await?;
        response = self.handle_response(response).await?;

        let body = response.text().await?;

        let token_info: Vec<TokenData> = from_str(&body)?;

        if token_info.is_empty() {
            return Err(ApiError::NotFound {
                error_data: ApiErrorData {
                    source: String::from("Jupiter Api"),
                    status_code: StatusCode::NOT_FOUND.as_u16(),
                    message: String::from("Token not found"),
                },
            }
            .into());
        }

        Ok(token_info.first().unwrap().clone())
    }

    async fn handle_response(&self, response: Response) -> Result<Response, Error> {
        let status_code = response.status().as_u16();

        if status_code == 200 {
            return Ok(response);
        }

        let jupiter_error_data: JupiterApiErrorData = from_str(&response.text().await?)?;

        let error_data = ApiErrorData {
            source: String::from("Jupiter Api"),
            status_code,
            message: jupiter_error_data.error,
        };

        match status_code {
            400 => Err(ApiError::BadRequest { error_data }.into()),
            401 => Err(ApiError::Unauthorized { error_data }.into()),
            404 => Err(ApiError::NotFound { error_data }.into()),
            429 => Err(ApiError::TooManyRequests { error_data }.into()),
            500 => Err(ApiError::InternalServerError { error_data }.into()),
            502 => Err(ApiError::BadGateway { error_data }.into()),
            503 => Err(ApiError::ServiceUnavailable { error_data }.into()),
            504 => Err(ApiError::GatewayTimeout { error_data }.into()),
            _ => Err(ApiError::Unknown { error_data }.into()),
        }
    }
}
