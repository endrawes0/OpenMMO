#!/bin/bash
# OpenMMO Login/Menu System Test Script
# Tests the complete authentication and character management flow

set -e

echo "🧪 OpenMMO Login/Menu System Test"
echo "=================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Test counter
TESTS_RUN=0
TESTS_PASSED=0

run_test() {
    local test_name="$1"
    local test_cmd="$2"

    echo -n "Running: $test_name... "
    TESTS_RUN=$((TESTS_RUN + 1))

    if eval "$test_cmd" >/dev/null 2>&1; then
        echo -e "${GREEN}✓ PASSED${NC}"
        TESTS_PASSED=$((TESTS_PASSED + 1))
    else
        echo -e "${RED}✗ FAILED${NC}"
    fi
}

# Test 1: Server health
echo "1. Testing Server Components"
run_test "Server health check" "curl -s http://localhost:8080/health | grep -q 'healthy'"
run_test "Database connectivity" "curl -s http://localhost:8080/health/db | grep -q 'connected'"

# Test 2: Client modules
echo -e "\n2. Testing Client Modules"
run_test "Client networking module" "cd /home/endrawes/opencode/OpenMMO && godot --script client/test_modules.gd 2>/dev/null | grep -q 'client_networking loaded successfully'"
run_test "Game state manager module" "cd /home/endrawes/opencode/OpenMMO && godot --script client/test_modules.gd 2>/dev/null | grep -q 'game_state_manager loaded successfully'"
run_test "Movement system module" "cd /home/endrawes/opencode/OpenMMO && godot --script client/test_modules.gd 2>/dev/null | grep -q 'movement_system loaded successfully'"
run_test "Input manager module" "cd /home/endrawes/opencode/OpenMMO && godot --script client/test_modules.gd 2>/dev/null | grep -q 'input_manager loaded successfully'"
run_test "UI state manager module" "cd /home/endrawes/opencode/OpenMMO && godot --script client/test_modules.gd 2>/dev/null | grep -q 'ui_state_manager loaded successfully'"

# Test 3: WebSocket connectivity
echo -e "\n3. Testing Network Connectivity"
run_test "WebSocket handshake" "python3 -c \"
import asyncio
import websockets
import json

async def test():
    try:
        async with websockets.connect('ws://localhost:8080/ws') as ws:
            msg = {'sequence_id': 1, 'timestamp': 1234567890, 'payload': {'HandshakeRequest': {'client_version': '1.0.0', 'protocol_version': '1.0', 'supported_features': 0}}}
            await ws.send(json.dumps(msg))
            resp = await ws.recv()
            data = json.loads(resp)
            if 'HandshakeResponse' in str(data):
                return True
    except:
        pass
    return False

result = asyncio.run(test())
exit(0 if result else 1)
\""

# Test 4: Authentication messages
echo -e "\n4. Testing Authentication Flow"
run_test "Login message format" "python3 -c \"
import asyncio
import websockets
import json

async def test():
    try:
        async with websockets.connect('ws://localhost:8080/ws') as ws:
            # Handshake first
            hs = {'sequence_id': 1, 'timestamp': 1234567890, 'payload': {'HandshakeRequest': {'client_version': '1.0.0', 'protocol_version': '1.0', 'supported_features': 0}}}
            await ws.send(json.dumps(hs))
            await ws.recv()
            
            # Login
            login = {'sequence_id': 2, 'timestamp': 1234567890, 'payload': {'AuthRequest': {'username': 'testuser', 'password_hash': 'testpass', 'character_name': None}}}
            await ws.send(json.dumps(login))
            resp = await ws.recv()
            data = json.loads(resp)
            if 'AuthResponse' in str(data):
                return True
    except:
        pass
    return False

result = asyncio.run(test())
exit(0 if result else 1)
\""

# Test 5: Character management
echo -e "\n5. Testing Character Management"
run_test "Character list request" "python3 -c \"
import asyncio
import websockets
import json

async def test():
    try:
        async with websockets.connect('ws://localhost:8080/ws') as ws:
            # Handshake
            hs = {'sequence_id': 1, 'timestamp': 1234567890, 'payload': {'HandshakeRequest': {'client_version': '1.0.0', 'protocol_version': '1.0', 'supported_features': 0}}}
            await ws.send(json.dumps(hs))
            await ws.recv()
            
            # Login
            login = {'sequence_id': 2, 'timestamp': 1234567890, 'payload': {'AuthRequest': {'username': 'testuser', 'password_hash': 'testpass', 'character_name': None}}}
            await ws.send(json.dumps(login))
            await ws.recv()
            
            # Request character list
            char_list = {'sequence_id': 3, 'timestamp': 1234567890, 'payload': {'CharacterListRequest': {}}}
            await ws.send(json.dumps(char_list))
            resp = await ws.recv()
            data = json.loads(resp)
            if 'CharacterListResponse' in str(data):
                return True
    except:
        pass
    return False

result = asyncio.run(test())
exit(0 if result else 1)
\""

# Test 6: UI scene validation
echo -e "\n6. Testing UI Components"
run_test "Main scene loads" "cd /home/endrawes/opencode/OpenMMO && timeout 5 godot --path client --scene Main.tscn --headless --quit-after 1 2>/dev/null && echo 'Scene loaded' || echo 'Scene failed' | grep -q 'Scene loaded'"

# Summary
echo -e "\n" "=================================="
echo -e "Test Results: ${TESTS_PASSED}/${TESTS_RUN} tests passed"

if [ $TESTS_PASSED -eq $TESTS_RUN ]; then
    echo -e "${GREEN}🎉 All tests passed! Login/menu system is working correctly.${NC}"
    exit 0
else
    echo -e "${RED}❌ Some tests failed. Check the output above for details.${NC}"
    exit 1
fi