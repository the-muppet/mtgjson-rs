#!/bin/bash
# merge-workstreams.sh - Merge all parallel workstreams into main
# Usage: ./merge-workstreams.sh

echo "========================================"
echo "MTGJSON Rust Workstream Merge Process"
echo "========================================"
echo ""

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Save current branch
ORIGINAL_BRANCH=$(git branch --show-current)

# Function to safely merge a branch
merge_branch() {
    local branch=$1
    local description=$2

    echo -e "${YELLOW}Merging $description${NC}"

    if git show-ref --verify --quiet refs/heads/$branch; then
        echo "  Attempting to merge $branch..."

        # Try to merge
        if git merge --no-ff $branch -m "Merge $branch: $description"; then
            echo -e "  ${GREEN}✓ Successfully merged $branch${NC}"
            return 0
        else
            echo -e "  ${RED}✗ Merge conflict in $branch${NC}"
            echo "  Conflicts detected. Please resolve manually."
            return 1
        fi
    else
        echo -e "  ${YELLOW}⚠ Branch $branch not found, skipping${NC}"
        return 0
    fi
}

# Confirmation prompt
echo "This script will merge all workstream branches into main."
echo "Branches to merge:"
echo "  - feature/github-module (WS1)"
echo "  - feature/pyo3-migration (WS2)"
echo "  - feature/type-fixes (WS3)"
echo "  - feature/lifetime-fixes (WS4)"
echo "  - feature/high-level-api (WS5)"
echo ""
read -p "Continue? (y/n) " -n 1 -r
echo ""

if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Merge cancelled."
    exit 1
fi

# Create backup tag
echo ""
echo "Creating backup tag before merge..."
git tag -a "pre-merge-$(date +%Y%m%d-%H%M%S)" -m "Backup before workstream merge"
echo -e "${GREEN}✓ Backup tag created${NC}"

# Switch to main branch
echo ""
echo "Switching to main branch..."
git checkout main

# Merge each workstream in order
echo ""
echo "=== Starting Merge Process ==="
echo ""

MERGE_FAILED=0

# WS1: GitHub Module (no conflicts expected)
if ! merge_branch "feature/github-module" "WS1: GitHub Module Implementation"; then
    MERGE_FAILED=1
fi
echo ""

# WS2: PyO3 Migration (may conflict with WS3/WS4)
if [ $MERGE_FAILED -eq 0 ]; then
    if ! merge_branch "feature/pyo3-migration" "WS2: PyO3 API Migration"; then
        MERGE_FAILED=1
    fi
    echo ""
fi

# WS3: Type Fixes (may conflict with WS2)
if [ $MERGE_FAILED -eq 0 ]; then
    if ! merge_branch "feature/type-fixes" "WS3: Type & Method Signature Fixes"; then
        MERGE_FAILED=1
    fi
    echo ""
fi

# WS4: Lifetime Fixes (may conflict with WS2)
if [ $MERGE_FAILED -eq 0 ]; then
    if ! merge_branch "feature/lifetime-fixes" "WS4: Lifetime & Memory Safety Fixes"; then
        MERGE_FAILED=1
    fi
    echo ""
fi

# WS5: High-Level API (depends on compilation)
if [ $MERGE_FAILED -eq 0 ]; then
    if ! merge_branch "feature/high-level-api" "WS5: High-Level API Implementation"; then
        MERGE_FAILED=1
    fi
    echo ""
fi

echo "=== Merge Process Complete ==="
echo ""

# Handle merge conflicts
if [ $MERGE_FAILED -eq 1 ]; then
    echo -e "${RED}✗ Merge conflicts detected${NC}"
    echo ""
    echo "To resolve conflicts:"
    echo "  1. Fix conflicts in the marked files"
    echo "  2. Run: git add <resolved-files>"
    echo "  3. Run: git merge --continue"
    echo "  4. Continue with remaining merges using this script"
    echo ""
    echo "To abort and restore:"
    echo "  git merge --abort"
    echo "  git checkout $ORIGINAL_BRANCH"
    exit 1
fi

# Test compilation
echo "=== Testing Compilation ==="
echo ""
echo "Running cargo build..."

if cargo build 2>&1 | tee /tmp/cargo_build.log; then
    echo ""
    echo -e "${GREEN}✅ SUCCESS! Compilation successful!${NC}"
    echo ""

    # Count warnings
    WARNING_COUNT=$(grep -c "warning:" /tmp/cargo_build.log || true)
    echo "Warnings remaining: $WARNING_COUNT"

    # Run tests if compilation succeeded
    echo ""
    echo "Running tests..."
    if cargo test --quiet; then
        echo -e "${GREEN}✓ All tests passing${NC}"
    else
        echo -e "${YELLOW}⚠ Some tests failing${NC}"
    fi
else
    echo ""
    echo -e "${RED}✗ Compilation still failing${NC}"
    ERROR_COUNT=$(grep -c "error\[" /tmp/cargo_build.log || true)
    echo "Errors remaining: $ERROR_COUNT"
    echo ""
    echo "Review errors with: cargo build 2>&1 | less"
fi

echo ""
echo "=== Summary ==="
echo ""
echo "All workstreams merged into main."
echo "Original branch was: $ORIGINAL_BRANCH"
echo ""
echo "Next steps:"
echo "  1. If compilation succeeded: Proceed with testing"
echo "  2. If compilation failed: Fix remaining errors"
echo "  3. Create PR when ready: git push origin main"
echo ""

# Offer to return to original branch
read -p "Return to original branch? (y/n) " -n 1 -r
echo ""
if [[ $REPLY =~ ^[Yy]$ ]]; then
    git checkout $ORIGINAL_BRANCH
fi