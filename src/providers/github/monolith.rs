use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use serde_json::{Value, Map};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::OnceCell;
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

type Result<T> = std::result::Result<T, ProviderError>;

// Configuration management
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

// Base provider trait
#[async_trait::async_trait]
pub trait AbstractProvider {
    async fn download(&self, url: &str) -> Result<Value>;
    fn log_download(&self, response: &Response);
}

// Base provider implementation
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

// Data structures
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

// GitHub Sealed Provider
pub struct GitHubSealedProvider {
    provider: Arc<BaseProvider>,
    sealed_products: OnceCell<HashMap<String, HashMap<String, Value>>>,
    sealed_contents: OnceCell<HashMap<String, HashMap<String, Value>>>,
}

impl GitHubSealedProvider {
    const SEALED_CONTENTS_URL: &'static str = 
        "https://github.com/mtgjson/mtg-sealed-content/blob/main/outputs/contents.json?raw=true";
    const SEALED_PRODUCTS_URL: &'static str = 
        "https://github.com/mtgjson/mtg-sealed-content/blob/main/outputs/products.json?raw=true";

    pub fn new(config: MtgjsonConfig) -> Result<Self> {
        Ok(Self {
            provider: Arc::new(BaseProvider::new(config)?),
            sealed_products: OnceCell::new(),
            sealed_contents: OnceCell::new(),
        })
    }

    async fn ensure_data_loaded(&self) -> Result<()> {
        // Load sealed products if not already loaded
        if self.sealed_products.get().is_none() {
            let products = self.provider.download(Self::SEALED_PRODUCTS_URL).await?;
            if let Value::Object(products_map) = products {
                let products_hash: HashMap<String, HashMap<String, Value>> = products_map
                    .into_iter()
                    .filter_map(|(k, v)| {
                        if let Value::Object(inner) = v {
                            Some((k, inner))
                        } else {
                            None
                        }
                    })
                    .collect();
                self.sealed_products.set(products_hash).map_err(|_| {
                    ProviderError::ConfigError("Failed to set sealed products".to_string())
                })?;
            }
        }

        // Load sealed contents if not already loaded
        if self.sealed_contents.get().is_none() {
            let contents = self.provider.download(Self::SEALED_CONTENTS_URL).await?;
            if let Value::Object(contents_map) = contents {
                let contents_hash: HashMap<String, HashMap<String, Value>> = contents_map
                    .into_iter()
                    .filter_map(|(k, v)| {
                        if let Value::Object(inner) = v {
                            Some((k, inner))
                        } else {
                            None
                        }
                    })
                    .collect();
                self.sealed_contents.set(contents_hash).map_err(|_| {
                    ProviderError::ConfigError("Failed to set sealed contents".to_string())
                })?;
            }
        }

        Ok(())
    }

    pub async fn get_sealed_products_data(&self, set_code: &str) -> Result<Vec<SealedProductObject>> {
        self.ensure_data_loaded().await?;
        
        log::info!("Getting sealed product data for {}", set_code);
        
        let products = self.sealed_products.get().unwrap();
        let set_products = products.get(&set_code.to_lowercase());
        
        if let Some(set_data) = set_products {
            let mut products_list = Vec::new();
            
            for (product_name, product_data) in set_data {
                if let Value::Object(product_obj) = product_data {
                    let mut product = SealedProductObject {
                        name: product_name.clone(),
                        release_date: product_obj.get("release_date")
                            .and_then(|v| v.as_str()).map(String::from),
                        language: product_obj.get("language")
                            .and_then(|v| v.as_str()).map(String::from),
                        category: product_obj.get("category")
                            .and_then(|v| v.as_str())
                            .and_then(|s| serde_json::from_str(&format!("\"{}\"", s.to_uppercase())).ok()),
                        subtype: product_obj.get("subtype")
                            .and_then(|v| v.as_str())
                            .and_then(|s| serde_json::from_str(&format!("\"{}\"", s.to_uppercase())).ok()),
                        identifiers: SealedProductIdentifiers {
                            amazon_asin: None,
                            card_market: None,
                            tcg_player: None,
                        },
                        raw_purchase_urls: product_obj.get("purchase_url")
                            .and_then(|v| serde_json::from_value(v.clone()).ok())
                            .unwrap_or_default(),
                        product_size: None,
                        card_count: None,
                        contents: None,
                    };

                    // Handle identifiers
                    if let Some(Value::Object(identifiers)) = product_obj.get("identifiers") {
                        for (location, identifier) in identifiers {
                            let snake_case_location = to_snake_case(location);
                            let id_str = identifier.to_string().trim_matches('"').to_string();
                            
                            match snake_case_location.as_str() {
                                "amazon_asin" => product.identifiers.amazon_asin = Some(id_str),
                                "card_market" => product.identifiers.card_market = Some(id_str),
                                "tcg_player" => product.identifiers.tcg_player = Some(id_str),
                                _ => {}
                            }
                        }
                    }

                    products_list.push(product);
                }
            }
            
            Ok(products_list)
        } else {
            Ok(Vec::new())
        }
    }

    pub async fn apply_sealed_contents_data(
        &self, 
        set_code: &str, 
        sealed_products: &mut [SealedProductObject]
    ) -> Result<()> {
        self.ensure_data_loaded().await?;
        
        log::info!("Adding sealed product contents to {}", set_code);
        
        let contents = self.sealed_contents.get().unwrap();
        let set_contents = contents.get(&set_code.to_lowercase());
        
        if let Some(set_data) = set_contents {
            for product in sealed_products.iter_mut() {
                if let Some(Value::Object(product_contents)) = set_data.get(&product.name) {
                    let mut contents_clone = product_contents.clone();
                    
                    if let Some(size) = contents_clone.remove("size") {
                        product.product_size = size.as_u64().map(|v| v as u32);
                    }
                    
                    if let Some(card_count) = contents_clone.remove("card_count") {
                        product.card_count = card_count.as_u64().map(|v| v as u32);
                    }
                    
                    product.contents = Some(contents_clone);
                }
            }
        }
        
        Ok(())
    }
}

// GitHub Decks Provider
pub struct GitHubDecksProvider {
    provider: Arc<BaseProvider>,
    decks_by_set: OnceCell<HashMap<String, Vec<DeckObject>>>,
}

impl GitHubDecksProvider {
    const DECKS_API_URL: &'static str = 
        "https://github.com/taw/magic-preconstructed-decks-data/blob/master/decks_v2.json?raw=true";
    const DECKS_UUID_API_URL: &'static str = 
        "https://github.com/mtgjson/mtg-sealed-content/blob/main/outputs/deck_map.json?raw=True";

    pub fn new(config: MtgjsonConfig) -> Result<Self> {
        Ok(Self {
            provider: Arc::new(BaseProvider::new(config)?),
            decks_by_set: OnceCell::new(),
        })
    }

    async fn ensure_decks_loaded(&self) -> Result<()> {
        if self.decks_by_set.get().is_some() {
            return Ok(());
        }

        let decks_uuid_content = self.provider.download(Self::DECKS_UUID_API_URL).await?;
        let decks_data = self.provider.download(Self::DECKS_API_URL).await?;

        let mut decks_by_set: HashMap<String, Vec<DeckObject>> = HashMap::new();

        if let Value::Array(decks) = decks_data {
            for deck_value in decks {
                if let Value::Object(deck) = deck_value {
                    let set_code = deck.get("set_code")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_uppercase();
                    
                    let deck_name = deck.get("name")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();

                    let sealed_uuids = decks_uuid_content
                        .get(&set_code.to_lowercase())
                        .and_then(|v| v.get(&deck_name))
                        .and_then(|v| serde_json::from_value::<Vec<String>>(v.clone()).ok());

                    let mtgjson_deck = DeckObject {
                        name: deck_name,
                        code: set_code.clone(),
                        deck_type: deck.get("type")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string(),
                        release_date: deck.get("release_date")
                            .and_then(|v| v.as_str())
                            .map(String::from),
                        sealed_uuids,
                        main_board: Self::build_deck_cards(deck.get("cards")),
                        side_board: Self::build_deck_cards(deck.get("sideboard")),
                        display_commander: Self::build_deck_cards(deck.get("displayCommander")),
                        commander: Self::build_deck_cards(deck.get("commander")),
                        planes: Self::build_deck_cards(deck.get("planarDeck")),
                        schemes: Self::build_deck_cards(deck.get("schemeDeck")),
                    };

                    decks_by_set.entry(set_code).or_default().push(mtgjson_deck);
                }
            }
        }

        self.decks_by_set.set(decks_by_set).map_err(|_| {
            ProviderError::ConfigError("Failed to set decks by set".to_string())
        })?;

        Ok(())
    }

    fn build_deck_cards(cards_value: Option<&Value>) -> Vec<DeckCardObject> {
        if let Some(Value::Array(cards)) = cards_value {
            cards.iter()
                .filter_map(|card_value| {
                    if let Value::Object(card) = card_value {
                        Some(DeckCardObject {
                            uuid: card.get("mtgjson_uuid")
                                .and_then(|v| v.as_str())
                                .unwrap_or("")
                                .to_string(),
                            count: card.get("count")
                                .and_then(|v| v.as_u64())
                                .unwrap_or(1) as u32,
                            is_foil: card.get("foil")
                                .and_then(|v| v.as_bool())
                                .unwrap_or(false),
                        })
                    } else {
                        None
                    }
                })
                .collect()
        } else {
            Vec::new()
        }
    }

    pub async fn get_decks_in_set(&self, set_code: &str) -> Result<Vec<DeckObject>> {
        self.ensure_decks_loaded().await?;
        
        let decks = self.decks_by_set.get().unwrap();
        Ok(decks.get(set_code).cloned().unwrap_or_default())
    }
}

// GitHub Card Sealed Products Provider
pub struct GitHubCardSealedProductsProvider {
    provider: Arc<BaseProvider>,
    card_uuid_to_products: OnceCell<HashMap<String, HashMap<String, Vec<String>>>>,
}

impl GitHubCardSealedProductsProvider {
    const CARD_PRODUCTS_API_URL: &'static str = 
        "https://github.com/mtgjson/mtg-sealed-content/raw/main/outputs/card_map.json?raw=True";

    pub fn new(config: MtgjsonConfig) -> Result<Self> {
        Ok(Self {
            provider: Arc::new(BaseProvider::new(config)?),
            card_uuid_to_products: OnceCell::new(),
        })
    }

    async fn ensure_data_loaded(&self) -> Result<()> {
        if self.card_uuid_to_products.get().is_some() {
            return Ok(());
        }

        let card_data = self.provider.download(Self::CARD_PRODUCTS_API_URL).await?;
        
        if let Ok(card_map) = serde_json::from_value::<HashMap<String, HashMap<String, Vec<String>>>>(card_data) {
            self.card_uuid_to_products.set(card_map).map_err(|_| {
                ProviderError::ConfigError("Failed to set card UUID to products mapping".to_string())
            })?;
        }

        Ok(())
    }

    pub async fn get_products_card_found_in(
        &self, 
        mtgjson_uuid: &str
    ) -> Result<Option<HashMap<String, Vec<String>>>> {
        self.ensure_data_loaded().await?;
        
        let card_map = self.card_uuid_to_products.get().unwrap();
        Ok(card_map.get(mtgjson_uuid).cloned())
    }
}

// GitHub Boosters Provider
pub struct GitHubBoostersProvider {
    provider: Arc<BaseProvider>,
    booster_data: OnceCell<HashMap<String, Value>>,
}

impl GitHubBoostersProvider {
    const BOOSTER_API_URL: &'static str = 
        "https://github.com/taw/magic-sealed-data/blob/master/experimental_export_for_mtgjson.json?raw=true";

    pub fn new(config: MtgjsonConfig) -> Result<Self> {
        Ok(Self {
            provider: Arc::new(BaseProvider::new(config)?),
            booster_data: OnceCell::new(),
        })
    }

    async fn ensure_data_loaded(&self) -> Result<()> {
        if self.booster_data.get().is_some() {
            return Ok(());
        }

        let booster_data = self.provider.download(Self::BOOSTER_API_URL).await?;
        
        if let Value::Object(booster_map) = booster_data {
            self.booster_data.set(booster_map).map_err(|_| {
                ProviderError::ConfigError("Failed to set booster data".to_string())
            })?;
        }

        Ok(())
    }

    pub async fn get_set_booster_data(&self, set_code: &str) -> Result<Option<Value>> {
        self.ensure_data_loaded().await?;
        
        log::info!("Getting booster data for {}", set_code);
        
        let booster_data = self.booster_data.get().unwrap();
        let data = booster_data.get(&set_code.to_uppercase()).cloned();
        
        Ok(data.map(recursive_sort))
    }
}

pub struct GithubProvider {
    provider: Arc<BaseProvider>,
    sealed_products: OnceCell<HashMap<String, HashMap<String, Value>>>,
    sealed_contents: OnceCell<HashMap<String, HashMap<String, Value>>>,
    decks_by_set: OnceCell<HashMap<String, Vec<DeckObject>>>,
    card_uuid_to_products: OnceCell<HashMap<String, HashMap<String, Vec<String>>>>,
    booster_data: OnceCell<HashMap<String, Value>>,
}

impl GithubProvider {
    pub fn new(config: MtgjsonConfig) -> Result<Self> {
        Ok(Self {
            provider: Arc::new(BaseProvider::new(config)?),
        })
    }
}

// Utility functions
fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() && i > 0 {
            result.push('_');
        }
        result.push(c.to_lowercase().next().unwrap());
    }
    result
}

fn recursive_sort(value: Value) -> Value {
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

// Example usage and tests
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sealed_provider() {
        let config = MtgjsonConfig::default();
        let provider = GitHubSealedProvider::new(config).unwrap();
        
        let products = provider.get_sealed_products_data("khm").await.unwrap();
        assert!(!products.is_empty());
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    
    let config = MtgjsonConfig::default();
    
    // Example: Get sealed products for Kaldheim
    let sealed_provider = GitHubSealedProvider::new(config.clone())?;
    let mut products = sealed_provider.get_sealed_products_data("khm").await?;
    sealed_provider.apply_sealed_contents_data("khm", &mut products).await?;
    
    println!("Found {} sealed products for KHM", products.len());
    for product in products.iter().take(3) {
        println!("- {}", product.name);
    }
    
    // Example: Get decks
    let decks_provider = GitHubDecksProvider::new(config.clone())?;
    let decks = decks_provider.get_decks_in_set("KHM").await?;
    println!("Found {} decks for KHM", decks.len());
    
    // Example: Get booster data
    let boosters_provider = GitHubBoostersProvider::new(config.clone())?;
    if let Some(booster_data) = boosters_provider.get_set_booster_data("khm").await? {
        println!("Booster data found for KHM: {}", booster_data);
    }
    
    Ok(())
}