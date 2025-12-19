# OpenMMO Testing Instructions

This document provides comprehensive testing instructions for the OpenMMO server and client components.

## Prerequisites

### System Requirements
- Rust 1.70+ with Cargo
- Docker and Docker Compose
- Godot 4.x engine
- PostgreSQL client tools (optional)

### Environment Setup
1. **Clone and setup the repository:**
   ```bash
   git clone <repository-url>
   cd OpenMMO
   cp .env.example .env
   ```

2. **Install dependencies:**
   ```bash
   # Automated setup (recommended)
   ./setup.sh

   # Or manual setup
   docker-compose up -d db
   cd server && cargo sqlx migrate run
   ```

## 1. Server Testing

### 1.1 Basic Server Startup
```bash
# Start the database
docker-compose up -d db

# Build and run server
cd server
cargo build --release
DATABASE_URL=postgres://openmmo_user:openmmo_dev_password@localhost:5432/openmmo \
SERVER_HOST=0.0.0.0 \
SERVER_PORT=8080 \
RUST_LOG=info \
cargo run --bin server
```

**Expected Output:**
- Server starts without errors
- Logs show: "OpenMMO server listening on 0.0.0.0:8080"
- No migration errors

### 1.2 Health Endpoint Testing
```bash
# Test server health
curl http://localhost:8080/health

# Test database connectivity
curl http://localhost:8080/health/db
```

**Expected Response:**
```json
{"status":"healthy","timestamp":"2025-12-16TXX:XX:XX.XXXZ"}
```

### 1.3 WebSocket Connection Testing
```bash
# Test WebSocket handshake (requires websocat or similar)
websocat -c ws://localhost:8080/ws --no-color -x '{
  "sequence_id": 1,
  "timestamp": 1234567890,
  "payload": {
    "HandshakeRequest": {
      "client_version": "1.0.0",
      "protocol_version": "1.0",
      "supported_features": 0
    }
  }
}'
```

**Expected Response:**
```json
{
  "sequence_id": 1,
  "timestamp": 1765896XXXXXXX,
  "payload": {
    "HandshakeResponse": {
      "accepted": true,
      "server_version": "0.1.0",
      "protocol_version": "1.0",
      "server_features": 0,
      "message": "Welcome to OpenMMO!"
    }
  }
}
```

## 2. Client Testing

### 2.1 Module Loading Test
```bash
# Create test script
cat > client/test_modules.gd << 'EOF'
extends SceneTree

func _init():
    print("Testing OpenMMO client modules...")

    # Test loading modules
    var modules = [
        "res://src/networking/client_networking.gd",
        "res://src/gamestate/game_state_manager.gd",
        "res://src/movement/movement_system.gd",
        "res://src/input/input_manager.gd",
        "res://src/ui/ui_state_manager.gd"
    ]

    for path in modules:
        var script = load(path)
        if script:
            var instance = script.new()
            print("✓ " + path.get_file().get_basename() + " loaded successfully")
        else:
            print("✗ Failed to load " + path)

    print("Module loading test completed")
    quit()
EOF

# Run test
cd client
godot --script test_modules.gd
```

**Expected Output:**
```
Testing OpenMMO client modules...
✓ client_networking loaded successfully
✓ game_state_manager loaded successfully
✓ movement_system loaded successfully
✓ input_manager loaded successfully
✓ ui_state_manager loaded successfully
Module loading test completed
```

> Note: `res://` resolves to the `client/` project root. Run the script from inside `client/` (as shown) so the `res://src/...` paths load correctly.

### 2.2 Client UI Testing
```bash
# Open Godot editor
cd client
godot --editor

# In Godot editor:
# 1. Open Main.tscn scene
# 2. Run the scene (F5)
# 3. Verify UI elements are present
# 4. Test connection form validation
```

**UI Elements to Verify:**
- Server address input field
- Username input field
- Password input field
- Connect button
- Exit button
- Network debug panel

### 2.3 Client Networking Test
```bash
# In Godot editor, run Main.tscn
# Fill in connection details:
# - Server: ws://localhost:8080/ws
# - Username: testuser
# - Password: testpass
# Click Connect button
```

**Expected Behavior:**
- Status changes to "Connecting..."
- Status changes to "Connected" (if server is running)
- Debug panel shows connection messages
- Ping messages sent every 5 seconds

## 3. Integration Testing

### 3.1 Full Client-Server Connection
```bash
# Terminal 1: Start server
cd server
DATABASE_URL=postgres://openmmo_user:openmmo_dev_password@localhost:5432/openmmo \
SERVER_HOST=0.0.0.0 \
SERVER_PORT=8080 \
RUST_LOG=debug \
cargo run --bin server

# Terminal 2: Test client connection
cd client
godot --editor
# Open Main.tscn and run it
# Connect to ws://localhost:8080/ws
```

**Expected Behavior:**
- Client connects successfully
- Server logs show new WebSocket connection
- Handshake completes
- Ping/pong messages work
- No connection errors

### 3.2 Movement System Testing
```bash
# In Godot editor, open GameWorld.tscn
# Run the scene
# Test input handling:
# - WASD keys for movement
# - Mouse movement for camera
# - ESC key to return to menu
```

**Expected Behavior:**
- Input manager captures keyboard/mouse input
- Movement system processes input vectors
- No errors in console
- Camera follows mouse movement

### 3.3 Message Protocol Testing
```bash
# Create comprehensive test script
cat > client/test_protocol.gd << 'EOF'
extends SceneTree

func _init():
    print("Testing message protocol...")

    # Mirror client_networking.gd message envelope creation without opening a socket
    func build_envelope(payload: Dictionary, sequence_id: int) -> Dictionary:
        return {
            "sequence_id": sequence_id,
            "timestamp": int(Time.get_unix_time_from_system() * 1000),
            "payload": payload
        }

    # Test message creation
    var test_messages = [
        {"Ping": {"timestamp": 1234567890}},
        {"MovementIntent": {
            "target_position": {"x": 10.0, "y": 0.0, "z": 5.0},
            "speed_modifier": 1.0,
            "stop_movement": false
        }},
        {"CombatAction": {
            "action_type": "AutoAttack",
            "target_entity_id": 2,
            "ability_id": 0
        }}
    ]

    var sequence := 0
    for payload in test_messages:
        var message = build_envelope(payload, sequence)
        sequence += 1
        print("Message created: ", JSON.stringify(message))

    print("Message protocol test completed")
    quit()
EOF

# Run protocol test
cd client
godot --script test_protocol.gd
```

## 4. Performance Testing

### 4.1 Server Load Testing
```bash
# Test multiple connections (requires additional tools)
# Using a WebSocket load testing tool or multiple client instances

# Monitor server performance
htop  # or top
# Check CPU/memory usage during connections
```

### 4.2 Client Performance Testing
```bash
# In Godot editor, enable profiler
# Run GameWorld scene
# Monitor FPS and frame time
# Test with different input patterns
```

## 5. Error Testing

### 5.1 Connection Failure Testing
```bash
# Test with invalid server address
# Test with server not running
# Test with invalid protocol versions
```

### 5.2 Message Error Testing
```bash
# Send malformed JSON messages
# Send messages with missing required fields
# Test server error responses
```

## 6. Automated Testing

### 6.1 Server Unit Tests
```bash
cd server
cargo test
```

### 6.2 Integration Tests
```bash
# Create integration test script
cat > test_integration.sh << 'EOF'
#!/bin/bash
echo "Running OpenMMO integration tests..."

# Start database
docker-compose up -d db
sleep 5

# Run migrations
cd server
cargo sqlx migrate run

# Start server in background
DATABASE_URL=postgres://openmmo_user:openmmo_dev_password@localhost:5432/openmmo \
SERVER_HOST=127.0.0.1 \
SERVER_PORT=8081 \
RUST_LOG=error \
cargo run --bin server &
SERVER_PID=$!

sleep 3

# Test health endpoints
curl -s http://127.0.0.1:8081/health | grep -q "healthy" && echo "✓ Server health OK" || echo "✗ Server health failed"

# Test WebSocket connection (simplified)
# ... WebSocket test logic ...

# Cleanup
kill $SERVER_PID
docker-compose down

echo "Integration tests completed"
EOF

chmod +x test_integration.sh
./test_integration.sh
```

## 7. Troubleshooting

### Common Issues

**Server won't start:**
- Check database is running: `docker-compose ps`
- Verify DATABASE_URL in .env file
- Check for port conflicts: `netstat -tlnp | grep 8080`

**Client connection fails:**
- Verify server is running and accessible
- Check WebSocket URL format
- Review server logs for connection errors

**Module loading errors:**
- Verify file paths in Godot
- Check for syntax errors in GDScript files
- Ensure all dependencies are loaded

**Performance issues:**
- Monitor system resources
- Check for memory leaks
- Profile with Godot's built-in tools

## 8. Test Results Documentation

When running tests, document:
- Test environment (OS, hardware specs)
- Godot version and server Rust version
- Success/failure status for each test
- Performance metrics (FPS, latency, memory usage)
- Any errors or unexpected behavior
- Screenshots/videos for visual tests

This comprehensive testing suite ensures all OpenMMO components work correctly and provides a foundation for ongoing development and quality assurance.
