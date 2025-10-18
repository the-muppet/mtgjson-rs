# MTGJSON Rust Data Flow Diagrams

## Complete End-to-End Data Flow

```mermaid
flowchart TB
    START([Python Application])

    subgraph FFI["PyO3 FFI Boundary"]
        ENTRY[mtgjson_rust.build_mtgjson_set_wrapper]
    end

    subgraph FETCH["Data Acquisition Phase"]
        SCRY[Scryfall API]
        TCG[TCGPlayer API]
        CK[CardKingdom Scraper]
        CM[CardMarket API]
        GH[GitHub Datasets]
        EDHREC[EDHRec API]
        WIKI[MTG Wiki]

        RATE{Rate Limit Check}
        RETRY{Retry Logic}
        CACHE{Cache Hit?}
    end

    subgraph PARSE["Parsing Phase"]
        JSONPARSE[Parse JSON Response]
        HTMLPARSE[Parse HTML Content]
        VALIDATE[Validate Data Schema]
        NORMALIZE[Normalize Field Names]
    end

    subgraph TRANSFORM["Transformation Phase"]
        CARDTYPE[Parse Card Types]
        COLORS[Calculate Colors]
        CMC[Calculate CMC]
        LEGAL[Parse Legalities]
        FOREIGN[Parse Foreign Data]
        RULINGS[Parse Rulings]
        PRINTINGS[Parse Printings]
    end

    subgraph BUILD["Object Construction"]
        CARD[Build MtgjsonCardObject]
        IDENT[Build MtgjsonIdentifiers]
        PRICES[Build MtgjsonPrices]
        LEGALITIES[Build MtgjsonLegalities]
        SET[Build MtgjsonSetObject]
    end

    subgraph ENRICH["Enrichment Phase"]
        METADATA[Add Metadata]
        LEADERSHIP[Add Leadership Skills]
        UUID[Generate UUIDs]
        TRANS[Add Translations]
        SEALED[Add Sealed Products]
    end

    subgraph AGGREGATE["Aggregation Phase"]
        ALLPRINT[MtgjsonAllPrintings]
        ATOMIC[MtgjsonAtomicCards]
        SETLIST[MtgjsonSetList]
        DECKLIST[MtgjsonDeckList]
        KEYWORDS[MtgjsonKeywords]
    end

    subgraph OUTPUT["Output Generation"]
        SERIALIZE[Serialize to JSON]
        FORMAT[Format JSON]
        VALIDATE2[Validate Output]
        WRITE[Write to File/Return]
    end

    START --> ENTRY

    ENTRY --> CACHE
    CACHE -->|Miss| RATE
    CACHE -->|Hit| PARSE
    RATE -->|OK| SCRY
    RATE -->|Limited| RETRY
    RETRY --> RATE

    SCRY --> JSONPARSE
    TCG --> JSONPARSE
    CM --> JSONPARSE
    GH --> JSONPARSE
    EDHREC --> JSONPARSE

    CK --> HTMLPARSE
    WIKI --> HTMLPARSE

    JSONPARSE --> VALIDATE
    HTMLPARSE --> VALIDATE
    VALIDATE --> NORMALIZE

    NORMALIZE --> CARDTYPE
    NORMALIZE --> COLORS
    NORMALIZE --> CMC
    NORMALIZE --> LEGAL
    NORMALIZE --> FOREIGN
    NORMALIZE --> RULINGS
    NORMALIZE --> PRINTINGS

    CARDTYPE --> CARD
    COLORS --> CARD
    CMC --> CARD
    LEGAL --> LEGALITIES
    FOREIGN --> CARD
    RULINGS --> CARD
    PRINTINGS --> CARD

    TCG --> PRICES
    CK --> PRICES
    CM --> PRICES

    SCRY --> IDENT

    CARD --> SET
    LEGALITIES --> CARD
    PRICES --> CARD
    IDENT --> CARD

    SET --> METADATA
    METADATA --> LEADERSHIP
    LEADERSHIP --> UUID
    UUID --> TRANS
    TRANS --> SEALED

    SEALED --> ALLPRINT
    SEALED --> ATOMIC
    SEALED --> SETLIST
    SEALED --> DECKLIST
    SEALED --> KEYWORDS

    ALLPRINT --> SERIALIZE
    ATOMIC --> SERIALIZE
    SETLIST --> SERIALIZE
    DECKLIST --> SERIALIZE
    KEYWORDS --> SERIALIZE

    SERIALIZE --> FORMAT
    FORMAT --> VALIDATE2
    VALIDATE2 --> WRITE

    WRITE --> START
```

## Set Building Sequence Diagram

```mermaid
sequenceDiagram
    participant PY as Python App
    participant SB as SetBuilder
    participant SF as ScryfallProvider
    participant TP as TCGPlayerProvider
    participant CK as CardKingdomProvider
    participant CP as CardParser
    participant PB as PriceBuilder
    participant OG as OutputGenerator

    PY->>SB: build_mtgjson_set("NEO")

    Note over SB: Phase 1: Data Collection
    SB->>SF: fetch_set_data("NEO")
    SF->>SF: check_rate_limit()
    SF-->>SB: Set JSON + Cards JSON

    SB->>SF: fetch_booster_data("NEO")
    SF-->>SB: Booster configuration

    SB->>SF: fetch_translations("NEO")
    SF-->>SB: Translation data

    Note over SB: Phase 2: Card Parsing
    loop For each card in set
        SB->>CP: parse_card_types(card_data)
        CP-->>SB: (types, supertypes, subtypes)

        SB->>CP: get_card_colors(mana_cost)
        CP-->>SB: ["U", "R"]

        SB->>CP: get_card_cmc(mana_cost)
        CP-->>SB: 3.0

        SB->>CP: parse_legalities(formats)
        CP-->>SB: MtgjsonLegalities

        SB->>CP: parse_foreign(foreign_data)
        CP-->>SB: Vec<MtgjsonForeignData>

        SB->>CP: parse_rulings(rulings_data)
        CP-->>SB: Vec<MtgjsonRulings>
    end

    Note over SB: Phase 3: Price Collection
    par Parallel Price Fetching
        SB->>TP: fetch_prices(card_ids)
        TP-->>SB: TCGPlayer prices
    and
        SB->>CK: scrape_prices(card_names)
        CK-->>SB: CardKingdom prices
    end

    SB->>PB: aggregate_prices(all_price_data)
    PB-->>SB: MtgjsonPrices objects

    Note over SB: Phase 4: Object Construction
    SB->>SB: build_base_mtgjson_cards()
    SB->>SB: enhance_cards_with_metadata()
    SB->>SB: add_leadership_skills()
    SB->>SB: add_uuid_placeholders()

    Note over SB: Phase 5: Set Assembly
    SB->>SB: create MtgjsonSetObject
    SB->>SB: add cards to set
    SB->>SB: add translations
    SB->>SB: add sealed products

    Note over SB: Phase 6: Output
    SB->>OG: generate_json(set_object)
    OG->>OG: serialize to JSON
    OG->>OG: format JSON
    OG-->>SB: JSON string

    SB-->>PY: MtgjsonSetObject
```

## Card Object Construction Flow

```
┌─────────────────────────────────────────────────────────────────┐
│              Raw Scryfall Card JSON                             │
│  {                                                              │
│    "name": "Lightning Bolt",                                    │
│    "mana_cost": "{R}",                                          │
│    "type_line": "Instant",                                      │
│    "oracle_text": "Deal 3 damage...",                           │
│    "legalities": {...},                                         │
│    ...                                                          │
│  }                                                              │
└────────────────────────┬────────────────────────────────────────┘
                         │
                         ▼
         ┌───────────────────────────┐
         │   Validation & Schema     │
         │   Checking                │
         └───────────┬───────────────┘
                     │
                     ▼
    ┌────────────────────────────────────┐
    │     Field Extraction               │
    ├────────────────────────────────────┤
    │ name         → "Lightning Bolt"    │
    │ mana_cost    → "{R}"               │
    │ type_line    → "Instant"           │
    │ oracle_text  → "Deal 3 damage..."  │
    └────────────┬───────────────────────┘
                 │
    ┌────────────┴────────────┬──────────────────┬──────────────┐
    │                         │                  │              │
    ▼                         ▼                  ▼              ▼
┌─────────┐         ┌──────────────┐   ┌─────────────┐  ┌──────────┐
│Parse    │         │Get Colors    │   │Calculate    │  │Parse     │
│Card     │         │from Mana     │   │CMC          │  │Legalities│
│Types    │         │Cost          │   │             │  │          │
└────┬────┘         └──────┬───────┘   └──────┬──────┘  └─────┬────┘
     │                     │                  │               │
     │                     │                  │               │
     │    ┌────────────────┴──────────────────┴───────────────┘
     │    │
     │    ▼
     │  ┌──────────────────────────────────────┐
     │  │   Parallel Processing                │
     │  │   - Foreign data parsing             │
     │  │   - Ruling extraction                │
     │  │   - Printings resolution             │
     │  └──────────────────┬───────────────────┘
     │                     │
     ▼                     ▼
┌──────────────────────────────────────────┐
│     MtgjsonCardObject Assembly           │
├──────────────────────────────────────────┤
│ uuid: "generated-uuid-here"              │
│ name: "Lightning Bolt"                   │
│ colors: ["R"]                            │
│ cmc: 1.0                                 │
│ types: ["Instant"]                       │
│ supertypes: []                           │
│ subtypes: []                             │
│ identifiers: MtgjsonIdentifiers {...}    │
│ legalities: MtgjsonLegalities {...}      │
│ foreign_data: Vec<ForeignData> {...}     │
│ rulings: Vec<Ruling> {...}               │
└──────────────────┬───────────────────────┘
                   │
                   ▼
        ┌─────────────────────┐
        │  Price Enrichment   │
        │  (Async)            │
        └──────────┬──────────┘
                   │
                   ▼
        ┌─────────────────────┐
        │ Final Card Object   │
        │ with all metadata   │
        └─────────────────────┘
```

## Price Aggregation Flow

```mermaid
flowchart LR
    CARD[Card Identifiers]

    subgraph PROVIDERS["Price Providers"]
        TCG[TCGPlayer]
        CK[CardKingdom]
        CH[CardHoarder]
        CM[CardMarket]
    end

    subgraph FETCH["Parallel Fetch"]
        F1[Fetch TCGPlayer]
        F2[Fetch CardKingdom]
        F3[Fetch CardHoarder]
        F4[Fetch CardMarket]
    end

    subgraph PARSE["Parse Responses"]
        P1[Parse TCG JSON]
        P2[Parse CK HTML]
        P3[Parse CH JSON]
        P4[Parse CM JSON]
    end

    subgraph NORMALIZE["Normalize Prices"]
        N1[USD Pricing]
        N2[EUR Pricing]
        N3[MTGO Pricing]
        N4[Paper Pricing]
    end

    subgraph AGG["Aggregate"]
        MERGE[Merge All Prices]
        DEDUPE[Remove Duplicates]
        FORMAT[Format Structure]
    end

    RESULT[MtgjsonPrices Object]

    CARD --> TCG
    CARD --> CK
    CARD --> CH
    CARD --> CM

    TCG --> F1 --> P1 --> N1
    CK --> F2 --> P2 --> N2
    CH --> F3 --> P3 --> N3
    CM --> F4 --> P4 --> N4

    N1 --> MERGE
    N2 --> MERGE
    N3 --> MERGE
    N4 --> MERGE

    MERGE --> DEDUPE
    DEDUPE --> FORMAT
    FORMAT --> RESULT
```

## Parallel Processing Architecture

```
Main Thread
    │
    ├─> Create Rayon Thread Pool (num_cpus cores)
    │
    ├─> Partition Card Data into Chunks
    │   │
    │   ├─> Chunk 1 (cards 1-100)    ───> Worker Thread 1
    │   │                                       │
    │   │                                       ├─> Parse card types
    │   │                                       ├─> Calculate colors
    │   │                                       ├─> Calculate CMC
    │   │                                       └─> Build card objects
    │   │
    │   ├─> Chunk 2 (cards 101-200)  ───> Worker Thread 2
    │   │                                       │
    │   │                                       └─> (same operations)
    │   │
    │   ├─> Chunk 3 (cards 201-300)  ───> Worker Thread 3
    │   │
    │   └─> Chunk N (cards ...)      ───> Worker Thread N
    │
    ├─> Collect Results (work-stealing scheduler)
    │
    └─> Merge into Final Vec<MtgjsonCardObject>


Async I/O (Tokio)
    │
    ├─> Spawn Multiple Futures
    │   │
    │   ├─> Future 1: Fetch Scryfall Set
    │   ├─> Future 2: Fetch Scryfall Translations
    │   ├─> Future 3: Fetch TCGPlayer Prices
    │   ├─> Future 4: Fetch CardKingdom Prices
    │   ├─> Future 5: Fetch EDHRec Rankings
    │   └─> Future N: Fetch GitHub Data
    │
    ├─> Await All Futures Concurrently
    │
    └─> Combine Results
```

## Error Handling Flow

```mermaid
flowchart TD
    START[API Request]

    REQ{HTTP Request}
    RESP{Response Status}

    SUCCESS[200 OK]
    RATELIMIT[429 Rate Limited]
    AUTH[401/403 Auth Error]
    NOTFOUND[404 Not Found]
    SERVERERR[5xx Server Error]
    NETWORK[Network Error]

    RETRY{Retry Count < Max?}
    BACKOFF[Exponential Backoff]
    CACHE{Fallback Cache?}
    DEFAULT{Default Value?}

    RESULT[Return Result]
    ERROR[Throw ProviderError]

    START --> REQ
    REQ --> RESP

    RESP --> SUCCESS
    RESP --> RATELIMIT
    RESP --> AUTH
    RESP --> NOTFOUND
    RESP --> SERVERERR
    RESP --> NETWORK

    SUCCESS --> RESULT

    RATELIMIT --> RETRY
    RETRY -->|Yes| BACKOFF
    BACKOFF --> REQ
    RETRY -->|No| ERROR

    AUTH --> ERROR

    NOTFOUND --> CACHE
    CACHE -->|Hit| RESULT
    CACHE -->|Miss| DEFAULT
    DEFAULT -->|Available| RESULT
    DEFAULT -->|None| ERROR

    SERVERERR --> RETRY
    NETWORK --> RETRY
```

## Compiled Output Generation Flow

```
┌─────────────────────────────────────────────────────────┐
│         Collection of MtgjsonSetObject (200+ sets)      │
└────────────────────────┬────────────────────────────────┘
                         │
            ┌────────────┴────────────┐
            │                         │
            ▼                         ▼
    ┌───────────────┐         ┌──────────────┐
    │ AllPrintings  │         │  SetList     │
    │               │         │  (metadata)  │
    │ Full sets with│         │              │
    │ all card data │         │ Sets without │
    └───────┬───────┘         │ card arrays  │
            │                 └──────────────┘
            │
            ▼
    ┌───────────────────┐
    │ Extract All Cards │
    └────────┬──────────┘
             │
             ▼
    ┌────────────────────┐
    │ Deduplicate Cards  │
    │ by oracle_id       │
    └────────┬───────────┘
             │
             ▼
    ┌────────────────────┐
    │  AtomicCards       │
    │  (unique cards)    │
    └────────────────────┘
             │
             ▼
    ┌────────────────────┐
    │ Extract Identifiers│
    └────────┬───────────┘
             │
             ▼
    ┌────────────────────┐
    │  AllIdentifiers    │
    │  (UUID mappings)   │
    └────────────────────┘
             │
             ▼
    ┌────────────────────┐
    │ Extract Keywords   │
    │ & Enum Values      │
    └────────┬───────────┘
             │
             ├───> Keywords.json
             ├───> EnumValues.json
             └───> CardTypes.json
```

## Real-Time Processing Timeline

```
Time (seconds)
0.0  │ Start: build_mtgjson_set("NEO")
     │
0.1  ├─> Initialize providers
     │   └─> Setup rate limiters
     │
0.2  ├─> Fetch Scryfall set metadata (async)
     │
0.5  ├─> Fetch Scryfall cards (async, bulk)
     │
2.0  ├─> Parse 300+ cards (parallel, rayon)
     │   ├─> Thread 1: Cards 1-75
     │   ├─> Thread 2: Cards 76-150
     │   ├─> Thread 3: Cards 151-225
     │   └─> Thread 4: Cards 226-300
     │
3.5  ├─> Fetch prices (async, parallel)
     │   ├─> TCGPlayer API (1.2s)
     │   ├─> CardKingdom scrape (2.1s)
     │   └─> CardMarket API (1.5s)
     │
5.6  ├─> Aggregate prices
     │
6.0  ├─> Enrich cards with metadata
     │   └─> Add UUIDs, leadership skills
     │
6.5  ├─> Build set object
     │
7.0  ├─> Generate JSON output
     │
7.2  └─> Return to Python
```

## State Transitions

```
[Uninitialized Provider]
         │
         ▼
[Provider Initialized]
         │
         ├─> Rate Limit Check ──> [Waiting]
         │                            │
         │                            └─> [Ready]
         ▼                                   │
    [Fetching Data] <──────────────────────┘
         │
         ├─> Success ──> [Data Downloaded]
         │                      │
         │                      ▼
         │              [Parsing Response]
         │                      │
         │                      ├─> Success ──> [Parsed Data]
         │                      │
         │                      └─> Error ──> [Parse Failed]
         │
         └─> Error ──> [Network Failed]
                            │
                            └─> Retry ──> [Fetching Data]
                            │
                            └─> Max Retries ──> [Provider Error]
```

## Memory Flow Pattern

```
Stack Allocation
    │
    ├─> Small structures (< 256 bytes)
    │   ├─> Identifiers
    │   ├─> Legalities
    │   └─> Basic types
    │
    └─> SmallVec for small collections
        └─> Color arrays, type arrays

Heap Allocation
    │
    ├─> Large collections
    │   ├─> Vec<Card> (1000+ cards)
    │   ├─> HashMap<String, Price>
    │   └─> String buffers
    │
    └─> Arc for shared data
        └─> Rate limiters, HTTP clients

Zero-Copy Operations
    │
    ├─> JSON parsing (serde_json)
    │   └─> Direct deserialization
    │
    └─> String slices where possible
```

## PyO3 FFI Data Flow

```
Python                    Boundary                    Rust
  │                          │                          │
  ├─> dict/list/str ────────>│─> serde_json::Value ────>│─> Struct
  │                          │                          │
  │                          │   Type Conversion        │
  │                          │   (automatic)            │
  │                          │                          │
  │<── dict/list/str <────────│<── Serialize ───────────│<── Struct
  │                          │                          │
  ├─> Exception ─────────────>│─> PyErr ────────────────>│─> Result::Err
  │<── PyException <──────────│<── ProviderError <──────│
  │                          │                          │
```
