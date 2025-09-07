use super::constants::{MAX_VALID_TOKEN_PAIRS, PUMP_SWAP_ADDRESS, RAYDIUM_CPMM_ADDRESS};
use crate::api::errors::{ApiError, ApiErrorData, MoralisApiErrorData};
use crate::api::moralis::models::{
    TokenHolderStats, TokenMetadata, TokenPair, TokenPairStats, TokenPairs,
};
use crate::errors::TokenPairError;
use anyhow::Error;
use reqwest::{
    Client, Method, Response,
    header::{HeaderMap, HeaderValue},
};
use serde_json::from_str;
use validator::Validate;

pub struct MoralisApiClient {
    client: Client,
    base_url: String,
}

impl MoralisApiClient {
    pub fn new(api_key: &str) -> Result<Self, Error> {
        let mut headers: HeaderMap = HeaderMap::new();
        headers.insert("accept", HeaderValue::from_static("application/json"));
        headers.insert("X-API-KEY", HeaderValue::from_str(api_key)?);

        Ok(Self {
            client: Client::builder().default_headers(headers).build()?,
            base_url: String::from("https://solana-gateway.moralis.io"),
        })
    }

    pub async fn get_token_metadata(&self, token_address: &str) -> Result<TokenMetadata, Error> {
        let mut response = self
            .client
            .request(
                Method::GET,
                format!("{}/token/mainnet/{}/metadata", self.base_url, token_address),
            )
            .send()
            .await?;

        response = self.handle_response(response).await?;

        let body = response.text().await?;

        let token_metadata: TokenMetadata = from_str(&body)?;

        token_metadata.validate()?;

        Ok(token_metadata)
    }

    pub async fn get_primary_token_pair_by_address(
        &self,
        token_address: &str,
    ) -> Result<TokenPair, Error> {
        let mut response = self
            .client
            .request(
                Method::GET,
                format!("{}/token/mainnet/{}/pairs", self.base_url, token_address),
            )
            .send()
            .await?;

        response = self.handle_response(response).await?;

        let token_pairs: TokenPairs = from_str(&response.text().await?)?;

        let active_token_pairs: Vec<TokenPair> = token_pairs
            .pairs
            .into_iter()
            .filter(|p| !p.inactive_pair)
            .take(MAX_VALID_TOKEN_PAIRS)
            .collect::<Vec<TokenPair>>();

        if active_token_pairs.is_empty() {
            return Err(TokenPairError::ActivePairNotFound {token_address: String::from(token_address)}.into());
        }

        let primary_token_pair_opt: Option<TokenPair> =
            active_token_pairs.clone().into_iter().find(|pair| {
                pair.exchange_address == PUMP_SWAP_ADDRESS
                    || pair.exchange_address == RAYDIUM_CPMM_ADDRESS
        });

        let primary_token_pair = match primary_token_pair_opt {
            Some(token_pair) => token_pair,
            None => active_token_pairs[0].clone(),
        };

        primary_token_pair.validate()?;

        Ok(primary_token_pair)
    }

    pub async fn get_token_holders(&self, token_address: &str) -> Result<TokenHolderStats, Error> {
        let mut response = self
            .client
            .request(
                Method::GET,
                format!("{}/token/mainnet/holders/{}", self.base_url, token_address),
            )
            .send()
            .await?;

        response = self.handle_response(response).await?;

        let holder_stats: TokenHolderStats = from_str(&response.text().await?)?;

        Ok(holder_stats)
    }

    pub async fn get_token_pair_stats(&self, pair_address: &str) -> Result<TokenPairStats, Error> {
        let mut response = self
            .client
            .request(
                Method::GET,
                format!(
                    "{}/token/mainnet/pairs/{}/stats",
                    self.base_url, pair_address
                ),
            )
            .send()
            .await?;

        response = self.handle_response(response).await?;

        let token_pair_stats: TokenPairStats = from_str(&response.text().await?)?;
        
        token_pair_stats.validate()?;

        Ok(token_pair_stats)
    }

    async fn handle_response(&self, response: Response) -> Result<Response, Error> {
        if response.status().is_success() {
            return Ok(response);
        }

        let moralis_error_data: MoralisApiErrorData = from_str(&response.text().await?)?;

        let error_data: ApiErrorData = ApiErrorData {
            status_code: moralis_error_data.status_code,
            source: String::from("Moralis Api"),
            message: moralis_error_data.message,
        };

        match error_data.status_code {
            400 => Err(ApiError::BadRequest { error_data }.into()),
            401 => Err(ApiError::Unauthorized { error_data }.into()),
            403 => Err(ApiError::Forbidden { error_data }.into()),
            404 => Err(ApiError::NotFound { error_data }.into()),
            429 => Err(ApiError::TooManyRequests { error_data }.into()),
            500 => Err(ApiError::InternalServerError { error_data }.into()),
            _ => Err(ApiError::Unknown { error_data }.into()),
        }
    }
}
