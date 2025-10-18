# Epic: MTGJSON Rust Compilation & Parallelization Implementation

## Feature Vision

**Goal**: Fix 31 compilation errors and implement parallel development strategy for MTGJSON Rust port
**Target Users**: MTGJSON consumers requiring high-performance Rust bindings
**Success Metric**: Zero compilation errors, 100% test coverage for critical paths
**Timeline**: 3 weeks (accelerated with parallelization)
**Team Size**: 3-4 engineers (or 1 engineer with parallel branches)
**Feature Type**: Critical Bug Fix & Infrastructure Enhancement

## Current State Analysis

### Existing Architecture

**Relevant Components**:
- **Classes Layer** (15 types): Complete but has PyO3 API issues
- **Providers Layer** (12 integrations): Missing GitHub module entirely
- **Builders Layer**: Type signature mismatches blocking compilation
- **Compiled Classes**: Ready but unused until compilation succeeds

**Integration Points**:
- **Python Interface**: PyO3 bindings outdated (v0.22 API changes)
- **HTTP Layer**: BaseProvider working but providers can't compile
- **Async Runtime**: Tokio integrated but untestable

**Existing Patterns to Follow**:
- **Provider Pattern**: EdhrecProviderCardRanks as template for GitHub module
- **PyO3 Patterns**: new_bound() methods throughout (v0.22 standard)
- **Error Handling**: ProviderResult<T> and PyResult<T> dual-error system

**Technical Debt Considerations**:
- **100 Warnings**: Unused imports, variables, shadowing - defer to Phase 3
- **Stub Implementations**: Many providers minimal - defer to future sprints
- **Documentation Gaps**: Inline docs missing - defer to post-compilation

## Feature Requirements

### User-Facing Changes

- Compilation succeeds allowing Python import of mtgjson_rust module
- All provider APIs functional (GitHub, Scryfall, TCGPlayer, etc.)
- High-level functions available: build_single_set, build_all_sets, write_to_file
- Performance: 3-5x faster than Python implementation

### Technical Requirements

- Fix 31 compilation errors across 20+ files
- Create missing GitHub provider module (6 files)
- Update all PyO3 API calls to v0.22 standard
- Fix method signature mismatches in builders
- Resolve lifetime issues in foreign_data.rs

### Non-Functional Requirements

- **Performance**: < 100ms provider response time p95
- **Scalability**: Support parallel set building (10+ concurrent)
- **Security**: Safe Rust patterns, no unsafe blocks
- **Compatibility**: Python 3.8+ support via PyO3

## Success Criteria

- [ ] Zero compilation errors (`cargo build` succeeds)
- [ ] All 31 original errors resolved
- [ ] GitHub module fully implemented with 6 providers
- [ ] PyO3 v0.22 API migration complete
- [ ] Python can import and call mtgjson_rust functions
- [ ] Integration tests pass for critical paths
- [ ] Benchmark shows 3x+ performance improvement

## Workstream Decomposition

### Workstream 1: GitHub Module Implementation (Rust/PyO3)

**Focus**: Create missing GitHub provider infrastructure

**Scope**:
- **New Code**: 6 new files in src/providers/github/
- **Modified Code**: src/providers/mod.rs (add module declaration)
- **Unchanged**: All other provider modules

**Deliverables**:
- Create `src/providers/github/mod.rs` with module exports
- Implement GitHubBoostersProvider with PyO3 bindings
- Implement GitHubDecksProvider following BaseProvider pattern
- Implement GitHubSealedProvider with async download support
- Implement GitHubCardSealedProductsProvider with caching
- Implement GitHubMTGSqliteProvider with database operations
- Add comprehensive tests for each provider

**Existing Code Integration**:
- **Modifies**: `src/providers/mod.rs:31` (uncomment github module)
- **Extends**: BaseProvider patterns from other providers
- **New**: 6 files totaling ~800 lines

**Technology Stack**:
- Rust 1.70+ (existing)
- PyO3 0.22 (existing)
- async-trait 0.1 (existing)
- tokio 1.0 (existing)

**Dependencies**:
- **On Existing Systems**:
  - BaseProvider: Fully implemented and working
  - AbstractProvider trait: Well-defined interface
- **On Other Workstreams**:
  - None - completely isolated
- **Provided to Others**:
  - To WS3: Working GitHub providers for testing

**Backwards Compatibility**:
- N/A - New functionality only

**Timeline**: Days 1-2
**Estimated Effort**: 8 person-hours
**Team Size**: 1 engineer

**Key Milestones**:
- Hour 2: Module structure complete
- Hour 4: 3 providers implemented
- Hour 8: All 6 providers with tests

---

### Workstream 2: PyO3 API Migration (Python Integration)

**Focus**: Update all PyO3 calls to v0.22 API standard

**Scope**:
- **Modified Code**: 20+ files with PyO3 usage
- **Pattern Changes**: PyList::new → new_bound, PyDict::new → new_bound
- **Unchanged**: Business logic, provider algorithms

**Deliverables**:
- Migrate PyList::new to PyList::new_bound (15 occurrences)
- Update PyDict::new to PyDict::new_bound (10 occurrences)
- Fix PySet::empty to PySet::new_bound (5 occurrences)
- Replace PyModule::import with import_bound (8 occurrences)
- Update CString::new() to CString::new("") (3 occurrences)
- Replace py.run() with PyModule::from_code_bound (2 occurrences)
- Fix all PyResult<Bound<'_, T>> lifetime patterns

**Existing Code Integration**:
- **Modifies**:
  - src/providers/third_party/*.rs (all files)
  - src/providers/cardmarket/monolith.rs
  - src/providers/scryfall/monolith.rs
  - src/classes/card.rs
  - src/builders/price_builder.rs
- **Extends**: None
- **New**: None

**Technology Stack**:
- PyO3 0.22 (upgrade from older version)
- Rust lifetime system (existing)

**Dependencies**:
- **On Existing Systems**:
  - PyO3 0.22 documentation for API changes
- **On Other Workstreams**:
  - Potential conflicts with WS3 in parallel_call.rs
  - Potential conflicts with WS4 in foreign_data.rs
- **Provided to Others**:
  - To WS3/4: Updated PyO3 patterns to follow

**Migration Strategy**:
```rust
// Before (Old API)
let py_list = PyList::new(py, items);
let py_dict = PyDict::new(py);

// After (New API)
let py_list = PyList::new_bound(py, items);
let py_dict = PyDict::new_bound(py);
```

**Timeline**: Days 1-2
**Estimated Effort**: 12 person-hours
**Team Size**: 1 engineer

**Key Milestones**:
- Hour 3: High-priority providers updated
- Hour 6: All providers migrated
- Hour 9: Classes layer migrated
- Hour 12: Builders and edge cases complete

---

### Workstream 3: Type & Method Signature Fixes (Core Logic)

**Focus**: Fix method call signatures and type mismatches

**Scope**:
- **Modified Code**: src/builders/set_builder.rs, parallel_call.rs
- **Fix Pattern**: Add missing Python context and parameters
- **Unchanged**: Provider implementations themselves

**Deliverables**:
- Fix download_all_pages calls (add py, url, params arguments)
- Fix download_cards calls (add py, set_code arguments)
- Update parallel_call.rs type annotations
- Add proper error propagation with ? operator
- Validate all builder-to-provider calls
- Add integration tests for fixed methods

**Existing Code Integration**:
- **Modifies**:
  - src/builders/set_builder.rs:172, 1522 (method calls)
  - src/builders/parallel_call.rs (type issues)
- **Extends**: None
- **New**: Integration tests

**Technology Stack**:
- Rust type system (existing)
- async/await patterns (existing)

**Dependencies**:
- **On Existing Systems**:
  - Provider method signatures as reference
- **On Other Workstreams**:
  - From WS2: Updated PyO3 patterns in parallel_call.rs
- **Provided to Others**:
  - To Testing: Working builder layer

**Fix Examples**:
```rust
// Before (Missing arguments)
let pages = provider.download_all_pages()?;

// After (Correct arguments)
let pages = provider.download_all_pages(py, starting_url, Some(params))?;
```

**Timeline**: Day 2
**Estimated Effort**: 6 person-hours
**Team Size**: 1 engineer

**Key Milestones**:
- Hour 2: set_builder.rs fixes complete
- Hour 4: parallel_call.rs fixes complete
- Hour 6: Integration tests passing

---

### Workstream 4: Lifetime & Memory Safety Fixes (Rust Safety)

**Focus**: Resolve lifetime issues in foreign_data.rs

**Scope**:
- **Modified Code**: src/classes/foreign_data.rs only
- **Fix Pattern**: Proper Bound<'_, PyDict> lifetime management
- **Unchanged**: Other classes

**Deliverables**:
- Fix to_dict lifetime annotations
- Fix to_dict_bound implementation logic
- Ensure all PyDict operations use new_bound
- Add proper lifetime bounds to methods
- Validate memory safety with miri
- Add unit tests for fixed methods

**Existing Code Integration**:
- **Modifies**: src/classes/foreign_data.rs (439 lines)
- **Extends**: None
- **New**: Lifetime tests

**Technology Stack**:
- Rust lifetime system (existing)
- PyO3 Bound types (existing)

**Dependencies**:
- **On Existing Systems**:
  - PyO3 Bound<'_, T> patterns
- **On Other Workstreams**:
  - From WS2: PyO3 patterns if touching same file
- **Provided to Others**:
  - Clean lifetime patterns for other classes

**Timeline**: Day 2
**Estimated Effort**: 4 person-hours
**Team Size**: 1 engineer

**Key Milestones**:
- Hour 1: Lifetime analysis complete
- Hour 2: to_dict methods fixed
- Hour 4: All tests passing

---

### Workstream 5: High-Level API Implementation (User Interface)

**Focus**: Add Python-facing entry points after compilation succeeds

**Scope**:
- **Modified Code**: src/lib.rs
- **New Functions**: 8 high-level wrappers
- **Unchanged**: Internal implementation

**Deliverables**:
- Implement build_single_set(set_code, include_extras)
- Implement build_multiple_sets(set_codes)
- Implement build_all_sets()
- Implement build_prices_only(set_code)
- Implement write_set_to_file(set_data, path)
- Implement generate_compiled_outputs()
- Implement generate_file_hashes()
- Add Python docstrings and type hints

**Existing Code Integration**:
- **Modifies**: src/lib.rs (add ~200 lines)
- **Extends**: Existing internal functions
- **New**: Python-facing API surface

**Technology Stack**:
- PyO3 #[pyfunction] macros (existing)
- Rust Result/Option handling (existing)

**Dependencies**:
- **On Existing Systems**:
  - All workstreams 1-4 must complete first
  - SetBuilder must be functional
- **On Other Workstreams**:
  - Blocked by WS1-4 compilation fixes
- **Provided to Others**:
  - To Users: Complete Python API

**Timeline**: Day 3
**Estimated Effort**: 8 person-hours
**Team Size**: 1 engineer

**Key Milestones**:
- Hour 2: Core build functions complete
- Hour 4: Output functions complete
- Hour 8: Full test coverage

## Critical Path

```
Parallel Execution Phase (Days 1-2):
WS1: GitHub Module ────────────────┐
WS2: PyO3 Migration ───────────────┤
WS3: Type Fixes ───────────────────┤ → MERGE → Compilation Succeeds (Day 2 EOD)
WS4: Lifetime Fixes ───────────────┘

Sequential Phase (Day 3):
Compilation Success → WS5: High-Level API → Integration Testing → Production Ready

Critical Path: WS2 (longest) → Merge → WS5 → Testing
Total Duration: 3 days with parallel execution (vs 5 days sequential)
```

**Blocking Relationships**:
- WS1-4 must ALL complete before first compilation
- WS5 blocked until compilation succeeds
- Testing blocked until WS5 complete

**Parallel Opportunities**:
- WS1, WS2, WS3, WS4 can run completely in parallel
- Different files = no merge conflicts (except 2 specific files)

## Implementation Phases

### Phase 1: Parallel Compilation Fix Blitz (Days 1-2)

**Goal**: Achieve first successful compilation

**Workstreams Active**: WS1, WS2, WS3, WS4 (all parallel)

**Key Deliverables**:
- GitHub module fully implemented
- All PyO3 API calls updated
- Method signatures fixed
- Lifetime issues resolved

**Success Criteria**:
- [ ] `cargo build` exits with 0 errors
- [ ] All 31 compilation errors resolved
- [ ] No new errors introduced
- [ ] Basic smoke tests pass

### Phase 2: API & Integration (Day 3)

**Goal**: Implement user-facing Python API

**Workstreams Active**: WS5

**Key Deliverables**:
- High-level Python functions
- Integration tests
- Performance benchmarks
- Documentation

**Success Criteria**:
- [ ] Python can import mtgjson_rust
- [ ] build_single_set('NEO') works
- [ ] 3x performance improvement verified
- [ ] All integration tests passing

### Phase 3: Quality & Polish (Day 4)

**Goal**: Clean up warnings and improve quality

**Key Activities**:
- Fix 100 warnings (unused vars, imports)
- Add comprehensive documentation
- Performance profiling
- Security audit

**Success Criteria**:
- [ ] Zero warnings
- [ ] Documentation complete
- [ ] Performance targets met
- [ ] Security review passed

### Phase 4: Production Readiness (Day 5)

**Goal**: Prepare for production deployment

**Key Activities**:
- Production build optimization
- CI/CD pipeline setup
- Release notes preparation
- Deployment guide

**Success Criteria**:
- [ ] Release build successful
- [ ] CI/CD pipeline green
- [ ] Documentation published
- [ ] Ready for v1.0 tag

## Backwards Compatibility Strategy

### Python API Compatibility

**Approach**: Maintain identical function signatures to Python version

**Function Mapping**:
```python
# Python MTGJSON (original)
mtgjson5.build_mtgjson_set(set_code, retry, pretty)

# Rust MTGJSON (new - same signature)
mtgjson_rust.build_mtgjson_set(set_code, retry, pretty)
```

### Data Format Compatibility

**JSON Output**: Byte-for-byte identical to Python version
- Same field names
- Same null handling
- Same number formatting
- Same Unicode handling

### Migration Strategy

**Phased Rollout**:
1. Week 1: Internal testing with diff comparison
2. Week 2: Beta users (5% traffic)
3. Week 3: Gradual rollout (25%, 50%, 75%)
4. Week 4: Full production (100%)

**Rollback Plan**:
- Keep Python version available as fallback
- Feature flag for instant switchover
- Data validation before each release

## Resource Requirements

### Team Allocation

**Option A: Team of 3 (Optimal)**
- **Developer 1**: WS1 (GitHub) + WS5 (API) - Rust specialist
- **Developer 2**: WS2 (PyO3 migration) - PyO3 expert
- **Developer 3**: WS3 (Types) + WS4 (Lifetimes) - Safety specialist

**Option B: Solo Developer**
- Use git branches for parallel work
- Context switch between workstreams
- 5 days total (vs 3 with team)

### Infrastructure Changes

- **Build Server**: Rust toolchain 1.70+
- **CI/CD**: GitHub Actions Rust workflow
- **Testing**: Increase timeout for integration tests
- **Monitoring**: Add Rust application metrics

### Budget Breakdown

- **Personnel**: $15,000 (3 engineers × 1 week)
- **Infrastructure**: $500 (CI/CD improvements)
- **Testing**: $500 (load testing infrastructure)
- **Contingency (15%)**: $2,400
- **Total**: $18,400

## Risk Assessment

### Technical Risks

| Risk | Probability | Impact | Mitigation Strategy |
|------|-------------|--------|---------------------|
| Merge conflicts in shared files | Medium | Low | Coordinate on parallel_call.rs, foreign_data.rs |
| PyO3 API changes incomplete | Low | High | Use PyO3 migration guide, extensive testing |
| Performance regression | Low | Medium | Benchmark before/after, profile hot paths |
| New compilation errors appear | Medium | Medium | Fix immediately, don't accumulate technical debt |

### Delivery Risks

| Risk | Probability | Impact | Mitigation Strategy |
|------|-------------|--------|---------------------|
| Underestimated PyO3 complexity | Medium | Medium | Time-box to 4 hours, escalate if blocked |
| GitHub module takes longer | Low | Low | Use existing providers as templates |
| Integration tests reveal issues | Medium | Medium | Reserve Day 4 for fixes |

## Testing Strategy

### Unit Testing

- **Coverage Target**: 80% for new GitHub module
- **Critical Paths**: 100% for high-level API functions
- **Existing Tests**: Update for PyO3 v0.22 changes

### Integration Testing

```python
# Test Python integration
import mtgjson_rust

# Test basic functionality
result = mtgjson_rust.build_single_set('NEO', include_extras=True)
assert 'cards' in result
assert len(result['cards']) > 0

# Test performance
import time
start = time.time()
mtgjson_rust.build_all_sets()
duration = time.time() - start
assert duration < 300  # Under 5 minutes
```

### Performance Testing

- **Benchmark**: Single set build < 2 seconds
- **Parallel**: 10 concurrent sets < 10 seconds
- **Memory**: Peak usage < 1GB

## Communication Plan

### Daily Standups

```
Day 1: Launch parallel workstreams
Day 2: Merge and compile attempt
Day 3: API implementation
Day 4: Testing and polish
Day 5: Production prep
```

### Stakeholder Updates

- **Day 2 EOD**: Compilation status report
- **Day 3 EOD**: API functionality demo
- **Day 5**: Go/no-go decision

## Coordination Infrastructure

### Multi-Agent Setup

For parallel development with multiple Claude instances:

#### Coordination Tools

Located in `coordination/`:

**sync-status.sh** - Check all workstream statuses
```bash
#!/bin/bash
echo "WS1 GitHub Module: $(git log -1 --oneline feature/github-module)"
echo "WS2 PyO3 Migration: $(git log -1 --oneline feature/pyo3-migration)"
echo "WS3 Type Fixes: $(git log -1 --oneline feature/type-fixes)"
echo "WS4 Lifetime Fixes: $(git log -1 --oneline feature/lifetime-fixes)"
```

**merge-all.sh** - Merge all workstreams
```bash
#!/bin/bash
git checkout main
git merge feature/github-module
git merge feature/pyo3-migration
git merge feature/type-fixes
git merge feature/lifetime-fixes
cargo build
```

### Git Branch Strategy

```bash
# Create branches for parallel work
git checkout -b feature/github-module
git checkout -b feature/pyo3-migration
git checkout -b feature/type-fixes
git checkout -b feature/lifetime-fixes

# Work in parallel on different branches
# Merge when all complete
```

## Success Metrics

### Compilation Success

- **Target**: 0 compilation errors
- **Current**: 31 errors
- **Success**: `cargo build --release` succeeds

### Performance Improvement

- **Single Set Build**: < 2 seconds (vs 6 seconds Python)
- **Memory Usage**: < 500MB (vs 1.5GB Python)
- **Parallel Builds**: 10 sets in < 10 seconds

### Code Quality

- **Warnings**: 0 (from 100)
- **Test Coverage**: > 80%
- **Documentation**: All public APIs documented

### User Adoption

- **Week 1**: 10 beta users testing
- **Week 2**: 100 users migrated
- **Month 1**: 50% of traffic on Rust version
- **Month 2**: 100% migrated

## Post-Launch Plan

### Week 1: Monitoring & Stabilization

- Monitor performance metrics
- Track error rates
- Collect user feedback
- Hot-fix critical issues

### Week 2-4: Optimization

- Profile and optimize hot paths
- Reduce memory allocations
- Improve caching strategy
- Add more providers

### Ongoing: Feature Parity

- Implement remaining stub providers
- Add missing edge cases
- Improve error messages
- Enhance documentation

## Version History

- **Version**: 1.0
- **Created**: 2025-01-18
- **Epic Type**: Critical Bug Fix & Infrastructure
- **Workstreams**: 5
- **Timeline**: 3-5 days
- **Budget**: $18,400
- **Priority**: CRITICAL - Blocks all development