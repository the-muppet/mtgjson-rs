#!/bin/bash
# create-workstream-branches.sh - Initialize all parallel workstream branches
# Usage: ./create-workstream-branches.sh

echo "============================================="
echo "MTGJSON Rust Workstream Branch Initialization"
echo "============================================="
echo ""

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Save current branch
ORIGINAL_BRANCH=$(git branch --show-current)

echo -e "${BLUE}Current branch:${NC} $ORIGINAL_BRANCH"
echo ""

# Function to create a workstream branch
create_branch() {
    local branch=$1
    local description=$2
    local files=$3

    echo -e "${YELLOW}Creating $branch${NC}"
    echo "  Purpose: $description"

    # Check if branch already exists
    if git show-ref --verify --quiet refs/heads/$branch; then
        echo -e "  ${YELLOW}⚠ Branch already exists, skipping${NC}"
    else
        # Create and switch to branch
        if git checkout -b $branch; then
            echo -e "  ${GREEN}✓ Branch created${NC}"

            # Create placeholder commit if files specified
            if [ ! -z "$files" ]; then
                echo "  Creating placeholder files..."
                for file in $files; do
                    mkdir -p $(dirname $file)
                    echo "// TODO: Implement $description" > $file
                    git add $file
                done
                git commit -m "Initial commit for $description" --allow-empty
                echo -e "  ${GREEN}✓ Initial commit created${NC}"
            fi

            # Return to original branch
            git checkout $ORIGINAL_BRANCH --quiet
        else
            echo -e "  ${RED}✗ Failed to create branch${NC}"
        fi
    fi
    echo ""
}

echo "This script will create the following workstream branches:"
echo "  - feature/github (WS1: GitHub providers)"
echo "  - feature/pyo3 (WS2: PyO3 v0.22 updates)"
echo "  - feature/type-fix (WS3: Type signature fixes)"
echo "  - feature/life-fix (WS4: Lifetime fixes)"
echo "  - feature/api-cli (WS5: Python API functions)"
echo ""
read -p "Continue? (y/n) " -n 1 -r
echo ""
echo ""

if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Branch creation cancelled."
    exit 1
fi

# Create each workstream branch
echo "=== Creating Workstream Branches ==="
echo ""

# WS1: GitHub Module
create_branch "feature/gh" \
    "WS1: GitHub Module Implementation" \
    ""

# WS2: PyO3 Migration
create_branch "feature/pyo3" \
    "WS2: PyO3 API Migration to v0.22" \
    ""

# WS3: Type Fixes
create_branch "feature/type-fix" \
    "WS3: Type & Method Signature Fixes" \
    ""

# WS4: Lifetime Fixes
create_branch "feature/life-fix" \
    "WS4: Lifetime & Memory Safety Fixes" \
    ""

# WS5: High-Level API (created but work starts after merge)
create_branch "feature/api-cli" \
    "WS5: High-Level Python API Implementation" \
    ""

echo "=== Branch Creation Complete ==="
echo ""

# Show all branches
echo "Current branch structure:"
git branch -a | grep -E "feature/|main|master" | sed 's/^/  /'
echo ""

# Provide next steps based on parallelization strategy
echo "=== Next Steps for Parallel Development ==="
echo ""
echo -e "${BLUE}For Multiple Developers:${NC}"
echo "  Developer 1: git checkout feature/github-module"
echo "             Work on src/providers/github/* module"
echo ""
echo "  Developer 2: git checkout feature/pyo3-migration"
echo "             Update PyO3 API calls across 20+ files"
echo ""
echo "  Developer 3: git checkout feature/type-fixes"
echo "             Fix src/builders/set_builder.rs"
echo "             Fix src/builders/parallel_call.rs"
echo ""
echo "  Developer 4: git checkout feature/lifetime-fixes"
echo "             Fix src/classes/foreign_data.rs"
echo ""
echo -e "${BLUE}For Solo Developer:${NC}"
echo "  Work on branches sequentially or in rotation:"
echo "  1. git checkout feature/github-module"
echo "  2. Implement GitHub providers"
echo "  3. Commit and switch to next branch"
echo "  4. Repeat for other workstreams"
echo ""
echo "When all workstreams complete:"
echo "  Run: ./merge-workstreams.sh"
echo ""

# Return to original branch
echo -e "Returning to ${BLUE}$ORIGINAL_BRANCH${NC}"
git checkout $ORIGINAL_BRANCH --quiet

