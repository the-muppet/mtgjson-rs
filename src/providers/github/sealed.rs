use serde_json::{Value, Map};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::OnceCell;

use super::{AbstractProvider, BaseProvider, MtgjsonConfig, ProviderError, Result, SealedProductObject, SealedProductIdentifiers, to_snake_case};

pub struct GitHubSealedProvider {
    provider: Arc<BaseProvider>,
    sealed_products: OnceCell<HashMap<String, Map<String, Value>>>,
    sealed_contents: OnceCell<HashMap<String, Map<String, Value>>>,
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
        if self.sealed_products.get().is_none() {
            let products = self.provider.download(Self::SEALED_PRODUCTS_URL).await?;
            if let Value::Object(products_map) = products {
                let products_hash: HashMap<String, Map<String, Value>> = products_map
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

        if self.sealed_contents.get().is_none() {
            let contents = self.provider.download(Self::SEALED_CONTENTS_URL).await?;
            if let Value::Object(contents_map) = contents {
                let contents_hash: HashMap<String, Map<String, Value>> = contents_map
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
