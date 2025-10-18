#!/bin/bash
# sync-status.sh - Check status of all parallel workstreams
# Usage: ./sync-status.sh

echo "=========================================="
echo "MTGJSON Rust Parallelization Status Check"
echo "=========================================="
echo ""

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Check current branch
CURRENT_BRANCH=$(git branch --show-current)
echo -e "${BLUE}Current Branch:${NC} $CURRENT_BRANCH"
echo ""

# Function to check workstream status
check_workstream() {
    local branch=$1
    local description=$2
    local files=$3

    echo -e "${YELLOW}$description${NC}"

    # Check if branch exists
    if git show-ref --verify --quiet refs/heads/$branch; then
        # Get last commit
        LAST_COMMIT=$(git log -1 --oneline $branch 2>/dev/null)
        echo "  Branch: $branch"
        echo "  Last Commit: $LAST_COMMIT"

        # Count commits ahead of main
        COMMITS_AHEAD=$(git rev-list --count main..$branch 2>/dev/null)
        echo "  Commits ahead of main: $COMMITS_AHEAD"

        # Check specific files if provided
        if [ ! -z "$files" ]; then
            echo "  Key files status:"
            for file in $files; do
                if git show $branch:$file &>/dev/null; then
                    echo -e "    ✓ ${GREEN}$file exists${NC}"
                else
                    echo -e "    ✗ ${RED}$file missing${NC}"
                fi
            done
        fi
    else
        echo -e "  ${RED}Branch not found${NC}"
    fi
    echo ""
}

echo "=== Workstream Status ==="
echo ""

# Check each workstream
check_workstream "feature/github-module" \
    "WS1: GitHub Module Implementation" \
    "src/providers/github/mod.rs src/providers/github/boosters.rs"

check_workstream "feature/pyo3-migration" \
    "WS2: PyO3 API Migration" \
    "src/providers/third_party/tcgplayer.rs"

check_workstream "feature/type-fixes" \
    "WS3: Type & Method Signature Fixes" \
    "src/builders/set_builder.rs"

check_workstream "feature/lifetime-fixes" \
    "WS4: Lifetime & Memory Safety Fixes" \
    "src/classes/foreign_data.rs"

check_workstream "feature/high-level-api" \
    "WS5: High-Level API Implementation" \
    "src/lib.rs"

echo "=== Compilation Status ==="
echo ""

# Try to build and capture errors
echo "Running cargo check..."
CARGO_OUTPUT=$(cargo check 2>&1)
ERROR_COUNT=$(echo "$CARGO_OUTPUT" | grep -c "error\[")

if [ $ERROR_COUNT -eq 0 ]; then
    echo -e "${GREEN}✓ Compilation successful!${NC}"
else
    echo -e "${RED}✗ Compilation failed with $ERROR_COUNT errors${NC}"
    echo ""
    echo "Top 5 errors:"
    echo "$CARGO_OUTPUT" | grep "error\[" | head -5
fi

echo ""
echo "=== Merge Readiness ==="
echo ""

# Check for potential conflicts
echo "Checking for potential merge conflicts..."

CONFLICTS=0

# Check if parallel_call.rs modified in multiple branches
if git show feature/pyo3-migration:src/builders/parallel_call.rs &>/dev/null && \
   git show feature/type-fixes:src/builders/parallel_call.rs &>/dev/null; then
    echo -e "${YELLOW}⚠ Warning: parallel_call.rs modified in both WS2 and WS3${NC}"
    CONFLICTS=$((CONFLICTS + 1))
fi

# Check if foreign_data.rs modified in multiple branches
if git show feature/pyo3-migration:src/classes/foreign_data.rs &>/dev/null && \
   git show feature/lifetime-fixes:src/classes/foreign_data.rs &>/dev/null; then
    echo -e "${YELLOW}⚠ Warning: foreign_data.rs modified in both WS2 and WS4${NC}"
    CONFLICTS=$((CONFLICTS + 1))
fi

if [ $CONFLICTS -eq 0 ]; then
    echo -e "${GREEN}✓ No potential conflicts detected${NC}"
fi

echo ""
echo "=========================================="
echo "Use ./merge-workstreams.sh when ready to merge all branches"
echo "=========================================="