use serde_json::{Value, Map};
use std::sync::Arc;
use tokio::sync::OnceCell;

use super::{AbstractProvider, BaseProvider, MtgjsonConfig, ProviderError, Result, recursive_sort};

pub struct GitHubBoostersProvider {
    provider: Arc<BaseProvider>,
    booster_data: OnceCell<Map<String, Value>>,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_booster_provider() {
        let config = MtgjsonConfig::default();
        let provider = GitHubBoostersProvider::new(config).unwrap();

        let booster_data = provider.get_set_booster_data("KHM").await.unwrap();
        assert!(booster_data.is_some());
    }
}
