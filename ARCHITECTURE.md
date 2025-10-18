# MTGJSON Rust Architecture

## High-Level System Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                            Python Layer (PyO3 FFI)                          │
│  ┌───────────────────────────────────────────────────────────────────────┐  │
│  │  Python Module: mtgjson_rust                                          │  │
│  │  - Classes, Providers, Builders exposed via PyO3                      │  │
│  └─────────────────────────────────┬─────────────────────────────────────┘  │
└────────────────────────────────────┼────────────────────────────────────────┘
                                     │
                    ┌────────────────┴────────────────┐
                    │   Rust Core Library (cdylib)    │
                    │   mtgjson_rust crate            │
                    └────────────────┬────────────────┘
                                     │
        ┌────────────────────────────┼───────────────────────────┐
        │                            │                           │
┌───────▼────────┐         ┌─────────▼────────┐      ┌──────────▼─────────┐
│    Classes     │         │    Providers     │      │      Builders      │
│   (Data Types) │         │ (Data Sources)   │      │  (Transformers)    │
└───────┬────────┘         └─────────┬────────┘      └──────────┬─────────┘
        │                            │                           │
        │                            │                           │
┌───────▼────────────────────────────▼───────────────────────────▼─────────┐
│                          Compiled Classes                                │
│                 (Aggregated Outputs & Collections)                       │
└──────────────────────────────────────────────────────────────────────────┘
```

## Module Organization

```
mtgjson-rust/
│
├── src/
│   ├── lib.rs                 (PyO3 module entry point)
│   │
│   ├── classes/               (MTGJSON Data Structures)
│   │   ├── mod.rs
│   │   ├── card.rs           (MtgjsonCardObject)
│   │   ├── set.rs            (MtgjsonSetObject)
│   │   ├── deck.rs           (Deck structures)
│   │   ├── prices.rs         (Price data)
│   │   ├── identifiers.rs    (Card identifiers)
│   │   ├── legalities.rs     (Format legality)
│   │   ├── foreign_data.rs   (Translations)
│   │   ├── sealed_product.rs (Product data)
│   │   └── ...               (15+ data types)
│   │
│   ├── providers/            (External Data Sources)
│   │   ├── mod.rs
│   │   ├── provider_base.rs  (Base traits & types)
│   │   │
│   │   ├── scryfall/         (Primary MTG API)
│   │   │   ├── monolith.rs   (Main Scryfall provider)
│   │   │   └── orientation_detector.rs
│   │   │
│   │   ├── third_party/      (Pricing & supplemental data)
│   │   │   ├── tcgplayer.rs
│   │   │   ├── cardkingdom.rs
│   │   │   ├── cardhoarder.rs
│   │   │   ├── cardmarket.rs
│   │   │   ├── mtgban.rs
│   │   │   ├── gatherer.rs
│   │   │   └── wizards.rs
│   │   │
│   │   ├── github/           (Community datasets)
│   │   │   ├── boosters.rs
│   │   │   ├── decks.rs
│   │   │   ├── sealed.rs
│   │   │   └── mtgsqlite.rs
│   │   │
│   │   ├── edhrec/          (Commander stats)
│   │   │   └── card_ranks.rs
│   │   │
│   │   └── mtgwiki/         (Wiki data)
│   │       └── secret_lair.rs
│   │
│   ├── builders/            (Data Processing Pipeline)
│   │   ├── mod.rs
│   │   ├── set_builder.rs   (Build set objects)
│   │   ├── set_builder_functions.rs (PyO3 wrappers)
│   │   ├── price_builder.rs (Aggregate pricing)
│   │   ├── output_generator.rs (JSON output)
│   │   └── parallel_call.rs (Async processing)
│   │
│   ├── compiled_classes/    (Output Formats)
│   │   ├── mod.rs
│   │   ├── all_printings.rs
│   │   ├── all_identifiers.rs
│   │   ├── atomic_cards.rs
│   │   ├── set_list.rs
│   │   ├── deck_list.rs
│   │   ├── keywords.rs
│   │   └── ...
│   │
│   ├── constants.rs         (Static configuration)
│   └── utils_functions.rs   (Shared utilities)
│
└── Cargo.toml               (Dependencies & config)
```

## Component Architecture Diagram

```
┌────────────────────────────────────────────────────────────────────────┐
│                         MTGJSON Rust Library                           │
│                                                                        │
│  ┌──────────────────────────────────────────────────────────────────┐ │
│  │                      PyO3 Bindings Layer                         │ │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐        │ │
│  │  │ Classes  │  │Providers │  │ Builders │  │Compiled  │        │ │
│  │  │ (15+)    │  │ (14+)    │  │  (4)     │  │ (10+)    │        │ │
│  │  └────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬─────┘        │ │
│  └───────┼─────────────┼─────────────┼─────────────┼──────────────┘ │
│          │             │             │             │                │
│  ┌───────▼─────────────▼─────────────▼─────────────▼──────────────┐ │
│  │                     Core Business Logic                         │ │
│  │                                                                  │ │
│  │  ┌─────────────┐    ┌──────────────┐    ┌──────────────┐       │ │
│  │  │   Card      │    │   Set        │    │  Price       │       │ │
│  │  │   Parser    │───▶│   Builder    │───▶│  Aggregator  │       │ │
│  │  └─────────────┘    └──────┬───────┘    └──────────────┘       │ │
│  │                             │                                   │ │
│  │  ┌─────────────┐    ┌──────▼───────┐    ┌──────────────┐       │ │
│  │  │  Legality   │    │   Metadata   │    │  Output      │       │ │
│  │  │  Resolver   │───▶│   Enricher   │───▶│  Generator   │       │ │
│  │  └─────────────┘    └──────────────┘    └──────────────┘       │ │
│  └──────────────────────────────────────────────────────────────┘ │
│                                                                        │
│  ┌──────────────────────────────────────────────────────────────────┐ │
│  │                    Data Layer & I/O                              │ │
│  │                                                                  │ │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐             │ │
│  │  │  Scryfall   │  │ TCGPlayer   │  │  CardMarket │             │ │
│  │  │   Client    │  │   Client    │  │   Client    │             │ │
│  │  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘             │ │
│  │         │                │                │                     │ │
│  │  ┌──────▼────────────────▼────────────────▼──────┐             │ │
│  │  │         HTTP Client (reqwest)                 │             │ │
│  │  └───────────────────────────────────────────────┘             │ │
│  └──────────────────────────────────────────────────────────────┘ │
│                                                                        │
│  ┌──────────────────────────────────────────────────────────────────┐ │
│  │                   Supporting Infrastructure                      │ │
│  │                                                                  │ │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐             │ │
│  │  │   Async     │  │  Parallel   │  │   Rate      │             │ │
│  │  │  Runtime    │  │  Processing │  │  Limiting   │             │ │
│  │  │  (tokio)    │  │  (rayon)    │  │             │             │ │
│  │  └─────────────┘  └─────────────┘  └─────────────┘             │ │
│  └──────────────────────────────────────────────────────────────┘ │
└────────────────────────────────────────────────────────────────────────┘
```

## Data Flow Architecture

```
┌─────────────┐
│   Python    │
│ Application │
└──────┬──────┘
       │
       │ FFI Call (PyO3)
       │
┌──────▼──────────────────────────────────────────────────────────────┐
│                       Rust Entry Point                              │
│                                                                     │
│  ┌──────────────────────────────────────────────────────────┐      │
│  │  1. Parse Input Parameters (Python → Rust)              │      │
│  └────────────────────┬─────────────────────────────────────┘      │
│                       │                                            │
│  ┌────────────────────▼─────────────────────────────────────┐      │
│  │  2. Fetch Data from Providers                            │      │
│  │     ┌────────────┐  ┌───────────┐  ┌──────────────┐     │      │
│  │     │ Scryfall   │  │ TCGPlayer │  │   GitHub     │     │      │
│  │     │   API      │  │   API     │  │   Datasets   │     │      │
│  │     └─────┬──────┘  └─────┬─────┘  └──────┬───────┘     │      │
│  │           └────────────────┴────────────────┘             │      │
│  └────────────────────┬─────────────────────────────────────┘      │
│                       │                                            │
│  ┌────────────────────▼─────────────────────────────────────┐      │
│  │  3. Parse & Normalize Data                              │      │
│  │     - Parse card types                                   │      │
│  │     - Calculate CMC                                      │      │
│  │     - Resolve colors                                     │      │
│  │     - Parse legalities                                   │      │
│  └────────────────────┬─────────────────────────────────────┘      │
│                       │                                            │
│  ┌────────────────────▼─────────────────────────────────────┐      │
│  │  4. Build MTGJSON Objects                                │      │
│  │     - Create Card objects                                │      │
│  │     - Build Set objects                                  │      │
│  │     - Aggregate prices                                   │      │
│  │     - Add metadata                                       │      │
│  └────────────────────┬─────────────────────────────────────┘      │
│                       │                                            │
│  ┌────────────────────▼─────────────────────────────────────┐      │
│  │  5. Parallel Processing (Optional)                       │      │
│  │     - Batch operations (rayon)                           │      │
│  │     - Async tasks (tokio)                                │      │
│  └────────────────────┬─────────────────────────────────────┘      │
│                       │                                            │
│  ┌────────────────────▼─────────────────────────────────────┐      │
│  │  6. Generate Output                                      │      │
│  │     - Serialize to JSON                                  │      │
│  │     - Create compiled files                              │      │
│  │     - Apply formatting                                   │      │
│  └────────────────────┬─────────────────────────────────────┘      │
│                       │                                            │
│  ┌────────────────────▼─────────────────────────────────────┐      │
│  │  7. Return Results (Rust → Python)                       │      │
│  └──────────────────────────────────────────────────────────┘      │
└─────────────────────────────────────────────────────────────────────┘
       │
       │ FFI Return
       │
┌──────▼──────┐
│   Python    │
│   Results   │
└─────────────┘
```

## Key Design Patterns

### 1. Separation of Concerns

- **Classes**: Pure data structures with minimal logic
- **Providers**: External data source interfaces
- **Builders**: Business logic for data transformation
- **Compiled Classes**: Output aggregation & formatting

### 2. Python Interoperability (PyO3)

- All public types use `#[pyclass]` macro
- Methods use `#[pymethods]` for Python exposure
- Type conversions handled automatically
- Error handling converted to Python exceptions

### 3. Performance Optimization

- **Parallel Processing**: Rayon for CPU-bound tasks
- **Async I/O**: Tokio for network requests
- **Fast Collections**: AHash for HashMap operations
- **Zero-Copy**: SmallVec for inline storage

### 4. Provider Pattern

```rust
pub trait AbstractProvider {
    fn initialize() -> Self;
    fn get_provider_name() -> &'static str;
    fn download() -> ProviderResult<Data>;
}
```

Each provider implements:
- Rate limiting
- Retry logic
- Error handling
- Data normalization

## Dependencies

### Core Libraries

- **pyo3** (0.22): Python FFI bindings
- **serde** (1.0): Serialization framework
- **tokio** (1.x): Async runtime
- **reqwest** (0.12): HTTP client

### Performance

- **rayon** (1.7): Data parallelism
- **ahash** (0.8): Fast hashing
- **smallvec** (1.10): Stack-allocated vectors
- **rustc-hash** (1.1): Fast HashMap

### Utilities

- **chrono** (0.4): Date/time handling
- **regex** (1.5): Pattern matching
- **thiserror** (1.0): Error definitions
- **scraper** (0.20): HTML parsing

## Build Configuration

### Crate Type

```toml
[lib]
crate-type = ["cdylib"]  # Shared library for Python
```

### Features

- `extension-module`: PyO3 Python module support
- `experimental-async`: Async Python support

## Thread Safety

- All providers implement `Send + Sync`
- Rate limiters use `Arc<Mutex<>>`
- Parallel processing via Rayon's work-stealing
- Async tasks coordinated through Tokio

## Error Handling Strategy

```rust
pub enum ProviderError {
    NetworkError(String),
    ParseError(String),
    AuthError(String),
    RateLimitError,
    ConfigurationError(String),
    ProcessingError(String),
}
```

Errors propagate through:
1. Rust `Result<T, E>` types
2. `thiserror` for error definitions
3. PyO3 conversion to Python exceptions

## Testing Architecture

- **Unit tests**: Per-module coverage
- **Comprehensive tests**: End-to-end scenarios
- **Test framework**: Custom helpers in `tests/`
- **Integration**: Python interop testing

## Output Formats

Generated files include:

1. **AllPrintings.json** - Complete card database
2. **AllIdentifiers.json** - Card ID mappings
3. **AtomicCards.json** - Unique card instances
4. **SetList.json** - Set metadata
5. **DeckList.json** - Preconstructed decks
6. **Keywords.json** - Game keywords
7. **EnumValues.json** - Valid field values

## Performance Characteristics

- **Concurrency**: Multi-threaded via Rayon
- **I/O**: Async HTTP with connection pooling
- **Memory**: Stack allocation where possible
- **Serialization**: Zero-copy with serde_json

## Future Architecture Considerations

1. **Caching Layer**: Persistent provider caching
2. **Database Backend**: Optional SQLite storage
3. **Incremental Updates**: Delta processing
4. **GraphQL API**: Alternative query interface
5. **WebAssembly**: Browser-based processing
