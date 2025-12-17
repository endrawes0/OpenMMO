# OpenMMO Client

This is the Godot 4.x client for OpenMMO, following the project's architecture guidelines.

## Project Structure

```
client/
├── project.godot          # Godot project configuration
├── scenes/                # Game scenes
│   ├── Main.tscn         # Main menu scene
│   └── GameWorld.tscn    # Game world placeholder
├── scripts/               # GDScript files (UI binding only)
│   ├── Main.gd           # Main menu controller
│   ├── NetworkDebug.gd   # Network debug overlay
│   └── GameWorld.gd      # Game world controller
├── assets/                # Game assets
│   ├── models/           # 3D models
│   ├── textures/         # Texture files
│   ├── audio/            # Audio files
│   └── ui/               # UI elements
└── export_presets.cfg    # Export configurations
```

## Architecture Notes

Following AGENTS.md guidelines:
- **GDScript ONLY** for UI binding, scene setup, and engine-specific code
- **NO business logic** or protocol logic inside Godot scripts
- Engine-agnostic game logic should be stored outside Godot scripts
- Uses ONLY built-in Godot packages and officially supported addons

## Development Setup

1. Install Godot 4.x editor
2. Open this project in Godot
3. The main scene is set to `scenes/Main.tscn`
4. Export presets are configured for Windows, Linux, and macOS

## Current Features

- Login, registration, and character selection UI with create/select flows
- Engine-agnostic networking, game state, input, movement, and UI modules
- Transition from character selection into the initial zone scene
- Third-person placeholder zone with terrain, spawn camp, and lighting pass
- Player capsule with WASD movement, mouse-driven camera, and scroll zoom
- Network debug overlay for monitoring connection status
- Export presets for Windows, Linux, and macOS

## Next Steps

The client structure is ready for integration with:
- Real authentication + character services backed by the Rust server
- Authoritative world snapshots and entity replication
- Combat, abilities, and inventory UI layers
- Server-driven movement reconciliation and prediction correction

## Controls

- `W/A/S/D`: Move the player capsule (strafe + forward/back)
- `Mouse Left Drag`: Orbit the camera around the player without turning
- `Mouse Right Drag`: Rotate the player and keep the camera aligned
- `Mouse Wheel`: Zoom the camera in/out
- `ESC`: Return to the character selection menu
