use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Default, Deserialize, Serialize, Validate)]
pub struct TokenData {
    #[validate(length(min = 32, max = 44))]
    pub id: String,
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(length(min = 1))]
    pub symbol: String,
    pub dev: Option<String>,
    pub launchpad: Option<String>,
    #[serde(rename = "holderCount")]
    pub holder_count: Option<i32>,
    #[serde(rename = "firstPool")]
    pub first_pool: Option<FirstPool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FirstPool {
    pub id: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
}
