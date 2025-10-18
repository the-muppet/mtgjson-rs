pub mod boosters;
pub mod card_sealed_products;
pub mod decks;
pub mod mtgsqlite;
pub mod sealed;

use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use serde_json::{Value, Map};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProviderError {
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),
    #[error("JSON parsing failed: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("GitHub API error: {status} - {message}")]
    GitHubApiError { status: u16, message: String },
    #[error("Configuration error: {0}")]
    ConfigError(String),
}

pub type Result<T> = std::result::Result<T, ProviderError>;

#[derive(Debug, Clone)]
pub struct MtgjsonConfig {
    pub github_token: Option<String>,
    pub user_agent: String,
}

impl Default for MtgjsonConfig {
    fn default() -> Self {
        Self {
            github_token: std::env::var("GITHUB_TOKEN").ok(),
            user_agent: std::env::var("USER_AGENT").unwrap_or_else(|_| "mtgserde1.0".to_string()),
        }
    }
}

#[async_trait::async_trait]
pub trait AbstractProvider {
    async fn download(&self, url: &str) -> Result<Value>;
    fn log_download(&self, response: &Response);
}

pub struct BaseProvider {
    client: Client,
    config: MtgjsonConfig,
}

impl BaseProvider {
    pub fn new(config: MtgjsonConfig) -> Result<Self> {
        let mut headers = reqwest::header::HeaderMap::new();

        if let Some(token) = &config.github_token {
            headers.insert(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {}", token).parse()
                    .map_err(|_| ProviderError::ConfigError("Invalid GitHub token".to_string()))?
            );
        }

        headers.insert(
            reqwest::header::USER_AGENT,
            config.user_agent.parse()
                .map_err(|_| ProviderError::ConfigError("Invalid user agent".to_string()))?
        );

        let client = Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Self { client, config })
    }
}

#[async_trait::async_trait]
impl AbstractProvider for BaseProvider {
    async fn download(&self, url: &str) -> Result<Value> {
        log::info!("Downloading from: {}", url);

        let response = self.client.get(url).send().await?;
        self.log_download(&response);

        if response.status().is_success() {
            let json = response.json::<Value>().await?;
            Ok(json)
        } else {
            Err(ProviderError::GitHubApiError {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            })
        }
    }

    fn log_download(&self, response: &Response) {
        log::info!(
            "Download response: {} {} - {}",
            response.status().as_u16(),
            response.status().canonical_reason().unwrap_or("Unknown"),
            response.url()
        );
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SealedProductCategory {
    Booster,
    Bundle,
    Deck,
    Duel,
    Gift,
    Memorabilia,
    Other,
    Pack,
    Planeswalker,
    Starter,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SealedProductSubtype {
    Blitz,
    Challenger,
    Commander,
    Event,
    Intro,
    Planeswalker,
    Theme,
    Welcome,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SealedProductIdentifiers {
    pub amazon_asin: Option<String>,
    pub card_market: Option<String>,
    pub tcg_player: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SealedProductObject {
    pub name: String,
    pub release_date: Option<String>,
    pub language: Option<String>,
    pub category: Option<SealedProductCategory>,
    pub subtype: Option<SealedProductSubtype>,
    pub identifiers: SealedProductIdentifiers,
    pub raw_purchase_urls: HashMap<String, String>,
    pub product_size: Option<u32>,
    pub card_count: Option<u32>,
    pub contents: Option<Map<String, Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeckCardObject {
    pub uuid: String,
    pub count: u32,
    pub is_foil: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeckObject {
    pub name: String,
    pub code: String,
    pub deck_type: String,
    pub release_date: Option<String>,
    pub sealed_uuids: Option<Vec<String>>,
    pub main_board: Vec<DeckCardObject>,
    pub side_board: Vec<DeckCardObject>,
    pub display_commander: Vec<DeckCardObject>,
    pub commander: Vec<DeckCardObject>,
    pub planes: Vec<DeckCardObject>,
    pub schemes: Vec<DeckCardObject>,
}

pub fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() && i > 0 {
            result.push('_');
        }
        result.push(c.to_lowercase().next().unwrap());
    }
    result
}

pub fn recursive_sort(value: Value) -> Value {
    match value {
        Value::Object(mut map) => {
            let mut sorted_map = serde_json::Map::new();
            let mut keys: Vec<_> = map.keys().cloned().collect();
            keys.sort();

            for key in keys {
                if let Some(val) = map.remove(&key) {
                    sorted_map.insert(key, recursive_sort(val));
                }
            }
            Value::Object(sorted_map)
        }
        Value::Array(arr) => {
            Value::Array(arr.into_iter().map(recursive_sort).collect())
        }
        other => other,
    }
}

pub use boosters::GitHubBoostersProvider;
pub use card_sealed_products::GitHubCardSealedProductsProvider;
pub use decks::GitHubDecksProvider;
pub use mtgsqlite::GitHubMTGSqliteProvider;
pub use sealed::GitHubSealedProvider;
