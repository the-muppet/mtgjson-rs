use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::OnceCell;

use super::{AbstractProvider, BaseProvider, MtgjsonConfig, ProviderError, Result};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_card_sealed_products_provider() {
        let config = MtgjsonConfig::default();
        let provider = GitHubCardSealedProductsProvider::new(config).unwrap();

        let result = provider.get_products_card_found_in("test-uuid").await;
        assert!(result.is_ok());
    }
}
