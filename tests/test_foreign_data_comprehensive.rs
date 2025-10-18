use mtgjson_rust::classes::*;

mod comprehensive_foreign_data_tests {
    use super::*;
    use mtgjson_rust::classes::foreign_data::MtgjsonForeignDataObject;

    /// Test all MtgjsonForeignDataObject constructors and return types
    #[test]
    fn test_foreign_data_constructors_return_types() {
        // Test default constructor
        let default_foreign_data = MtgjsonForeignDataObject::new();
        assert!(default_foreign_data.face_name.is_none());
        assert!(default_foreign_data.flavor_text.is_none());
        assert!(default_foreign_data.language.is_empty());
        assert!(default_foreign_data.multiverse_id.is_none());
        assert!(default_foreign_data.name.is_empty());
        assert!(default_foreign_data.text.is_none());
        assert!(default_foreign_data.type_.is_none());
        
        // Verify return types
        let face_name: Option<String> = default_foreign_data.face_name.clone();
        let flavor_text: Option<String> = default_foreign_data.flavor_text.clone();
        let language: String = default_foreign_data.language.clone();
        let multiverse_id: Option<String> = default_foreign_data.multiverse_id.clone();
        let name: String = default_foreign_data.name.clone();
        let text: Option<String> = default_foreign_data.text.clone();
        let type_: Option<String> = default_foreign_data.type_.clone();
        
        assert!(face_name.is_none());
        assert!(flavor_text.is_none());
        assert!(language.is_empty());
        assert!(multiverse_id.is_none());
        assert!(name.is_empty());
        assert!(text.is_none());
        assert!(type_.is_none());
    }

    /// Test all field assignments and return types
    #[test]
    fn test_foreign_data_field_assignment_return_types() {
        let mut foreign_data = MtgjsonForeignDataObject::new();
        
        // Test all field assignments
        foreign_data.face_name = Some("Nom de Face".to_string());
        foreign_data.flavor_text = Some("Texte de saveur français".to_string());
        foreign_data.language = "French".to_string();
        foreign_data.multiverse_id = Some("12345".to_string());
        foreign_data.name = "Nom Français".to_string();
        foreign_data.text = Some("Texte de règles français".to_string());
        foreign_data.type_ = Some("Créature — Humain Sorcier".to_string());
        
        // Verify return types
        let face_name: Option<String> = foreign_data.face_name.clone();
        let flavor_text: Option<String> = foreign_data.flavor_text.clone();
        let language: String = foreign_data.language.clone();
        let multiverse_id: Option<String> = foreign_data.multiverse_id.clone();
        let name: String = foreign_data.name.clone();
        let text: Option<String> = foreign_data.text.clone();
        let type_: Option<String> = foreign_data.type_.clone();
        
        assert_eq!(face_name.unwrap(), "Nom de Face");
        assert_eq!(flavor_text.unwrap(), "Texte de saveur français");
        assert_eq!(language, "French");
        assert_eq!(multiverse_id.unwrap(), "12345");
        assert_eq!(name, "Nom Français");
        assert_eq!(text.unwrap(), "Texte de règles français");
        assert_eq!(type_.unwrap(), "Créature — Humain Sorcier");
    }

    /// Test JSON serialization methods and return types
    #[test]
    fn test_foreign_data_json_methods_return_types() {
        let mut foreign_data = MtgjsonForeignDataObject::new();
        foreign_data.face_name = Some("Cara del Rostro".to_string());
        foreign_data.flavor_text = Some("Texto de sabor español".to_string());
        foreign_data.language = "Spanish".to_string();
        foreign_data.multiverse_id = Some("67890".to_string());
        foreign_data.name = "Nombre Español".to_string();
        foreign_data.text = Some("Texto de reglas español".to_string());
        foreign_data.type_ = Some("Criatura — Humano Hechicero".to_string());
        
        // Test to_json method return type
        let json_result: Result<String, pyo3::PyErr> = foreign_data.to_json();
        assert!(json_result.is_ok());
        let json_string: String = json_result.unwrap();
        
        assert!(!json_string.is_empty());
        assert!(json_string.contains("Cara del Rostro"));
        assert!(json_string.contains("Texto de sabor español"));
        assert!(json_string.contains("Spanish"));
        assert!(json_string.contains("67890"));
        assert!(json_string.contains("Nombre Español"));
        assert!(json_string.contains("Texto de reglas español"));
        assert!(json_string.contains("Criatura — Humano Hechicero"));
    }

    /// Test edge cases and special characters return types
    #[test]
    fn test_foreign_data_edge_cases_return_types() {
        // Test with unicode and special characters
        let mut unicode_foreign_data = MtgjsonForeignDataObject::new();
        unicode_foreign_data.face_name = Some("顔の名前".to_string());
        unicode_foreign_data.flavor_text = Some("これは日本語のフレーバーテキストです🎌".to_string());
        unicode_foreign_data.language = "Japanese".to_string();
        unicode_foreign_data.multiverse_id = Some("999999".to_string());
        unicode_foreign_data.name = "日本語の名前".to_string();
        unicode_foreign_data.text = Some("{T}: カードを1枚引く。".to_string());
        unicode_foreign_data.type_ = Some("クリーチャー — 人間・ウィザード".to_string());
        
        // Verify unicode handling
        let unicode_name: String = unicode_foreign_data.name.clone();
        let unicode_text: Option<String> = unicode_foreign_data.text.clone();
        let unicode_flavor: Option<String> = unicode_foreign_data.flavor_text.clone();
        let unicode_type: Option<String> = unicode_foreign_data.type_.clone();
        
        assert_eq!(unicode_name, "日本語の名前");
        assert_eq!(unicode_text.unwrap(), "{T}: カードを1枚引く。");
        assert!(unicode_flavor.unwrap().contains("🎌"));
        assert!(unicode_type.unwrap().contains("ウィザード"));
        
        // Test with empty strings
        let mut empty_foreign_data = MtgjsonForeignDataObject::new();
        empty_foreign_data.face_name = Some("".to_string());
        empty_foreign_data.flavor_text = Some("".to_string());
        empty_foreign_data.language = "".to_string();
        empty_foreign_data.multiverse_id = Some("".to_string());
        empty_foreign_data.name = "".to_string();
        empty_foreign_data.text = Some("".to_string());
        empty_foreign_data.type_ = Some("".to_string());
        
        let empty_name: String = empty_foreign_data.name.clone();
        let empty_language: String = empty_foreign_data.language.clone();
        let empty_face_name: Option<String> = empty_foreign_data.face_name.clone();
        let empty_text: Option<String> = empty_foreign_data.text.clone();
        
        assert_eq!(empty_name, "");
        assert_eq!(empty_language, "");
        assert_eq!(empty_face_name.unwrap(), "");
        assert_eq!(empty_text.unwrap(), "");
        
        // Test with None values
        let none_foreign_data = MtgjsonForeignDataObject::new();
        let none_face_name: Option<String> = none_foreign_data.face_name.clone();
        let none_flavor_text: Option<String> = none_foreign_data.flavor_text.clone();
        let none_multiverse_id: Option<String> = none_foreign_data.multiverse_id.clone();
        let none_text: Option<String> = none_foreign_data.text.clone();
        let none_type: Option<String> = none_foreign_data.type_.clone();
        
        assert!(none_face_name.is_none());
        assert!(none_flavor_text.is_none());
        assert!(none_multiverse_id.is_none());
        assert!(none_text.is_none());
        assert!(none_type.is_none());
    }

    /// Test comprehensive trait implementations
    #[test]
    fn test_foreign_data_trait_implementations() {
        let mut foreign_data1 = MtgjsonForeignDataObject::new();
        foreign_data1.face_name = Some("German Face".to_string());
        foreign_data1.flavor_text = Some("Deutsche Geschmackstext".to_string());
        foreign_data1.language = "German".to_string();
        foreign_data1.multiverse_id = Some("555555".to_string());
        foreign_data1.name = "Deutscher Name".to_string();
        foreign_data1.text = Some("Deutsche Regeltext".to_string());
        foreign_data1.type_ = Some("Kreatur — Mensch Zauberer".to_string());
        
        let mut foreign_data2 = MtgjsonForeignDataObject::new();
        foreign_data2.face_name = Some("Italian Face".to_string());
        foreign_data2.flavor_text = Some("Testo di sapore italiano".to_string());
        foreign_data2.language = "Italian".to_string();
        foreign_data2.multiverse_id = Some("777777".to_string());
        foreign_data2.name = "Nome Italiano".to_string();
        foreign_data2.text = Some("Testo delle regole italiane".to_string());
        foreign_data2.type_ = Some("Creatura — Umano Mago".to_string());
        
        // Test Clone trait
        let cloned_foreign_data1 = foreign_data1.clone();
        assert_eq!(foreign_data1.face_name, cloned_foreign_data1.face_name);
        assert_eq!(foreign_data1.flavor_text, cloned_foreign_data1.flavor_text);
        assert_eq!(foreign_data1.language, cloned_foreign_data1.language);
        assert_eq!(foreign_data1.multiverse_id, cloned_foreign_data1.multiverse_id);
        assert_eq!(foreign_data1.name, cloned_foreign_data1.name);
        assert_eq!(foreign_data1.text, cloned_foreign_data1.text);
        assert_eq!(foreign_data1.type_, cloned_foreign_data1.type_);
        
        // Test PartialEq trait
        assert_eq!(foreign_data1, cloned_foreign_data1);
        assert_ne!(foreign_data1, foreign_data2);
        
        // Test Default trait
        let default_foreign_data = MtgjsonForeignDataObject::default();
        assert!(default_foreign_data.face_name.is_none());
        assert!(default_foreign_data.name.is_empty());
        assert!(default_foreign_data.language.is_empty());
        
        // Test Debug trait
        let debug_output = format!("{:?}", foreign_data1);
        assert!(debug_output.contains("MtgjsonForeignDataObject"));
        assert!(debug_output.contains("German"));
        assert!(debug_output.contains("Deutscher Name"));
        
        // Test equality with different combinations
        let mut foreign_data3 = foreign_data1.clone();
        foreign_data3.name = "Different Name".to_string();
        assert_ne!(foreign_data1, foreign_data3);
        
        let mut foreign_data4 = foreign_data1.clone();
        foreign_data4.language = "Different Language".to_string();
        assert_ne!(foreign_data1, foreign_data4);
    }

    /// Test various language scenarios
    #[test]
    fn test_foreign_data_various_languages_return_types() {
        // Test Chinese (Simplified)
        let mut chinese_foreign_data = MtgjsonForeignDataObject::new();
        chinese_foreign_data.language = "Chinese Simplified".to_string();
        chinese_foreign_data.name = "闪电箭".to_string();
        chinese_foreign_data.text = Some("闪电箭对任意一个目标造成3点伤害。".to_string());
        chinese_foreign_data.type_ = Some("瞬间".to_string());
        
        // Test Russian
        let mut russian_foreign_data = MtgjsonForeignDataObject::new();
        russian_foreign_data.language = "Russian".to_string();
        russian_foreign_data.name = "Молния".to_string();
        russian_foreign_data.text = Some("Молния наносит 3 повреждения любой цели.".to_string());
        russian_foreign_data.type_ = Some("Мгновенное заклинание".to_string());
        
        // Test Korean
        let mut korean_foreign_data = MtgjsonForeignDataObject::new();
        korean_foreign_data.language = "Korean".to_string();
        korean_foreign_data.name = "번개 화살".to_string();
        korean_foreign_data.text = Some("원하는 목표 하나에게 피해 3점을 입힌다.".to_string());
        korean_foreign_data.type_ = Some("순간마법".to_string());
        
        // Test Portuguese
        let mut portuguese_foreign_data = MtgjsonForeignDataObject::new();
        portuguese_foreign_data.language = "Portuguese (Brazil)".to_string();
        portuguese_foreign_data.name = "Raio".to_string();
        portuguese_foreign_data.text = Some("Raio causa 3 pontos de dano a qualquer alvo.".to_string());
        portuguese_foreign_data.type_ = Some("Mágica Instantânea".to_string());
        
        // Verify all languages work correctly
        let chinese_name: String = chinese_foreign_data.name.clone();
        let russian_name: String = russian_foreign_data.name.clone();
        let korean_name: String = korean_foreign_data.name.clone();
        let portuguese_name: String = portuguese_foreign_data.name.clone();
        
        assert_eq!(chinese_name, "闪电箭");
        assert_eq!(russian_name, "Молния");
        assert_eq!(korean_name, "번개 화살");
        assert_eq!(portuguese_name, "Raio");
        
        // Test JSON serialization for all languages
        let chinese_json: Result<String, pyo3::PyErr> = chinese_foreign_data.to_json();
        let russian_json: Result<String, pyo3::PyErr> = russian_foreign_data.to_json();
        let korean_json: Result<String, pyo3::PyErr> = korean_foreign_data.to_json();
        let portuguese_json: Result<String, pyo3::PyErr> = portuguese_foreign_data.to_json();
        
        assert!(chinese_json.is_ok());
        assert!(russian_json.is_ok());
        assert!(korean_json.is_ok());
        assert!(portuguese_json.is_ok());
        
        assert!(chinese_json.unwrap().contains("闪电箭"));
        assert!(russian_json.unwrap().contains("Молния"));
        assert!(korean_json.unwrap().contains("번개 화살"));
        assert!(portuguese_json.unwrap().contains("Raio"));
    }

    /// Test complex integration scenarios
    #[test]
    fn test_foreign_data_complex_integration_return_types() {
        // Create multiple foreign data entries for the same card
        let mut english_data = MtgjsonForeignDataObject::new();
        english_data.language = "English".to_string();
        english_data.name = "Lightning Bolt".to_string();
        english_data.text = Some("Lightning Bolt deals 3 damage to any target.".to_string());
        english_data.type_ = Some("Instant".to_string());
        english_data.flavor_text = Some("The spark of inspiration.".to_string());
        english_data.multiverse_id = Some("1001".to_string());
        
        let mut french_data = MtgjsonForeignDataObject::new();
        french_data.language = "French".to_string();
        french_data.name = "Éclair".to_string();
        french_data.text = Some("L'Éclair inflige 3 blessures à n'importe quelle cible.".to_string());
        french_data.type_ = Some("Éphémère".to_string());
        french_data.flavor_text = Some("L'étincelle de l'inspiration.".to_string());
        french_data.multiverse_id = Some("1002".to_string());
        
        let mut german_data = MtgjsonForeignDataObject::new();
        german_data.language = "German".to_string();
        german_data.name = "Blitzschlag".to_string();
        german_data.text = Some("Blitzschlag fügt einem Ziel deiner Wahl 3 Schadenspunkte zu.".to_string());
        german_data.type_ = Some("Spontanzauber".to_string());
        german_data.flavor_text = Some("Der Funke der Inspiration.".to_string());
        german_data.multiverse_id = Some("1003".to_string());
        
        // Test all three together
        let foreign_data_list = vec![english_data, french_data, german_data];
        
        for (index, foreign_data) in foreign_data_list.iter().enumerate() {
            // Verify each has correct data
            let language: String = foreign_data.language.clone();
            let name: String = foreign_data.name.clone();
            let text: Option<String> = foreign_data.text.clone();
            let type_: Option<String> = foreign_data.type_.clone();
            let flavor_text: Option<String> = foreign_data.flavor_text.clone();
            let multiverse_id: Option<String> = foreign_data.multiverse_id.clone();
            
            match index {
                0 => {
                    assert_eq!(language, "English");
                    assert_eq!(name, "Lightning Bolt");
                    assert!(text.unwrap().contains("Lightning Bolt deals"));
                    assert_eq!(type_.unwrap(), "Instant");
                    assert!(flavor_text.unwrap().contains("inspiration"));
                    assert_eq!(multiverse_id.unwrap(), "1001");
                },
                1 => {
                    assert_eq!(language, "French");
                    assert_eq!(name, "Éclair");
                    assert!(text.unwrap().contains("L'Éclair inflige"));
                    assert_eq!(type_.unwrap(), "Éphémère");
                    assert!(flavor_text.unwrap().contains("inspiration"));
                    assert_eq!(multiverse_id.unwrap(), "1002");
                },
                2 => {
                    assert_eq!(language, "German");
                    assert_eq!(name, "Blitzschlag");
                    assert!(text.unwrap().contains("Blitzschlag fügt"));
                    assert_eq!(type_.unwrap(), "Spontanzauber");
                    assert!(flavor_text.unwrap().contains("Inspiration"));
                    assert_eq!(multiverse_id.unwrap(), "1003");
                },
                _ => panic!("Unexpected index"),
            }
            
            // Test JSON serialization for each
            let json_result: Result<String, pyo3::PyErr> = foreign_data.to_json();
            assert!(json_result.is_ok());
            let json_string: String = json_result.unwrap();
            assert!(json_string.contains(&name));
            assert!(json_string.contains(&language));
        }
        
        // Test cloning and equality
        let english_clone = foreign_data_list[0].clone();
        assert_eq!(foreign_data_list[0], english_clone);
        assert_ne!(foreign_data_list[0], foreign_data_list[1]);
        assert_ne!(foreign_data_list[1], foreign_data_list[2]);
        
        // Test modification doesn't affect originals
        let mut modified_english = foreign_data_list[0].clone();
        modified_english.name = "Modified Name".to_string();
        assert_ne!(foreign_data_list[0], modified_english);
        assert_eq!(foreign_data_list[0], english_clone);
    }

    /// Test JSON object trait implementation
    #[test]
    fn test_foreign_data_json_object_trait_return_types() {
        let mut foreign_data = MtgjsonForeignDataObject::new();
        foreign_data.language = "Italian".to_string();
        foreign_data.name = "Nome Italiano".to_string();
        foreign_data.text = Some("Testo italiano".to_string());
        
        // Test JsonObject trait methods
        let keys_to_skip = foreign_data.build_keys_to_skip();
        let keys_to_skip_type: std::collections::HashSet<String> = keys_to_skip;
        assert!(keys_to_skip_type.is_empty()); // Foreign data doesn't skip keys by default
    }

    /// Test special formatting and symbols in text
    #[test]
    fn test_foreign_data_special_formatting_return_types() {
        let mut symbol_foreign_data = MtgjsonForeignDataObject::new();
        symbol_foreign_data.language = "English".to_string();
        symbol_foreign_data.name = "Mana Test".to_string();
        symbol_foreign_data.text = Some("{T}: Add {C}. {1}{R}: Deal 1 damage.".to_string());
        symbol_foreign_data.type_ = Some("Artifact — Equipment".to_string());
        symbol_foreign_data.flavor_text = Some("\"Power flows through the ancient runes.\" —Urza".to_string());
        
        // Test that symbols and special formatting are preserved
        let text_with_symbols: Option<String> = symbol_foreign_data.text.clone();
        let flavor_with_quotes: Option<String> = symbol_foreign_data.flavor_text.clone();
        
        assert!(text_with_symbols.unwrap().contains("{T}"));
        assert!(text_with_symbols.as_ref().unwrap().contains("{C}"));
        assert!(text_with_symbols.as_ref().unwrap().contains("{1}{R}"));
        assert!(flavor_with_quotes.unwrap().contains("\""));
        assert!(flavor_with_quotes.as_ref().unwrap().contains("—"));
        
        // Test JSON serialization preserves symbols
        let json_result: Result<String, pyo3::PyErr> = symbol_foreign_data.to_json();
        assert!(json_result.is_ok());
        let json_string: String = json_result.unwrap();
        assert!(json_string.contains("{T}"));
        assert!(json_string.contains("{C}"));
        assert!(json_string.contains("—Urza"));
    }
}