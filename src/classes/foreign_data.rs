
use crate::base::{skip_if_empty_optional_string, JsonObject};
use crate::identifiers::MtgjsonIdentifiers;
use pyo3::prelude::*;

use pyo3::types::{PyAny, PyDict, PyString};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// MTGJSON Singular Card.ForeignData Object
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[pyclass(name = "MtgjsonForeignDataObject")]
pub struct MtgjsonForeignDataObject {
    #[pyo3(get, set)]
    pub language: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[pyo3(get, set)]
    pub multiverse_id: Option<i32>,
    #[pyo3(get, set)]
    pub identifiers: MtgjsonIdentifiers,
    #[serde(skip_serializing_if = "skip_if_empty_optional_string")]
    #[pyo3(get, set)]
    pub face_name: Option<String>,

    #[serde(skip_serializing_if = "skip_if_empty_optional_string")]
    #[pyo3(get, set)]
    pub flavor_text: Option<String>,

    #[serde(skip_serializing_if = "skip_if_empty_optional_string")]
    #[pyo3(get, set)]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "skip_if_empty_optional_string")]
    #[pyo3(get, set)]
    pub text: Option<String>,

    #[serde(skip_serializing_if = "skip_if_empty_optional_string")]
    #[pyo3(get, set)]
    pub type_: Option<String>,
}

#[pymethods]
impl MtgjsonForeignDataObject {
    #[new]
    pub fn new() -> Self {
        Self {
            face_name: None,
            flavor_text: None,
            language: String::new(),
            multiverse_id: None,
            name: None,
            text: None,
            type_: None,
            identifiers: MtgjsonIdentifiers::new(),
        }
    }

    /// Convert to JSON string
    pub fn to_json_string(&self) -> PyResult<String> {
        serde_json::to_string(self)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
    }

    /// Convert to dictionary for Python compatibility
    pub fn to_dict(&self, py: Python) -> PyResult<Bound<'_, PyDict>> {
        let result = PyDict::new_bound(py);

        result.set_item("language", &self.language)?;

        if let Some(val) = self.multiverse_id {
            result.set_item("multiverseId", &val)?;
        }

        let identifiers_dict = self.identifiers.to_dict()?;
        result.set_item("identifiers", &identifiers_dict)?;

        if let Some(ref val) = self.face_name {
            if !val.is_empty() {
                result.set_item("faceName", val)?;
            }
        }

        if let Some(ref val) = self.flavor_text {
            if !val.is_empty() {
                result.set_item("flavorText", val)?;
            }
        }

        if let Some(ref val) = self.name {
            if !val.is_empty() {
                result.set_item("name", val)?;
            }
        }

        if let Some(ref val) = self.text {
            if !val.is_empty() {
                result.set_item("text", val)?;
            }
        }

        if let Some(ref val) = self.type_ {
            if !val.is_empty() {
                result.set_item("type", val)?;
            }
        }

        Ok(result)
    }

    pub fn to_dict_bound(&self, py: Python) -> PyResult<Bound<'_, PyDict>> {
        self.to_dict(py)
    }

    /// Check if foreign data has meaningful content
    pub fn has_content(&self) -> bool {
        self.name.is_some()
            || self.text.is_some()
            || self.flavor_text.is_some()
            || self.type_.is_some()
            || self.face_name.is_some()
    }

    /// Get display name (face_name if available, otherwise name)
    pub fn get_display_name(&self) -> Option<String> {
        self.face_name.clone().or_else(|| self.name.clone())
    }
}

impl Default for MtgjsonForeignDataObject {
    fn default() -> Self {
        Self::new()
    }
}

impl JsonObject for MtgjsonForeignDataObject {
    fn build_keys_to_skip(&self) -> HashSet<String> {
        let mut keys_to_skip = HashSet::new();
        keys_to_skip.insert("url".to_string());
        keys_to_skip.insert("number".to_string());
        keys_to_skip.insert("set_code".to_string());
        keys_to_skip
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foreign_data_creation() {
        let foreign_data = MtgjsonForeignDataObject::new();
        assert_eq!(foreign_data.face_name, None);
        assert_eq!(foreign_data.flavor_text, None);
        assert_eq!(foreign_data.language, "");
        assert_eq!(foreign_data.multiverse_id, None);
        assert_eq!(foreign_data.name, None);
        assert_eq!(foreign_data.text, None);
        assert_eq!(foreign_data.type_, None);
    }

    #[test]
    fn test_foreign_data_setters() {
        let mut foreign_data = MtgjsonForeignDataObject::new();

        foreign_data.language = "Japanese".to_string();
        foreign_data.name = Some("稲妻".to_string());
        foreign_data.text = Some(
            "クリーチャー1体かプレイヤー1人を対象とする。稲妻はそれに3点のダメージを与える。"
                .to_string(),
        );
        foreign_data.flavor_text = Some("雷の力".to_string());
        foreign_data.face_name = Some("表面".to_string());
        foreign_data.type_ = Some("インスタント".to_string());
        foreign_data.multiverse_id = Some(12345);

        assert_eq!(foreign_data.language, "Japanese");
        assert_eq!(foreign_data.name, Some("稲妻".to_string()));
        assert_eq!(
            foreign_data.text,
            Some(
                "クリーチャー1体かプレイヤー1人を対象とする。稲妻はそれに3点のダメージを与える。"
                    .to_string()
            )
        );
        assert_eq!(foreign_data.flavor_text, Some("雷の力".to_string()));
        assert_eq!(foreign_data.face_name, Some("表面".to_string()));
        assert_eq!(foreign_data.type_, Some("インスタント".to_string()));
        assert_eq!(foreign_data.multiverse_id, Some(12345));
    }

    #[test]
    fn test_foreign_data_languages() {
        let mut foreign_data = MtgjsonForeignDataObject::new();

        // Test various languages
        let languages = vec![
            "English",
            "Spanish",
            "French",
            "German",
            "Italian",
            "Portuguese",
            "Japanese",
            "Korean",
            "Russian",
            "Simplified Chinese",
            "Traditional Chinese",
        ];

        for language in languages {
            foreign_data.language = language.to_string();
            assert_eq!(foreign_data.language, language);
        }
    }

    #[test]
    fn test_foreign_data_special_characters() {
        let mut foreign_data = MtgjsonForeignDataObject::new();

        // Test special characters in different languages
        foreign_data.name = Some("Tëst Cärd with Spëcial Chärs".to_string());
        foreign_data.text = Some("Tëxt with ümlauts änd àccents éñç".to_string());
        foreign_data.flavor_text = Some("Flävör tëxt with spëcial châractërs".to_string());

        assert_eq!(
            foreign_data.name,
            Some("Tëst Cärd with Spëcial Chärs".to_string())
        );
        assert_eq!(
            foreign_data.text,
            Some("Tëxt with ümlauts änd àccents éñç".to_string())
        );
        assert_eq!(
            foreign_data.flavor_text,
            Some("Flävör tëxt with spëcial châractërs".to_string())
        );
    }

    #[test]
    fn test_foreign_data_unicode() {
        let mut foreign_data = MtgjsonForeignDataObject::new();

        // Test Unicode characters
        foreign_data.name = Some("🔥Lightning Bolt⚡".to_string());
        foreign_data.text = Some("Deal 3 damage to any target 🎯".to_string());

        assert!(foreign_data.name.as_ref().unwrap().contains("🔥"));
        assert!(foreign_data.name.as_ref().unwrap().contains("⚡"));
        assert!(foreign_data.text.as_ref().unwrap().contains("🎯"));
    }

    #[test]
    fn test_foreign_data_empty_values() {
        let mut foreign_data = MtgjsonForeignDataObject::new();

        // Test empty strings
        foreign_data.language = "".to_string();
        foreign_data.name = Some("".to_string());
        foreign_data.text = Some("".to_string());
        foreign_data.flavor_text = Some("".to_string());

        assert_eq!(foreign_data.language, "");
        assert_eq!(foreign_data.name, Some("".to_string()));
        assert_eq!(foreign_data.text, Some("".to_string()));
        assert_eq!(foreign_data.flavor_text, Some("".to_string()));
    }

    #[test]
    fn test_foreign_data_long_text() {
        let mut foreign_data = MtgjsonForeignDataObject::new();

        // Test very long text
        let long_text =
            "This is a very long text that simulates a card with extensive rules text. "
                .repeat(100);
        foreign_data.text = Some(long_text.clone());

        assert_eq!(foreign_data.text, Some(long_text));
        assert!(foreign_data.text.as_ref().unwrap().len() > 1000);
    }

    #[test]
    fn test_foreign_data_multiverse_id_edge_cases() {
        let mut foreign_data = MtgjsonForeignDataObject::new();

        // Test edge cases for multiverse_id
        foreign_data.multiverse_id = Some(0);
        assert_eq!(foreign_data.multiverse_id, Some(0));

        foreign_data.multiverse_id = Some(i32::MAX);
        assert_eq!(foreign_data.multiverse_id, Some(i32::MAX));

        foreign_data.multiverse_id = None;
        assert_eq!(foreign_data.multiverse_id, None);
    }

    #[test]
    fn test_foreign_data_json_serialization() {
        let mut foreign_data = MtgjsonForeignDataObject::new();
        foreign_data.language = "French".to_string();
        foreign_data.name = Some("Éclair".to_string());
        foreign_data.text = Some("Infligez 3 blessures à n'importe quelle cible.".to_string());
        foreign_data.multiverse_id = Some(54321);

        let json_result = serde_json::to_string(&foreign_data);
        assert!(json_result.is_ok());

        let json_string = json_result.unwrap();
        assert!(json_string.contains("French"));
        assert!(json_string.contains("Éclair"));
        assert!(json_string.contains("54321"));

        // Test deserialization
        let deserialized: MtgjsonForeignDataObject = serde_json::from_str(&json_string).unwrap();
        assert_eq!(deserialized.language, "French");
        assert_eq!(deserialized.name, Some("Éclair".to_string()));
        assert_eq!(deserialized.multiverse_id, Some(54321));
    }

    #[test]
    fn test_foreign_data_default_trait() {
        let foreign_data = MtgjsonForeignDataObject::default();
        assert_eq!(foreign_data.language, "");
        assert_eq!(foreign_data.name, None);
        assert_eq!(foreign_data.text, None);
        assert_eq!(foreign_data.flavor_text, None);
        assert_eq!(foreign_data.face_name, None);
        assert_eq!(foreign_data.type_, None);
        assert_eq!(foreign_data.multiverse_id, None);
    }

    #[test]
    fn test_foreign_data_clone() {
        let mut original = MtgjsonForeignDataObject::new();
        original.language = "Spanish".to_string();
        original.name = Some("Rayo".to_string());
        original.multiverse_id = Some(98765);

        let cloned = original.clone();
        assert_eq!(cloned.language, "Spanish");
        assert_eq!(cloned.name, Some("Rayo".to_string()));
        assert_eq!(cloned.multiverse_id, Some(98765));

        // Verify independence
        assert_eq!(original.language, cloned.language);
    }

    #[test]
    fn test_foreign_data_equality() {
        let mut foreign_data1 = MtgjsonForeignDataObject::new();
        let mut foreign_data2 = MtgjsonForeignDataObject::new();

        foreign_data1.language = "German".to_string();
        foreign_data1.name = Some("Blitz".to_string());

        foreign_data2.language = "German".to_string();
        foreign_data2.name = Some("Blitz".to_string());

        assert_eq!(foreign_data1, foreign_data2);

        foreign_data2.name = Some("Blitzschlag".to_string());
        assert_ne!(foreign_data1, foreign_data2);
    }

    #[test]
    fn test_foreign_data_partial_eq() {
        let foreign_data1 = MtgjsonForeignDataObject {
            language: "Italian".to_string(),
            name: Some("Fulmine".to_string()),
            text: Some("Infliggi 3 danni a una qualsiasi creatura o giocatore.".to_string()),
            flavor_text: None,
            face_name: None,
            type_: Some("Istantaneo".to_string()),
            multiverse_id: Some(11111),
            identifiers: MtgjsonIdentifiers::new(),
        };

        let foreign_data2 = MtgjsonForeignDataObject {
            language: "Italian".to_string(),
            name: Some("Fulmine".to_string()),
            text: Some("Infliggi 3 danni a una qualsiasi creatura o giocatore.".to_string()),
            flavor_text: None,
            face_name: None,
            type_: Some("Istantaneo".to_string()),
            multiverse_id: Some(11111),
            identifiers: MtgjsonIdentifiers::new(),
        };

        assert_eq!(foreign_data1, foreign_data2);
    }

    #[test]
    fn test_foreign_data_validation() {
        let mut foreign_data = MtgjsonForeignDataObject::new();

        // Test that we can set valid language codes
        let valid_languages = vec![
            "en", "es", "fr", "de", "it", "pt", "ja", "ko", "ru", "zh-CN", "zh-TW",
        ];

        for lang in valid_languages {
            foreign_data.language = lang.to_string();
            assert_eq!(foreign_data.language, lang);
        }
    }

    #[test]
    fn test_foreign_data_comprehensive_fields() {
        let foreign_data = MtgjsonForeignDataObject {
            language: "Portuguese".to_string(),
            name: Some("Raio".to_string()),
            text: Some("Cause 3 pontos de dano a qualquer alvo.".to_string()),
            flavor_text: Some("Rápido como o raio.".to_string()),
            face_name: Some("Frente".to_string()),
            type_: Some("Mágica Instantânea".to_string()),
            multiverse_id: Some(77777),
            identifiers: MtgjsonIdentifiers::new(),
        };

        // Verify all fields are set correctly
        assert_eq!(foreign_data.language, "Portuguese");
        assert_eq!(foreign_data.name, Some("Raio".to_string()));
        assert_eq!(
            foreign_data.text,
            Some("Cause 3 pontos de dano a qualquer alvo.".to_string())
        );
        assert_eq!(
            foreign_data.flavor_text,
            Some("Rápido como o raio.".to_string())
        );
        assert_eq!(foreign_data.face_name, Some("Frente".to_string()));
        assert_eq!(foreign_data.type_, Some("Mágica Instantânea".to_string()));
        assert_eq!(foreign_data.multiverse_id, Some(77777));
    }

    #[test]
    fn test_to_dict_basic() {
        pyo3::prepare_freethreaded_python();

        Python::with_gil(|py| {
            let mut foreign_data = MtgjsonForeignDataObject::new();
            foreign_data.language = "Japanese".to_string();
            foreign_data.name = Some("稲妻".to_string());
            foreign_data.multiverse_id = Some(12345);

            let dict_result = foreign_data.to_dict(py);
            assert!(dict_result.is_ok());

            let dict = dict_result.unwrap();
            assert!(dict.contains("language").unwrap());
            assert!(dict.contains("name").unwrap());
            assert!(dict.contains("multiverseId").unwrap());
            assert!(dict.contains("identifiers").unwrap());
        });
    }

    #[test]
    fn test_to_dict_with_all_fields() {
        pyo3::prepare_freethreaded_python();

        Python::with_gil(|py| {
            let mut foreign_data = MtgjsonForeignDataObject::new();
            foreign_data.language = "French".to_string();
            foreign_data.name = Some("Éclair".to_string());
            foreign_data.text = Some("Infligez 3 blessures.".to_string());
            foreign_data.flavor_text = Some("Rapide!".to_string());
            foreign_data.face_name = Some("Avant".to_string());
            foreign_data.type_ = Some("Éphémère".to_string());
            foreign_data.multiverse_id = Some(54321);

            let dict_result = foreign_data.to_dict(py);
            assert!(dict_result.is_ok());

            let dict = dict_result.unwrap();
            assert!(dict.contains("language").unwrap());
            assert!(dict.contains("name").unwrap());
            assert!(dict.contains("text").unwrap());
            assert!(dict.contains("flavorText").unwrap());
            assert!(dict.contains("faceName").unwrap());
            assert!(dict.contains("type").unwrap());
            assert!(dict.contains("multiverseId").unwrap());
        });
    }

    #[test]
    fn test_to_dict_skips_empty_strings() {
        pyo3::prepare_freethreaded_python();

        Python::with_gil(|py| {
            let mut foreign_data = MtgjsonForeignDataObject::new();
            foreign_data.language = "German".to_string();
            foreign_data.name = Some("".to_string());
            foreign_data.text = Some("".to_string());

            let dict_result = foreign_data.to_dict(py);
            assert!(dict_result.is_ok());

            let dict = dict_result.unwrap();
            assert!(dict.contains("language").unwrap());
            assert!(!dict.contains("name").unwrap());
            assert!(!dict.contains("text").unwrap());
        });
    }

    #[test]
    fn test_to_dict_bound() {
        pyo3::prepare_freethreaded_python();

        Python::with_gil(|py| {
            let mut foreign_data = MtgjsonForeignDataObject::new();
            foreign_data.language = "Spanish".to_string();
            foreign_data.name = Some("Rayo".to_string());
            foreign_data.multiverse_id = Some(98765);

            let dict_result = foreign_data.to_dict_bound(py);
            assert!(dict_result.is_ok());

            let dict = dict_result.unwrap();
            assert!(dict.contains("language").unwrap());
            assert!(dict.contains("name").unwrap());
            assert!(dict.contains("multiverseId").unwrap());
        });
    }

    #[test]
    fn test_to_dict_bound_matches_to_dict() {
        pyo3::prepare_freethreaded_python();

        Python::with_gil(|py| {
            let mut foreign_data = MtgjsonForeignDataObject::new();
            foreign_data.language = "Italian".to_string();
            foreign_data.name = Some("Fulmine".to_string());
            foreign_data.text = Some("Infliggi 3 danni.".to_string());
            foreign_data.multiverse_id = Some(11111);

            let dict1 = foreign_data.to_dict(py).unwrap();
            let dict2 = foreign_data.to_dict_bound(py).unwrap();

            assert_eq!(dict1.len(), dict2.len());
            assert!(dict2.contains("language").unwrap());
            assert!(dict2.contains("name").unwrap());
            assert!(dict2.contains("text").unwrap());
        });
    }

    #[test]
    fn test_to_dict_lifetime_safety() {
        pyo3::prepare_freethreaded_python();

        Python::with_gil(|py| {
            let foreign_data = MtgjsonForeignDataObject {
                language: "Korean".to_string(),
                name: Some("번개".to_string()),
                text: Some("피해 3점".to_string()),
                flavor_text: None,
                face_name: None,
                type_: Some("순간마법".to_string()),
                multiverse_id: Some(99999),
                identifiers: MtgjsonIdentifiers::new(),
            };

            let dict = foreign_data.to_dict(py).unwrap();

            assert!(dict.contains("language").unwrap());
            assert!(dict.contains("name").unwrap());
            assert!(dict.contains("text").unwrap());
            assert!(dict.contains("type").unwrap());
            assert!(!dict.contains("flavorText").unwrap());
            assert!(!dict.contains("faceName").unwrap());
        });
    }

    #[test]
    fn test_has_content() {
        let mut foreign_data = MtgjsonForeignDataObject::new();
        assert!(!foreign_data.has_content());

        foreign_data.name = Some("Test".to_string());
        assert!(foreign_data.has_content());

        foreign_data.name = None;
        foreign_data.text = Some("Text".to_string());
        assert!(foreign_data.has_content());

        foreign_data.text = None;
        foreign_data.flavor_text = Some("Flavor".to_string());
        assert!(foreign_data.has_content());
    }

    #[test]
    fn test_get_display_name() {
        let mut foreign_data = MtgjsonForeignDataObject::new();
        assert_eq!(foreign_data.get_display_name(), None);

        foreign_data.name = Some("Card Name".to_string());
        assert_eq!(foreign_data.get_display_name(), Some("Card Name".to_string()));

        foreign_data.face_name = Some("Face Name".to_string());
        assert_eq!(foreign_data.get_display_name(), Some("Face Name".to_string()));
    }
}
