# Details

Date : 2025-10-07 10:30:22

Directory c:\\Users\\rprat\\projects\\mtgjson-v5\\mtgjson-rust\\src

Total : 60 files,  13895 codes, 1003 comments, 2458 blanks, all 17356 lines

[Summary](results.md) / Details / [Diff Summary](diff.md) / [Diff Details](diff-details.md)

## Files
| filename | language | code | comment | blank | total |
| :--- | :--- | ---: | ---: | ---: | ---: |
| [src/builders/mod.rs](/src/builders/mod.rs) | Rust | 22 | 2 | 5 | 29 |
| [src/builders/output\_generator.rs](/src/builders/output_generator.rs) | Rust | 718 | 53 | 154 | 925 |
| [src/builders/parallel\_call.rs](/src/builders/parallel_call.rs) | Rust | 498 | 61 | 66 | 625 |
| [src/builders/price\_builder.rs](/src/builders/price_builder.rs) | Rust | 118 | 33 | 19 | 170 |
| [src/builders/set\_builder.rs](/src/builders/set_builder.rs) | Rust | 1,249 | 152 | 252 | 1,653 |
| [src/builders/set\_builder\_functions.rs](/src/builders/set_builder_functions.rs) | Rust | 178 | 31 | 27 | 236 |
| [src/classes/base.rs](/src/classes/base.rs) | Rust | 70 | 10 | 13 | 93 |
| [src/classes/card.rs](/src/classes/card.rs) | Rust | 1,115 | 54 | 231 | 1,400 |
| [src/classes/deck.rs](/src/classes/deck.rs) | Rust | 882 | 53 | 193 | 1,128 |
| [src/classes/foreign\_data.rs](/src/classes/foreign_data.rs) | Rust | 353 | 15 | 71 | 439 |
| [src/classes/game\_formats.rs](/src/classes/game_formats.rs) | Rust | 69 | 3 | 15 | 87 |
| [src/classes/identifiers.rs](/src/classes/identifiers.rs) | Rust | 794 | 33 | 113 | 940 |
| [src/classes/leadership\_skills.rs](/src/classes/leadership_skills.rs) | Rust | 46 | 4 | 11 | 61 |
| [src/classes/legalities.rs](/src/classes/legalities.rs) | Rust | 103 | 6 | 24 | 133 |
| [src/classes/meta.rs](/src/classes/meta.rs) | Rust | 46 | 7 | 11 | 64 |
| [src/classes/mod.rs](/src/classes/mod.rs) | Rust | 34 | 0 | 2 | 36 |
| [src/classes/prices.rs](/src/classes/prices.rs) | Rust | 814 | 25 | 110 | 949 |
| [src/classes/purchase\_urls.rs](/src/classes/purchase_urls.rs) | Rust | 119 | 5 | 17 | 141 |
| [src/classes/related\_cards.rs](/src/classes/related_cards.rs) | Rust | 77 | 9 | 15 | 101 |
| [src/classes/rulings.rs](/src/classes/rulings.rs) | Rust | 41 | 5 | 9 | 55 |
| [src/classes/sealed\_product.rs](/src/classes/sealed_product.rs) | Rust | 669 | 45 | 93 | 807 |
| [src/classes/set.rs](/src/classes/set.rs) | Rust | 919 | 58 | 236 | 1,213 |
| [src/classes/translations.rs](/src/classes/translations.rs) | Rust | 181 | 6 | 24 | 211 |
| [src/classes/utils.rs](/src/classes/utils.rs) | Rust | 66 | 6 | 14 | 86 |
| [src/compiled\_classes.rs](/src/compiled_classes.rs) | Rust | 22 | 2 | 2 | 26 |
| [src/compiled\_classes/all\_identifiers.rs](/src/compiled_classes/all_identifiers.rs) | Rust | 240 | 21 | 45 | 306 |
| [src/compiled\_classes/all\_printings.rs](/src/compiled_classes/all_printings.rs) | Rust | 169 | 25 | 34 | 228 |
| [src/compiled\_classes/atomic\_cards.rs](/src/compiled_classes/atomic_cards.rs) | Rust | 26 | 1 | 5 | 32 |
| [src/compiled\_classes/card\_types.rs](/src/compiled_classes/card_types.rs) | Rust | 25 | 1 | 5 | 31 |
| [src/compiled\_classes/compiled\_list.rs](/src/compiled_classes/compiled_list.rs) | Rust | 93 | 9 | 20 | 122 |
| [src/compiled\_classes/deck\_list.rs](/src/compiled_classes/deck_list.rs) | Rust | 167 | 17 | 31 | 215 |
| [src/compiled\_classes/enum\_values.rs](/src/compiled_classes/enum_values.rs) | Rust | 25 | 1 | 5 | 31 |
| [src/compiled\_classes/keywords.rs](/src/compiled_classes/keywords.rs) | Rust | 273 | 16 | 31 | 320 |
| [src/compiled\_classes/set\_list.rs](/src/compiled_classes/set_list.rs) | Rust | 25 | 1 | 5 | 31 |
| [src/compiled\_classes/structures.rs](/src/compiled_classes/structures.rs) | Rust | 152 | 10 | 12 | 174 |
| [src/compiled\_classes/tcgplayer\_skus.rs](/src/compiled_classes/tcgplayer_skus.rs) | Rust | 26 | 1 | 5 | 32 |
| [src/constants.rs](/src/constants.rs) | Rust | 165 | 1 | 3 | 169 |
| [src/lib.rs](/src/lib.rs) | Rust | 134 | 19 | 18 | 171 |
| [src/providers/cardmarket/mod.rs](/src/providers/cardmarket/mod.rs) | Rust | 2 | 0 | 2 | 4 |
| [src/providers/cardmarket/monolith.rs](/src/providers/cardmarket/monolith.rs) | Rust | 545 | 47 | 86 | 678 |
| [src/providers/edhrec/card\_ranks.rs](/src/providers/edhrec/card_ranks.rs) | Rust | 69 | 0 | 10 | 79 |
| [src/providers/edhrec/mod.rs](/src/providers/edhrec/mod.rs) | Rust | 2 | 0 | 2 | 4 |
| [src/providers/mod.rs](/src/providers/mod.rs) | Rust | 81 | 10 | 11 | 102 |
| [src/providers/mtgwiki/mod.rs](/src/providers/mtgwiki/mod.rs) | Rust | 2 | 0 | 2 | 4 |
| [src/providers/mtgwiki/secret\_lair.rs](/src/providers/mtgwiki/secret_lair.rs) | Rust | 68 | 2 | 11 | 81 |
| [src/providers/provider\_base.rs](/src/providers/provider_base.rs) | Rust | 166 | 19 | 31 | 216 |
| [src/providers/scryfall/mod.rs](/src/providers/scryfall/mod.rs) | Rust | 6 | 0 | 2 | 8 |
| [src/providers/scryfall/monolith.rs](/src/providers/scryfall/monolith.rs) | Rust | 402 | 23 | 61 | 486 |
| [src/providers/scryfall/orientation\_detector.rs](/src/providers/scryfall/orientation_detector.rs) | Rust | 130 | 3 | 23 | 156 |
| [src/providers/scryfall/sf\_utils.rs](/src/providers/scryfall/sf_utils.rs) | Rust | 104 | 11 | 21 | 136 |
| [src/providers/third\_party/cardhoarder.rs](/src/providers/third_party/cardhoarder.rs) | Rust | 146 | 7 | 19 | 172 |
| [src/providers/third\_party/cardkingdom.rs](/src/providers/third_party/cardkingdom.rs) | Rust | 293 | 14 | 37 | 344 |
| [src/providers/third\_party/gatherer.rs](/src/providers/third_party/gatherer.rs) | Rust | 195 | 14 | 32 | 241 |
| [src/providers/third\_party/mod.rs](/src/providers/third_party/mod.rs) | Rust | 16 | 2 | 2 | 20 |
| [src/providers/third\_party/mtgban.rs](/src/providers/third_party/mtgban.rs) | Rust | 69 | 0 | 10 | 79 |
| [src/providers/third\_party/multiverse\_bridge.rs](/src/providers/third_party/multiverse_bridge.rs) | Rust | 68 | 2 | 11 | 81 |
| [src/providers/third\_party/tcgplayer.rs](/src/providers/third_party/tcgplayer.rs) | Rust | 217 | 6 | 32 | 255 |
| [src/providers/third\_party/whats\_in\_standard.rs](/src/providers/third_party/whats_in_standard.rs) | Rust | 400 | 24 | 71 | 495 |
| [src/providers/third\_party/wizards.rs](/src/providers/third_party/wizards.rs) | Rust | 128 | 8 | 22 | 158 |
| [src/utils\_functions.rs](/src/utils_functions.rs) | Rust | 63 | 10 | 14 | 87 |

[Summary](results.md) / Details / [Diff Summary](diff.md) / [Diff Details](diff-details.md)