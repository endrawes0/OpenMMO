# OpenMMO Client

This is the Godot 4.x client for OpenMMO, following the project's architecture guidelines.

## Project Structure

```
client/
├── project.godot          # Godot project configuration
├── scenes/                # Game scenes
│   ├── Main.tscn         # UI scaffolding for auth/character flows
│   └── GameWorld.tscn    # Placeholder zone with simple terrain + capsule player
├── scripts/               # Scene-specific controllers (UI binding only)
│   ├── Main.gd
│   ├── NetworkDebug.gd
│   └── GameWorld.gd
├── src/                   # Engine-agnostic modules (networking, movement, state, input, UI)
├── assets/                # Empty placeholder folders + README stubs (no shipped art assets)
└── export_presets.cfg    # Export configurations for desktop targets
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

- UI scaffolding for login/registration/character selection wired to engine-agnostic modules (no bundled backend)
- Engine-agnostic networking, game state, input, movement, and UI state modules under `src/`
- Transition from character selection into a placeholder GameWorld scene
- Placeholder zone with simple terrain, basic lighting, and a capsule player with WASD + mouse orbit/zoom controls
- Network debug overlay for connection status text
- Export presets for Windows, Linux, and macOS
- Asset folders are empty aside from README stubs; no models, textures, audio, or UI art are shipped yet

## Next Steps

The client structure is ready for integration with:
- Real authentication + character services backed by the Rust server
- Importing licensed placeholder assets (models, textures, audio, UI) and wiring them into scenes
- Authoritative world snapshots, entity replication, and richer zone content
- Server-driven movement reconciliation, combat, abilities, and inventory UI layers

## Controls

- `W/A/S/D`: Move the player capsule (strafe + forward/back)
- `Mouse Left Drag`: Orbit the camera around the player without turning
- `Mouse Right Drag`: Rotate the player and keep the camera aligned
- `Mouse Wheel`: Zoom the camera in/out
- `ESC`: Return to the character selection menu
