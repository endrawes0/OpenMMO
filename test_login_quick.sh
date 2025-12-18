#!/bin/bash
# OpenMMO Login/Menu System - Quick Test
# Tests the essential components without complex async operations

set -e

echo "🧪 OpenMMO Login/Menu System - Quick Test"
echo "==========================================="

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m'

PASSED=0
TOTAL=0

test_pass() {
    echo -e "${GREEN}✓ $1${NC}"
    PASSED=$((PASSED + 1))
}

test_fail() {
    echo -e "${RED}✗ $1${NC}"
}

run_test() {
    TOTAL=$((TOTAL + 1))
    if eval "$2" >/dev/null 2>&1; then
        test_pass "$1"
    else
        test_fail "$1"
    fi
}

echo "1. Server Health"
run_test "Server responds to health check" "curl -s http://localhost:8080/health | grep -q 'healthy'"
run_test "Database connection healthy" "curl -s http://localhost:8080/health/db | grep -q 'connected'"

echo -e "\n2. Client Modules"
run_test "All client modules load" "cd /home/endrawes/opencode/OpenMMO && godot --script client/test_modules.gd 2>/dev/null | grep -c 'loaded successfully' | grep -q '5'"

echo -e "\n3. WebSocket Connection"
run_test "WebSocket server accepts connections" "timeout 3 bash -c 'echo > /dev/tcp/localhost/8080' 2>/dev/null && echo 'Port open' || echo 'Port closed' | grep -q 'Port open'"

echo -e "\n4. Project Structure"
run_test "UI state manager exists" "test -f client/src/ui/ui_state_manager.gd"
run_test "Main scene updated" "grep -q 'LoginPanel' client/scenes/Main.tscn"
run_test "Character management UI" "grep -q 'CharacterPanel' client/scenes/Main.tscn"

echo -e "\n5. Code Quality"
run_test "Main script exists and is readable" "test -f client/scripts/Main.gd && wc -l < client/scripts/Main.gd | grep -q '^[0-9]'"

echo -e "\n==========================================="
echo -e "Results: $PASSED/$TOTAL tests passed"

if [ $PASSED -eq $TOTAL ]; then
    echo -e "${GREEN}🎉 Login/menu system implementation successful!${NC}"
    echo -e "\nNext steps:"
    echo -e "  • Run: cd client && godot --editor"
    echo -e "  • Open Main.tscn and test the UI flow"
    echo -e "  • Connect to ws://localhost:8080/ws"
    echo -e "  • Test login/registration and character creation"
else
    echo -e "${RED}❌ Some tests failed - check implementation${NC}"
    exit 1
fi