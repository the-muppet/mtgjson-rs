use mtgjson_rust::classes::*;
use std::collections::HashMap;

mod comprehensive_translations_tests {
    use super::*;
    use mtgjson_rust::classes::translations::MtgjsonTranslations;

    /// Test all MtgjsonTranslations constructors and return types
    #[test]
    fn test_translations_constructors_return_types() {
        // Test default constructor with None
        let default_translations = MtgjsonTranslations::new(None);
        assert_eq!(default_translations.chinese_simplified, None);
        assert_eq!(default_translations.chinese_traditional, None);
        assert_eq!(default_translations.french, None);
        assert_eq!(default_translations.german, None);
        assert_eq!(default_translations.italian, None);
        assert_eq!(default_translations.japanese, None);
        assert_eq!(default_translations.korean, None);
        assert_eq!(default_translations.portuguese_brazil, None);
        assert_eq!(default_translations.russian, None);
        assert_eq!(default_translations.spanish, None);
        
        // Test Default trait constructor
        let default_trait_translations = MtgjsonTranslations::default();
        assert_eq!(default_trait_translations.chinese_simplified, None);
        assert_eq!(default_trait_translations.french, None);
        assert_eq!(default_trait_translations.german, None);
        
        // Verify return types
        let chinese_simplified: Option<String> = default_translations.chinese_simplified.clone();
        let chinese_traditional: Option<String> = default_translations.chinese_traditional.clone();
        let french: Option<String> = default_translations.french.clone();
        let german: Option<String> = default_translations.german.clone();
        let italian: Option<String> = default_translations.italian.clone();
        let japanese: Option<String> = default_translations.japanese.clone();
        let korean: Option<String> = default_translations.korean.clone();
        let portuguese_brazil: Option<String> = default_translations.portuguese_brazil.clone();
        let russian: Option<String> = default_translations.russian.clone();
        let spanish: Option<String> = default_translations.spanish.clone();
        
        assert_eq!(chinese_simplified, None);
        assert_eq!(chinese_traditional, None);
        assert_eq!(french, None);
        assert_eq!(german, None);
        assert_eq!(italian, None);
        assert_eq!(japanese, None);
        assert_eq!(korean, None);
        assert_eq!(portuguese_brazil, None);
        assert_eq!(russian, None);
        assert_eq!(spanish, None);
    }

    /// Test constructor with HashMap input and return types
    #[test]
    fn test_translations_constructor_with_dict_return_types() {
        let mut translation_dict = HashMap::new();
        translation_dict.insert("Chinese Simplified".to_string(), "简体中文测试".to_string());
        translation_dict.insert("Chinese Traditional".to_string(), "繁體中文測試".to_string());
        translation_dict.insert("French".to_string(), "Test français".to_string());
        translation_dict.insert("German".to_string(), "Deutscher Test".to_string());
        translation_dict.insert("Italian".to_string(), "Test italiano".to_string());
        translation_dict.insert("Japanese".to_string(), "日本語テスト".to_string());
        translation_dict.insert("Korean".to_string(), "한국어 테스트".to_string());
        translation_dict.insert("Portuguese (Brazil)".to_string(), "Teste português brasileiro".to_string());
        translation_dict.insert("Russian".to_string(), "Русский тест".to_string());
        translation_dict.insert("Spanish".to_string(), "Prueba en español".to_string());
        
        let translations = MtgjsonTranslations::new(Some(translation_dict));
        
        // Verify return types and values
        let chinese_simplified: Option<String> = translations.chinese_simplified.clone();
        let chinese_traditional: Option<String> = translations.chinese_traditional.clone();
        let french: Option<String> = translations.french.clone();
        let german: Option<String> = translations.german.clone();
        let italian: Option<String> = translations.italian.clone();
        let japanese: Option<String> = translations.japanese.clone();
        let korean: Option<String> = translations.korean.clone();
        let portuguese_brazil: Option<String> = translations.portuguese_brazil.clone();
        let russian: Option<String> = translations.russian.clone();
        let spanish: Option<String> = translations.spanish.clone();
        
        assert_eq!(chinese_simplified, Some("简体中文测试".to_string()));
        assert_eq!(chinese_traditional, Some("繁體中文測試".to_string()));
        assert_eq!(french, Some("Test français".to_string()));
        assert_eq!(german, Some("Deutscher Test".to_string()));
        assert_eq!(italian, Some("Test italiano".to_string()));
        assert_eq!(japanese, Some("日本語テスト".to_string()));
        assert_eq!(korean, Some("한국어 테스트".to_string()));
        assert_eq!(portuguese_brazil, Some("Teste português brasileiro".to_string()));
        assert_eq!(russian, Some("Русский тест".to_string()));
        assert_eq!(spanish, Some("Prueba en español".to_string()));
    }

    /// Test alternative key formats and return types
    #[test]
    fn test_translations_alternative_keys_return_types() {
        let mut translation_dict = HashMap::new();
        translation_dict.insert("fr".to_string(), "Français alternatif".to_string());
        translation_dict.insert("de".to_string(), "Deutsches Alternativ".to_string());
        translation_dict.insert("it".to_string(), "Italiano alternativo".to_string());
        translation_dict.insert("es".to_string(), "Español alternativo".to_string());
        
        let translations = MtgjsonTranslations::new(Some(translation_dict));
        
        // Test that alternative keys work
        let french: Option<String> = translations.french.clone();
        let german: Option<String> = translations.german.clone();
        let italian: Option<String> = translations.italian.clone();
        let spanish: Option<String> = translations.spanish.clone();
        
        assert_eq!(french, Some("Français alternatif".to_string()));
        assert_eq!(german, Some("Deutsches Alternativ".to_string()));
        assert_eq!(italian, Some("Italiano alternativo".to_string()));
        assert_eq!(spanish, Some("Español alternativo".to_string()));
    }

    /// Test parse_key static method return types
    #[test]
    fn test_translations_parse_key_return_types() {
        // Test parse_key static method return type
        let parsed_chinese_simplified: String = MtgjsonTranslations::parse_key("chinese_simplified");
        assert_eq!(parsed_chinese_simplified, "Chinese Simplified");
        
        let parsed_chinese_traditional: String = MtgjsonTranslations::parse_key("chinese_traditional");
        assert_eq!(parsed_chinese_traditional, "Chinese Traditional");
        
        let parsed_portuguese_brazil: String = MtgjsonTranslations::parse_key("portuguese_brazil");
        assert_eq!(parsed_portuguese_brazil, "Portuguese (Brazil)");
        
        let parsed_french: String = MtgjsonTranslations::parse_key("french");
        assert_eq!(parsed_french, "French");
        
        let parsed_german: String = MtgjsonTranslations::parse_key("german");
        assert_eq!(parsed_german, "German");
        
        let parsed_italian: String = MtgjsonTranslations::parse_key("italian");
        assert_eq!(parsed_italian, "Italian");
        
        let parsed_japanese: String = MtgjsonTranslations::parse_key("japanese");
        assert_eq!(parsed_japanese, "Japanese");
        
        let parsed_korean: String = MtgjsonTranslations::parse_key("korean");
        assert_eq!(parsed_korean, "Korean");
        
        let parsed_russian: String = MtgjsonTranslations::parse_key("russian");
        assert_eq!(parsed_russian, "Russian");
        
        let parsed_spanish: String = MtgjsonTranslations::parse_key("spanish");
        assert_eq!(parsed_spanish, "Spanish");
        
        // Test edge cases
        let parsed_empty: String = MtgjsonTranslations::parse_key("");
        assert_eq!(parsed_empty, "");
        
        let parsed_single: String = MtgjsonTranslations::parse_key("single");
        assert_eq!(parsed_single, "Single");
        
        let parsed_multiple_underscores: String = MtgjsonTranslations::parse_key("multiple_word_test");
        assert_eq!(parsed_multiple_underscores, "Multiple Word Test");
    }

    /// Test to_json method return types
    #[test]
    fn test_translations_to_json_return_types() {
        let mut translations = MtgjsonTranslations::new(None);
        translations.french = Some("Test français".to_string());
        translations.german = Some("Deutscher Test".to_string());
        translations.spanish = Some("Prueba en español".to_string());
        
        // Test to_json method return type
        let json_result: Result<String, pyo3::PyErr> = translations.to_json();
        assert!(json_result.is_ok());
        let json_string: String = json_result.unwrap();
        assert!(!json_string.is_empty());
        assert!(json_string.contains("Test français"));
        assert!(json_string.contains("Deutscher Test"));
        assert!(json_string.contains("Prueba en español"));
    }

    /// Test to_dict method return types
    #[test]
    fn test_translations_to_dict_return_types() {
        let mut translations = MtgjsonTranslations::new(None);
        translations.chinese_simplified = Some("简体中文".to_string());
        translations.french = Some("Français".to_string());
        translations.german = Some("Deutsch".to_string());
        translations.japanese = Some("日本語".to_string());
        translations.portuguese_brazil = Some("Português brasileiro".to_string());
        
        // Test to_dict method return type
        let dict_result: Result<HashMap<String, String>, pyo3::PyErr> = translations.to_dict();
        assert!(dict_result.is_ok());
        let dict: HashMap<String, String> = dict_result.unwrap();
        
        // Verify return type and content
        assert_eq!(dict.len(), 5);
        assert_eq!(dict.get("Chinese Simplified"), Some(&"简体中文".to_string()));
        assert_eq!(dict.get("French"), Some(&"Français".to_string()));
        assert_eq!(dict.get("German"), Some(&"Deutsch".to_string()));
        assert_eq!(dict.get("Japanese"), Some(&"日本語".to_string()));
        assert_eq!(dict.get("Portuguese (Brazil)"), Some(&"Português brasileiro".to_string()));
        
        // Test that empty strings are excluded
        let mut empty_translations = MtgjsonTranslations::new(None);
        empty_translations.french = Some("".to_string()); // Empty string should be excluded
        empty_translations.german = Some("Deutsch".to_string());
        
        let empty_dict_result: Result<HashMap<String, String>, pyo3::PyErr> = empty_translations.to_dict();
        assert!(empty_dict_result.is_ok());
        let empty_dict: HashMap<String, String> = empty_dict_result.unwrap();
        assert_eq!(empty_dict.len(), 1); // Only German should be included
        assert_eq!(empty_dict.get("German"), Some(&"Deutsch".to_string()));
        assert!(empty_dict.get("French").is_none());
    }

    /// Test get_available_languages method return types
    #[test]
    fn test_translations_get_available_languages_return_types() {
        let mut translations = MtgjsonTranslations::new(None);
        
        // Test with no languages
        let no_languages: Vec<String> = translations.get_available_languages();
        assert_eq!(no_languages.len(), 0);
        
        // Test with some languages
        translations.french = Some("Français".to_string());
        translations.german = Some("Deutsch".to_string());
        translations.japanese = Some("日本語".to_string());
        
        let some_languages: Vec<String> = translations.get_available_languages();
        assert_eq!(some_languages.len(), 3);
        assert!(some_languages.contains(&"French".to_string()));
        assert!(some_languages.contains(&"German".to_string()));
        assert!(some_languages.contains(&"Japanese".to_string()));
        
        // Test with all languages
        translations.chinese_simplified = Some("简体中文".to_string());
        translations.chinese_traditional = Some("繁體中文".to_string());
        translations.italian = Some("Italiano".to_string());
        translations.korean = Some("한국어".to_string());
        translations.portuguese_brazil = Some("Português".to_string());
        translations.russian = Some("Русский".to_string());
        translations.spanish = Some("Español".to_string());
        
        let all_languages: Vec<String> = translations.get_available_languages();
        assert_eq!(all_languages.len(), 10);
        assert!(all_languages.contains(&"Chinese Simplified".to_string()));
        assert!(all_languages.contains(&"Chinese Traditional".to_string()));
        assert!(all_languages.contains(&"French".to_string()));
        assert!(all_languages.contains(&"German".to_string()));
        assert!(all_languages.contains(&"Italian".to_string()));
        assert!(all_languages.contains(&"Japanese".to_string()));
        assert!(all_languages.contains(&"Korean".to_string()));
        assert!(all_languages.contains(&"Portuguese (Brazil)".to_string()));
        assert!(all_languages.contains(&"Russian".to_string()));
        assert!(all_languages.contains(&"Spanish".to_string()));
    }

    /// Test has_translations method return types
    #[test]
    fn test_translations_has_translations_return_types() {
        let mut translations = MtgjsonTranslations::new(None);
        
        // Test with no translations
        let has_none: bool = translations.has_translations();
        assert_eq!(has_none, false);
        
        // Test with one translation
        translations.french = Some("Français".to_string());
        let has_one: bool = translations.has_translations();
        assert_eq!(has_one, true);
        
        // Test with multiple translations
        translations.german = Some("Deutsch".to_string());
        translations.spanish = Some("Español".to_string());
        let has_multiple: bool = translations.has_translations();
        assert_eq!(has_multiple, true);
        
        // Test with all translations set to None again
        let no_translations = MtgjsonTranslations::new(None);
        let has_none_again: bool = no_translations.has_translations();
        assert_eq!(has_none_again, false);
    }

    /// Test field getter/setter return types
    #[test]
    fn test_translations_field_return_types() {
        let mut translations = MtgjsonTranslations::new(None);
        
        // Test all language field assignments and retrievals
        translations.chinese_simplified = Some("简体中文测试".to_string());
        let chinese_simplified: Option<String> = translations.chinese_simplified.clone();
        assert_eq!(chinese_simplified, Some("简体中文测试".to_string()));
        
        translations.chinese_traditional = Some("繁體中文測試".to_string());
        let chinese_traditional: Option<String> = translations.chinese_traditional.clone();
        assert_eq!(chinese_traditional, Some("繁體中文測試".to_string()));
        
        translations.french = Some("Test en français avec accents: àáâãäèéêëìíîïòóôõöùúûü".to_string());
        let french: Option<String> = translations.french.clone();
        assert_eq!(french, Some("Test en français avec accents: àáâãäèéêëìíîïòóôõöùúûü".to_string()));
        
        translations.german = Some("Deutscher Test mit Umlauten: äöüß".to_string());
        let german: Option<String> = translations.german.clone();
        assert_eq!(german, Some("Deutscher Test mit Umlauten: äöüß".to_string()));
        
        translations.italian = Some("Test italiano con caratteri speciali: àèéìíîòóù".to_string());
        let italian: Option<String> = translations.italian.clone();
        assert_eq!(italian, Some("Test italiano con caratteri speciali: àèéìíîòóù".to_string()));
        
        translations.japanese = Some("日本語テスト：ひらがな、カタカナ、漢字".to_string());
        let japanese: Option<String> = translations.japanese.clone();
        assert_eq!(japanese, Some("日本語テスト：ひらがな、カタカナ、漢字".to_string()));
        
        translations.korean = Some("한국어 테스트: 한글 문자".to_string());
        let korean: Option<String> = translations.korean.clone();
        assert_eq!(korean, Some("한국어 테스트: 한글 문자".to_string()));
        
        translations.portuguese_brazil = Some("Teste em português brasileiro: ção, ão, ã".to_string());
        let portuguese_brazil: Option<String> = translations.portuguese_brazil.clone();
        assert_eq!(portuguese_brazil, Some("Teste em português brasileiro: ção, ão, ã".to_string()));
        
        translations.russian = Some("Русский тест: кириллица".to_string());
        let russian: Option<String> = translations.russian.clone();
        assert_eq!(russian, Some("Русский тест: кириллица".to_string()));
        
        translations.spanish = Some("Prueba en español: ñáéíóúü".to_string());
        let spanish: Option<String> = translations.spanish.clone();
        assert_eq!(spanish, Some("Prueba en español: ñáéíóúü".to_string()));
    }

    /// Test edge cases and error conditions with return types
    #[test]
    fn test_translations_edge_cases_return_types() {
        let mut translations = MtgjsonTranslations::new(None);
        
        // Test empty strings
        translations.french = Some("".to_string());
        translations.german = Some("".to_string());
        
        let empty_french: Option<String> = translations.french.clone();
        let empty_german: Option<String> = translations.german.clone();
        assert_eq!(empty_french, Some("".to_string()));
        assert_eq!(empty_german, Some("".to_string()));
        
        // Test very long strings
        let long_text = "Very long translation text ".repeat(100);
        translations.spanish = Some(long_text.clone());
        let long_spanish: Option<String> = translations.spanish.clone();
        assert_eq!(long_spanish, Some(long_text));
        
        // Test special characters and symbols
        translations.italian = Some("Test with symbols: ©®™€$¥£¢".to_string());
        let symbol_italian: Option<String> = translations.italian.clone();
        assert_eq!(symbol_italian, Some("Test with symbols: ©®™€$¥£¢".to_string()));
        
        // Test numbers and mixed content
        translations.japanese = Some("Mixed content: 123 数字 ABC アルファベット".to_string());
        let mixed_japanese: Option<String> = translations.japanese.clone();
        assert_eq!(mixed_japanese, Some("Mixed content: 123 数字 ABC アルファベット".to_string()));
        
        // Test newlines and whitespace
        translations.korean = Some("Text with\nnewlines\tand\ttabs   and   spaces".to_string());
        let whitespace_korean: Option<String> = translations.korean.clone();
        assert_eq!(whitespace_korean, Some("Text with\nnewlines\tand\ttabs   and   spaces".to_string()));
    }

    /// Test trait implementations and their return types
    #[test]
    fn test_translations_trait_implementations() {
        let mut translations1 = MtgjsonTranslations::new(None);
        let mut translations2 = MtgjsonTranslations::new(None);
        
        translations1.french = Some("Français".to_string());
        translations1.german = Some("Deutsch".to_string());
        translations1.spanish = Some("Español".to_string());
        
        translations2.french = Some("Français".to_string());
        translations2.german = Some("Deutsch".to_string());
        translations2.spanish = Some("Español".to_string());
        
        // Test Clone trait
        let cloned_translations: MtgjsonTranslations = translations1.clone();
        assert_eq!(cloned_translations.french, translations1.french);
        assert_eq!(cloned_translations.german, translations1.german);
        assert_eq!(cloned_translations.spanish, translations1.spanish);
        
        // Test PartialEq trait
        let equality_result: bool = translations1 == translations2;
        assert_eq!(equality_result, true);
        
        translations2.italian = Some("Italiano".to_string());
        let inequality_result: bool = translations1 != translations2;
        assert_eq!(inequality_result, true);
        
        // Test Debug trait
        let debug_string: String = format!("{:?}", translations1);
        assert!(!debug_string.is_empty());
        assert!(debug_string.contains("MtgjsonTranslations"));
        
        // Test Default trait
        let default_translations: MtgjsonTranslations = MtgjsonTranslations::default();
        assert_eq!(default_translations.french, None);
        assert_eq!(default_translations.german, None);
        assert_eq!(default_translations.spanish, None);
    }

    /// Test JSON serialization/deserialization return types
    #[test]
    fn test_translations_json_operations_return_types() {
        let mut translations = MtgjsonTranslations::new(None);
        translations.chinese_simplified = Some("简体中文".to_string());
        translations.french = Some("Français".to_string());
        translations.german = Some("Deutsch".to_string());
        translations.japanese = Some("日本語".to_string());
        translations.spanish = Some("Español".to_string());
        
        // Test Serialize trait
        let serialize_result: Result<String, serde_json::Error> = serde_json::to_string(&translations);
        assert!(serialize_result.is_ok());
        let serialized_json: String = serialize_result.unwrap();
        assert!(!serialized_json.is_empty());
        assert!(serialized_json.contains("简体中文"));
        assert!(serialized_json.contains("Français"));
        assert!(serialized_json.contains("Deutsch"));
        assert!(serialized_json.contains("日本語"));
        assert!(serialized_json.contains("Español"));
        
        // Test Deserialize trait
        let deserialize_result: Result<MtgjsonTranslations, serde_json::Error> = 
            serde_json::from_str(&serialized_json);
        assert!(deserialize_result.is_ok());
        let deserialized_translations: MtgjsonTranslations = deserialize_result.unwrap();
        
        assert_eq!(deserialized_translations.chinese_simplified, Some("简体中文".to_string()));
        assert_eq!(deserialized_translations.french, Some("Français".to_string()));
        assert_eq!(deserialized_translations.german, Some("Deutsch".to_string()));
        assert_eq!(deserialized_translations.japanese, Some("日本語".to_string()));
        assert_eq!(deserialized_translations.spanish, Some("Español".to_string()));
        
        // Test to_json method
        let to_json_result: Result<String, pyo3::PyErr> = translations.to_json();
        assert!(to_json_result.is_ok());
        let to_json_string: String = to_json_result.unwrap();
        assert!(!to_json_string.is_empty());
    }

    /// Test comprehensive real-world examples with all field types
    #[test]
    fn test_translations_comprehensive_examples() {
        // Test Magic card name translations
        let mut card_translations = HashMap::new();
        card_translations.insert("French".to_string(), "Éclair".to_string()); // Lightning Bolt
        card_translations.insert("German".to_string(), "Blitzschlag".to_string());
        card_translations.insert("Italian".to_string(), "Dardo Fulminante".to_string());
        card_translations.insert("Spanish".to_string(), "Rayo".to_string());
        card_translations.insert("Japanese".to_string(), "稲妻".to_string());
        card_translations.insert("Korean".to_string(), "번개 화살".to_string());
        card_translations.insert("Chinese Simplified".to_string(), "闪电箭".to_string());
        card_translations.insert("Portuguese (Brazil)".to_string(), "Raio".to_string());
        card_translations.insert("Russian".to_string(), "Молния".to_string());
        
        let card_name_translations = MtgjsonTranslations::new(Some(card_translations));
        
        // Verify all translations are properly set
        assert_eq!(card_name_translations.french, Some("Éclair".to_string()));
        assert_eq!(card_name_translations.german, Some("Blitzschlag".to_string()));
        assert_eq!(card_name_translations.italian, Some("Dardo Fulminante".to_string()));
        assert_eq!(card_name_translations.spanish, Some("Rayo".to_string()));
        assert_eq!(card_name_translations.japanese, Some("稲妻".to_string()));
        assert_eq!(card_name_translations.korean, Some("번개 화살".to_string()));
        assert_eq!(card_name_translations.chinese_simplified, Some("闪电箭".to_string()));
        assert_eq!(card_name_translations.portuguese_brazil, Some("Raio".to_string()));
        assert_eq!(card_name_translations.russian, Some("Молния".to_string()));
        
        // Test has_translations
        let has_translations: bool = card_name_translations.has_translations();
        assert_eq!(has_translations, true);
        
        // Test get_available_languages
        let available_languages: Vec<String> = card_name_translations.get_available_languages();
        assert_eq!(available_languages.len(), 9);
        
        // Test to_dict
        let dict_result: Result<HashMap<String, String>, pyo3::PyErr> = card_name_translations.to_dict();
        assert!(dict_result.is_ok());
        let dict: HashMap<String, String> = dict_result.unwrap();
        assert_eq!(dict.len(), 9);
        
        // Test set name translations with longer text
        let mut set_translations = HashMap::new();
        set_translations.insert("French".to_string(), "Les Rues de la Nouvelle Capenna".to_string());
        set_translations.insert("German".to_string(), "Straßen von Neu-Capenna".to_string());
        set_translations.insert("Japanese".to_string(), "ニューカペナの街角".to_string());
        set_translations.insert("Chinese Simplified".to_string(), "新卡佩纳街头".to_string());
        
        let set_name_translations = MtgjsonTranslations::new(Some(set_translations));
        
        // Verify longer text translations
        assert_eq!(set_name_translations.french, Some("Les Rues de la Nouvelle Capenna".to_string()));
        assert_eq!(set_name_translations.german, Some("Straßen von Neu-Capenna".to_string()));
        assert_eq!(set_name_translations.japanese, Some("ニューカペナの街角".to_string()));
        assert_eq!(set_name_translations.chinese_simplified, Some("新卡佩纳街头".to_string()));
        
        // Test JSON serialization of comprehensive examples
        let json_result: Result<String, pyo3::PyErr> = set_name_translations.to_json();
        assert!(json_result.is_ok());
        let json_string: String = json_result.unwrap();
        assert!(json_string.contains("Les Rues de la Nouvelle Capenna"));
        assert!(json_string.contains("ニューカペナの街角"));
    }

    /// Test Unicode and special character handling return types
    #[test]
    fn test_translations_unicode_handling_return_types() {
        let mut translations = MtgjsonTranslations::new(None);
        
        // Test various Unicode ranges
        translations.chinese_simplified = Some("测试简体中文：一二三四五六七八九十".to_string());
        translations.chinese_traditional = Some("測試繁體中文：壹貳參肆伍陸柒捌玖拾".to_string());
        translations.japanese = Some("テスト：ひらがな、カタカナ、漢字 🎌".to_string());
        translations.korean = Some("테스트：한글 문자 🇰🇷".to_string());
        translations.russian = Some("Тест：кириллические символы 🇷🇺".to_string());
        translations.french = Some("Test：caractères français avec émojis 🇫🇷".to_string());
        translations.german = Some("Test：deutsche Zeichen mit Emojis 🇩🇪".to_string());
        translations.spanish = Some("Prueba：caracteres españoles con emojis 🇪🇸".to_string());
        translations.portuguese_brazil = Some("Teste：caracteres portugueses com emojis 🇧🇷".to_string());
        translations.italian = Some("Test：caratteri italiani con emoji 🇮🇹".to_string());
        
        // Test that all Unicode content is preserved
        let chinese_simplified: Option<String> = translations.chinese_simplified.clone();
        let chinese_traditional: Option<String> = translations.chinese_traditional.clone();
        let japanese: Option<String> = translations.japanese.clone();
        let korean: Option<String> = translations.korean.clone();
        let russian: Option<String> = translations.russian.clone();
        let french: Option<String> = translations.french.clone();
        let german: Option<String> = translations.german.clone();
        let spanish: Option<String> = translations.spanish.clone();
        let portuguese_brazil: Option<String> = translations.portuguese_brazil.clone();
        let italian: Option<String> = translations.italian.clone();
        
        assert!(chinese_simplified.unwrap().contains("一二三四五六七八九十"));
        assert!(chinese_traditional.unwrap().contains("壹貳參肆伍陸柒捌玖拾"));
        assert!(japanese.unwrap().contains("🎌"));
        assert!(korean.unwrap().contains("🇰🇷"));
        assert!(russian.unwrap().contains("🇷🇺"));
        assert!(french.unwrap().contains("🇫🇷"));
        assert!(german.unwrap().contains("🇩🇪"));
        assert!(spanish.unwrap().contains("🇪🇸"));
        assert!(portuguese_brazil.unwrap().contains("🇧🇷"));
        assert!(italian.unwrap().contains("🇮🇹"));
        
        // Test JSON serialization preserves Unicode
        let json_result: Result<String, pyo3::PyErr> = translations.to_json();
        assert!(json_result.is_ok());
        let json_string: String = json_result.unwrap();
        assert!(json_string.contains("一二三四五六七八九十"));
        assert!(json_string.contains("🎌"));
        assert!(json_string.contains("🇰🇷"));
    }

    /// Test JsonObject trait implementation return types
    #[test]
    fn test_translations_json_object_trait_return_types() {
        let translations = MtgjsonTranslations::new(None);
        
        // Test build_keys_to_skip method return type from JsonObject trait
        let keys_to_skip: std::collections::HashSet<String> = translations.build_keys_to_skip();
        
        // Test that it's a proper HashSet<String>
        let is_hashset: bool = keys_to_skip.is_empty() || !keys_to_skip.is_empty();
        assert_eq!(is_hashset, true); // Always true for HashSet
        
        // Default JsonObject trait implementation should return empty set
        assert!(keys_to_skip.is_empty());
    }
}