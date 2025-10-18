use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::OnceCell;

use super::{AbstractProvider, BaseProvider, DeckCardObject, DeckObject, MtgjsonConfig, ProviderError, Result};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_decks_provider() {
        let config = MtgjsonConfig::default();
        let provider = GitHubDecksProvider::new(config).unwrap();

        let decks = provider.get_decks_in_set("KHM").await.unwrap();
        assert!(!decks.is_empty());
    }
}
