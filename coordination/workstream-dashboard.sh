#!/bin/bash
# workstream-dashboard.sh - Interactive dashboard for monitoring parallel workstreams
# Usage: ./workstream-dashboard.sh

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
MAGENTA='\033[0;35m'
BOLD='\033[1m'
NC='\033[0m'

# Clear screen function
clear_screen() {
    clear
}

# Function to display header
show_header() {
    echo -e "${BOLD}${CYAN}╔══════════════════════════════════════════════════════════╗${NC}"
    echo -e "${BOLD}${CYAN}║     MTGJSON Rust Parallelization Dashboard               ║${NC}"
    echo -e "${BOLD}${CYAN}║     $(date '+%Y-%m-%d %H:%M:%S')                         ║${NC}"
    echo -e "${BOLD}${CYAN}╚══════════════════════════════════════════════════════════╝${NC}"
    echo ""
}

# Function to check compilation status
check_compilation() {
    echo -e "${BOLD}${YELLOW}📊 Compilation Status${NC}"
    echo "─────────────────────────"

    # Run cargo check and count errors
    CARGO_OUTPUT=$(cargo check 2>&1)
    ERROR_COUNT=$(echo "$CARGO_OUTPUT" | grep -c "error\[" || echo "0")
    WARNING_COUNT=$(echo "$CARGO_OUTPUT" | grep -c "warning:" || echo "0")

    if [ "$ERROR_COUNT" -eq "0" ]; then
        echo -e "${GREEN}✅ Compilation: SUCCESSFUL${NC}"
    else
        echo -e "${RED}❌ Compilation: FAILED${NC}"
        echo -e "   Errors: ${RED}$ERROR_COUNT${NC}"
    fi
    echo -e "   Warnings: ${YELLOW}$WARNING_COUNT${NC}"
    echo ""
}

# Function to display workstream status
show_workstream() {
    local ws_num=$1
    local branch=$2
    local description=$3
    local key_file=$4
    local status_color=$5

    echo -e "${BOLD}${status_color}WS$ws_num: $description${NC}"

    if git show-ref --verify --quiet refs/heads/$branch; then
        # Get commit info
        local last_commit=$(git log -1 --pretty=format:"%h %s" $branch 2>/dev/null | cut -c1-40)
        local commits_ahead=$(git rev-list --count main..$branch 2>/dev/null || echo "0")
        local last_modified=$(git log -1 --pretty=format:"%ar" $branch 2>/dev/null)

        echo "  📌 Branch: $branch"
        echo "  📝 Last: $last_commit"
        echo "  ⏱️  Modified: $last_modified"
        echo "  📊 Commits ahead: $commits_ahead"

        # Check key file if specified
        if [ ! -z "$key_file" ]; then
            if git show $branch:$key_file &>/dev/null; then
                local lines=$(git show $branch:$key_file 2>/dev/null | wc -l)
                echo -e "  ✅ Key file: $key_file (${GREEN}$lines lines${NC})"
            else
                echo -e "  ❌ Key file: $key_file (${RED}missing${NC})"
            fi
        fi

        # Determine status
        if [ "$commits_ahead" -gt "0" ]; then
            echo -e "  ${GREEN}▶ Status: ACTIVE${NC}"
        else
            echo -e "  ${YELLOW}⏸ Status: NOT STARTED${NC}"
        fi
    else
        echo -e "  ${RED}❌ Branch not created${NC}"
    fi
    echo ""
}

# Function to show progress bar
show_progress() {
    local current=$1
    local total=$2
    local width=30
    local percentage=$((current * 100 / total))
    local filled=$((current * width / total))

    echo -n "["
    for ((i=0; i<$filled; i++)); do
        echo -n "█"
    done
    for ((i=$filled; i<$width; i++)); do
        echo -n "─"
    done
    echo -n "] $percentage% ($current/$total)"
}

# Main dashboard loop
while true; do
    clear_screen
    show_header

    # Overall progress
    echo -e "${BOLD}${MAGENTA}🎯 Overall Progress${NC}"
    echo "─────────────────────────"

    # Count completed tasks (branches with commits)
    COMPLETED=0
    TOTAL=5

    for branch in feature/github-module feature/pyo3-migration feature/type-fixes feature/lifetime-fixes feature/high-level-api; do
        if git show-ref --verify --quiet refs/heads/$branch; then
            commits=$(git rev-list --count main..$branch 2>/dev/null || echo "0")
            if [ "$commits" -gt "0" ]; then
                COMPLETED=$((COMPLETED + 1))
            fi
        fi
    done

    echo -n "Workstreams: "
    show_progress $COMPLETED $TOTAL
    echo ""
    echo ""

    # Compilation status
    check_compilation

    # Workstream statuses
    echo -e "${BOLD}${BLUE}📋 Workstream Status${NC}"
    echo "═══════════════════════════════════════════"
    echo ""

    show_workstream 1 "feature/github-module" "GitHub Module" \
        "src/providers/github/mod.rs" "${GREEN}"

    show_workstream 2 "feature/pyo3-migration" "PyO3 Migration" \
        "src/providers/third_party/tcgplayer.rs" "${YELLOW}"

    show_workstream 3 "feature/type-fixes" "Type Fixes" \
        "src/builders/set_builder.rs" "${CYAN}"

    show_workstream 4 "feature/lifetime-fixes" "Lifetime Fixes" \
        "src/classes/foreign_data.rs" "${MAGENTA}"

    show_workstream 5 "feature/high-level-api" "High-Level API" \
        "src/lib.rs" "${BLUE}"

    # Merge conflicts check
    echo -e "${BOLD}${YELLOW}⚠️  Potential Conflicts${NC}"
    echo "─────────────────────────"

    CONFLICTS_FOUND=false

    # Check for parallel_call.rs conflicts
    if git show feature/pyo3-migration:src/builders/parallel_call.rs &>/dev/null && \
       git show feature/type-fixes:src/builders/parallel_call.rs &>/dev/null; then
        echo -e "${YELLOW}• parallel_call.rs modified in WS2 and WS3${NC}"
        CONFLICTS_FOUND=true
    fi

    # Check for foreign_data.rs conflicts
    if git show feature/pyo3-migration:src/classes/foreign_data.rs &>/dev/null && \
       git show feature/lifetime-fixes:src/classes/foreign_data.rs &>/dev/null; then
        echo -e "${YELLOW}• foreign_data.rs modified in WS2 and WS4${NC}"
        CONFLICTS_FOUND=true
    fi

    if [ "$CONFLICTS_FOUND" = false ]; then
        echo -e "${GREEN}✅ No conflicts detected${NC}"
    fi
    echo ""

    # Commands menu
    echo -e "${BOLD}${CYAN}📌 Quick Commands${NC}"
    echo "─────────────────────────"
    echo "  [1] Sync all branches with main"
    echo "  [2] Run merge process"
    echo "  [3] Show detailed errors"
    echo "  [4] Create missing branches"
    echo "  [5] Generate status report"
    echo "  [R] Refresh"
    echo "  [Q] Quit"
    echo ""

    # Read user input with timeout
    read -t 10 -n 1 -p "Select option (auto-refresh in 10s): " choice
    echo ""

    case $choice in
        1)
            echo "Syncing all branches with main..."
            for branch in feature/github-module feature/pyo3-migration feature/type-fixes feature/lifetime-fixes; do
                git checkout $branch 2>/dev/null && git merge main --no-edit 2>/dev/null
            done
            git checkout main 2>/dev/null
            read -p "Press any key to continue..."
            ;;
        2)
            ./merge-workstreams.sh
            read -p "Press any key to continue..."
            ;;
        3)
            echo "Compilation errors:"
            cargo check 2>&1 | grep "error\[" | head -20
            read -p "Press any key to continue..."
            ;;
        4)
            ./create-workstream-branches.sh
            read -p "Press any key to continue..."
            ;;
        5)
            echo "Generating status report..."
            ./sync-status.sh > status-report-$(date +%Y%m%d-%H%M%S).txt
            echo "Report saved."
            read -p "Press any key to continue..."
            ;;
        [Qq])
            echo "Exiting dashboard..."
            exit 0
            ;;
        [Rr]|"")
            # Refresh - loop will continue
            ;;
        *)
            echo "Invalid option"
            sleep 1
            ;;
    esac
done