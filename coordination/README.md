# MTGJSON Rust Parallelization Coordination Tools

This directory contains scripts to coordinate parallel development of the MTGJSON Rust compilation fixes across multiple workstreams.

## Overview

The parallelization strategy allows fixing 31 compilation errors in parallel across 5 workstreams, reducing time from 5 days (sequential) to 3 days (parallel).

## Scripts

### 1. create-workstream-branches.sh
**Purpose**: Initialize all parallel workstream branches

```bash
./create-workstream-branches.sh
```

Creates these feature branches:
- `feature/github-module` - WS1: GitHub provider implementation
- `feature/pyo3-migration` - WS2: PyO3 v0.22 API updates
- `feature/type-fixes` - WS3: Type signature fixes
- `feature/lifetime-fixes` - WS4: Lifetime issue fixes
- `feature/high-level-api` - WS5: Python API functions

### 2. sync-status.sh
**Purpose**: Check status of all workstreams

```bash
./sync-status.sh
```

Shows:
- Current branch status for each workstream
- Number of commits ahead of main
- Key file existence checks
- Current compilation error count
- Potential merge conflicts

### 3. workstream-dashboard.sh
**Purpose**: Interactive real-time dashboard

```bash
./workstream-dashboard.sh
```

Features:
- Auto-refreshing status display (10 second intervals)
- Overall progress tracking
- Compilation status monitoring
- Conflict detection
- Quick commands for common operations

Dashboard Commands:
- `1` - Sync all branches with main
- `2` - Run merge process
- `3` - Show detailed compilation errors
- `4` - Create missing branches
- `5` - Generate status report
- `R` - Manual refresh
- `Q` - Quit

### 4. merge-workstreams.sh
**Purpose**: Merge all completed workstreams into main

```bash
./merge-workstreams.sh
```

Process:
1. Creates backup tag before merge
2. Merges branches in order (WS1 → WS2 → WS3 → WS4 → WS5)
3. Handles conflicts interactively
4. Tests compilation after merge
5. Reports success/failure

## Workflow

### For Multiple Developers (Optimal)

1. **Setup Phase**:
```bash
# Each developer runs on their machine
git pull origin main
./create-workstream-branches.sh
```

2. **Development Phase**:
```bash
# Developer 1
git checkout feature/github-module
# Implement GitHub providers in src/providers/github/

# Developer 2
git checkout feature/pyo3-migration
# Update PyO3 API calls across codebase

# Developer 3
git checkout feature/type-fixes
# Fix type issues in builders

# Developer 4
git checkout feature/lifetime-fixes
# Fix lifetime issues in foreign_data.rs
```

3. **Coordination Phase**:
```bash
# Team lead monitors progress
./workstream-dashboard.sh  # Keep running for live updates

# Check status periodically
./sync-status.sh > status-$(date +%Y%m%d).txt
```

4. **Merge Phase**:
```bash
# When all workstreams complete
./merge-workstreams.sh
cargo build  # Should succeed!
```

### For Solo Developer

1. **Setup**:
```bash
./create-workstream-branches.sh
```

2. **Parallel Development** (using branch switching):
```bash
# Work on WS1 (2 hours)
git checkout feature/github-module
# Create GitHub module files
git add . && git commit -m "Implement GitHub providers"

# Switch to WS2 (1.5 hours)
git checkout feature/pyo3-migration
# Fix PyO3 API calls in first batch of files
git add . && git commit -m "Migrate PyO3 APIs part 1"

# Switch to WS3 (1 hour)
git checkout feature/type-fixes
# Fix type issues
git add . && git commit -m "Fix type signatures"

# Back to WS2 (1.5 hours)
git checkout feature/pyo3-migration
# Finish remaining PyO3 updates
git add . && git commit -m "Complete PyO3 migration"

# Work on WS4 (1 hour)
git checkout feature/lifetime-fixes
# Fix lifetime issues
git add . && git commit -m "Fix lifetime issues"
```

3. **Monitor Progress**:
```bash
# Check overall status
./sync-status.sh

# Or use interactive dashboard
./workstream-dashboard.sh
```

4. **Merge When Complete**:
```bash
./merge-workstreams.sh
```

## Workstream Details

### WS1: GitHub Module (8 hours)
- **Files**: Create 6 new files in src/providers/github/
- **Template**: Follow EdhrecProviderCardRanks pattern
- **No conflicts**: Completely isolated work

### WS2: PyO3 Migration (12 hours)
- **Files**: Update 20+ files
- **Pattern**: Replace old PyO3 API with v0.22 methods
- **Conflicts**: May touch parallel_call.rs (WS3) and foreign_data.rs (WS4)

### WS3: Type Fixes (6 hours)
- **Files**: src/builders/set_builder.rs, parallel_call.rs
- **Pattern**: Add missing Python context arguments
- **Conflicts**: May conflict with WS2 in parallel_call.rs

### WS4: Lifetime Fixes (4 hours)
- **Files**: src/classes/foreign_data.rs
- **Pattern**: Fix Bound<'_, PyDict> lifetimes
- **Conflicts**: May conflict with WS2 in same file

### WS5: High-Level API (8 hours)
- **Files**: src/lib.rs
- **Pattern**: Add Python entry point functions
- **Dependencies**: Must wait for WS1-4 to complete

## Conflict Resolution

### Expected Conflicts

Two files may have conflicts when merging:

1. **src/builders/parallel_call.rs**
   - Modified by WS2 (PyO3 updates)
   - Modified by WS3 (type fixes)
   - Resolution: Keep both sets of changes

2. **src/classes/foreign_data.rs**
   - Modified by WS2 (PyO3 updates)
   - Modified by WS4 (lifetime fixes)
   - Resolution: Keep both sets of changes

### Resolution Process

If conflicts occur during merge:

```bash
# 1. View conflicts
git status

# 2. Open conflicted file and resolve
# Look for <<<<<<< HEAD markers

# 3. After resolving
git add <resolved-file>
git merge --continue

# 4. Continue merge script
./merge-workstreams.sh
```

## Success Metrics

Track progress with these metrics:

- **Compilation Errors**: Start with 31, target 0
- **Warnings**: Start with 100, target 0 (Phase 2)
- **Time to Compile**: Should succeed by Day 2 EOD
- **Test Coverage**: 80% minimum for new code

## Troubleshooting

### Branch Creation Issues
```bash
# If branches already exist
git branch -D feature/github-module  # Delete and recreate
./create-workstream-branches.sh
```

### Merge Conflicts
```bash
# Abort merge if needed
git merge --abort

# Return to clean state
git checkout main
git reset --hard origin/main
```

### Dashboard Not Refreshing
```bash
# Check if git operations are slow
git gc  # Garbage collect
git fetch --prune  # Clean remote references
```

### Can't Find Scripts
```bash
# Make scripts executable
chmod +x *.sh

# Run from coordination directory
cd coordination/
./sync-status.sh
```

## Tips for Success

1. **Frequent Commits**: Commit often to track progress
2. **Clear Messages**: Use descriptive commit messages
3. **Test Locally**: Run `cargo check` before committing
4. **Communicate**: If multiple devs, coordinate on shared files
5. **Monitor Dashboard**: Keep dashboard running for awareness

## Post-Merge Steps

After successful merge and compilation:

1. **Run Full Test Suite**:
```bash
cargo test --all
```

2. **Check Python Integration**:
```python
import mtgjson_rust
result = mtgjson_rust.build_single_set('NEO', True)
assert len(result['cards']) > 0
```

3. **Benchmark Performance**:
```bash
cargo bench
```

4. **Clean Up Branches**:
```bash
# Delete local branches
git branch -d feature/github-module
git branch -d feature/pyo3-migration
# ... etc

# Delete remote branches
git push origin --delete feature/github-module
# ... etc
```

## Support

For issues or questions:
1. Check compilation errors: `cargo build 2>&1 | head -20`
2. Review the EPIC.md for detailed specifications
3. Consult PARALLELIZATION_STRATEGY.md for context
4. Use `git log --oneline --graph` to visualize branch history