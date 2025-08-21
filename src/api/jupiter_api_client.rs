use super::{models::TokenInfo, traits::TokenApiClient};
use anyhow::Error;
use async_trait::async_trait;
use reqwest::{Client, Method};
use serde_json::from_str;

pub struct JupiterApiClient {
    client: Client,
    base_url: String,
}

#[async_trait]
impl TokenApiClient for JupiterApiClient {
    async fn fetch_token_info(&self, token_mint: String) -> Result<Option<TokenInfo>, Error> {
        let url = format!("{}/search?query={}", &self.base_url, token_mint);

        let response = self.client.request(Method::GET, url).send().await?;

        let body = response.text().await?;

        let token_info: Vec<TokenInfo> = from_str(&body)?;

        if token_info.is_empty() {
            return Ok(None);
        }

        Ok(Some(token_info.first().cloned().unwrap()))
    }
}

impl JupiterApiClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: String::from("https://lite-api.jup.ag/tokens/v2"),
        }
    }
}
