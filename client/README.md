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

- Basic main menu with connection UI (placeholder)
- Network debug overlay for monitoring connection status
- Basic game world scene with placeholder player and environment
- Input mappings for keyboard + mouse controls
- Export presets for all target platforms

## Next Steps

The client structure is ready for integration with:
- Engine-agnostic networking modules
- Protocol message handling
- Server communication
- Actual game logic implementation

## Controls

- W/A/S/D: Movement (placeholder)
- Mouse: Camera control (placeholder)
- E: Interact (placeholder)
- Left Click: Attack (placeholder)
- T: Toggle chat (placeholder)
- I: Toggle inventory (placeholder)
- ESC: Return to menu