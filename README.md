# OpenMMO

A fully open-source MMORPG framework built with modern technologies, designed to be extensible and community-driven.

## What is OpenMMO?

OpenMMO is a complete, open-source MMORPG (Massively Multiplayer Online Role-Playing Game) framework that lets you create and host your own online multiplayer game worlds. Whether you're a developer building a game or a community running a server, OpenMMO provides everything you need for persistent multiplayer gameplay.

### Key Features (MVP Target)

- **Real-time Multiplayer**: Server-authoritative gameplay supporting 10-30 concurrent players per zone
- **Character Progression**: Level up, gain experience, and unlock abilities
- **Combat System**: Tab-target combat with auto-attack and special abilities
- **World Exploration**: Multiple zones with NPCs, mobs, and interactive objects
- **Social Features**: Chat, party system, and player interaction
- **Extensible Content**: Easy to add new zones, classes, items, and quests
- **Cross-Platform**: Play on Windows, Linux, or macOS

### Game Content (MVP)

- **2 Game Zones**: Starter area and expanded second zone
- **3 Character Classes**: Melee, ranged, and support specializations
- **4 Player Character Models**: Diverse character appearances
- **2 Mob Types**: Hostile creatures with combat AI
- **Quest System**: Kill and collect quests with progression tracking
- **Item System**: Weapons, armor, consumables, and loot drops

## Current Status

- **Networking**: WebSocket server with JSON envelopes using the manual structs in `server/src/network/messages.rs`; `proto/messages.proto` exists but Protobuf generation is not wired, so the runtime does not use binary messages yet.
- **Implemented pieces**: Handshake + ping/pong, session creation, basic auth/register flow backed by SQLx, synthetic player ID allocation, two placeholder zones with simple movement/combat queues.
- **Not yet implemented**: Chat, quest pipeline, inventory/equipment handling, character list/create/select/delete flows over the network, Protobuf transport, class-specific abilities, mobs/NPC content, or client-side content for the advertised zones/classes.

## Getting Started

### For Players
1. **Download** the game client for your platform (Windows/Linux/macOS)
2. **Connect** to a game server
3. **Create** your character and choose a class
4. **Explore** the game world, fight monsters, and complete quests
5. **Level up** and unlock new abilities

### For Server Operators
1. **Set up** the server using the provided Docker environment
2. **Configure** your game world and content
3. **Launch** your server and let players connect
4. **Customize** zones, NPCs, and quests as needed

### For Developers
See the [Development Setup](#development-setup) section below.

## Gameplay Features

### Combat & Classes
- **Tab-Target Combat**: Click on enemies to attack them
- **Auto-Attack**: Automatic weapon attacks while in combat
- **Special Abilities**: Unique skills for each character class
- **Cooldown System**: Strategic timing of abilities

### World & Exploration
- **Multiple Zones**: Travel between different game areas
- **Interactive NPCs**: Quest givers, vendors, and story characters
- **Hostile Mobs**: Combat encounters with AI-controlled enemies
- **Loot System**: Collect items from defeated enemies

### Character Development
- **Experience & Levels**: Gain XP through combat and quests
- **Ability Unlocks**: Learn new skills as you progress
- **Equipment**: Wear weapons and armor to improve stats
- **Inventory Management**: Store and organize your items

### Social Features
- **Global Chat**: Communicate with all players
- **Zone Chat**: Talk with players in your current area
- **Party System**: Group up with other players (planned)
- **Private Messages**: Direct communication with friends

## Development Setup

### Prerequisites
- Rust 1.70+ with cargo, rustfmt, clippy
- Godot 4.x engine
- Docker and Docker Compose
- PostgreSQL client tools

### Quick Start for Developers

1. **Clone the repository**
    ```bash
    git clone https://github.com/your-org/OpenMMO.git
    cd OpenMMO
    ```

2. **Set up the development environment**
    ```bash
    # Start PostgreSQL database
    docker-compose up -d db

    # Install Rust dependencies
    cargo build

    # Set up database migrations
    cargo sqlx migrate run
    ```

3. **Run the server**
    ```bash
    cargo run --bin server
    ```

4. **Run the client**
    ```bash
    # Open in Godot Editor
    godot --path client/
    ```

### Project Structure

```
OpenMMO/
├── server/                 # Rust server code
│   ├── src/
│   ├── migrations/         # Database migrations
│   └── Cargo.toml
├── client/                 # Godot client project
│   ├── scenes/            # Game scenes
│   ├── scripts/           # GDScript/C# files
│   └── project.godot
├── assets/                # Game assets with licensing metadata
├── docs/                  # Documentation
├── proto/                 # Protobuf definitions
├── .github/workflows/     # CI/CD pipelines
├── docker-compose.yml     # Development database
└── Cargo.toml            # Rust workspace
```

## Development Roadmap

This project follows a structured development approach:

1. **Phase 0** - Repository & Infrastructure Setup ✅
2. **Phase 1** - Networking & Protocol Skeleton 🚧 (WebSocket JSON prototype; Protobuf wiring pending)
3. **Phase 2** - Core Server Gameplay Loop 🚧 (Basic movement/combat tick only)
4. **Phase 3** - Persistence, Accounts, and Characters 🚧 (Account auth present; character lifecycle not exposed)
5. **Phase 4** - Inventory, Items, Classes, and NPCs
6. **Phase 5** - Second Zone and Content Pass
7. **Phase 6** - Admin Tools, Packaging, and Release Prep

### Technical Architecture

**Server (Rust)**
- Async runtime with Tokio
- WebSocket networking with JSON envelopes today; Protobuf schema lives in `proto/messages.proto` and will replace JSON once generation is wired up
- PostgreSQL database with SQLx
- Entity Component System (ECS) for game logic
- 20 Hz simulation tick loop

**Client (Godot 4.x)**
- 3D rendering with low-poly stylized graphics
- Real-time multiplayer networking
- Input handling and UI systems
- Cross-platform export support

## Contributing

Please read [AGENTS.md](AGENTS.md) for detailed guidelines on automated development and [CONTRIBUTING.md](CONTRIBUTING.md) for human contributors.

### Development Rules

- Follow the specification in `open_mmorpg_mvp_specification.md`
- Use only approved dependencies (see AGENTS.md)
- All changes must be submitted via pull requests to `master`
- Maintain clean, documented code following project conventions

## License

- **Code**: [GNU Affero General Public License v3.0](LICENSE) (AGPL-3.0)
- **Assets**: Creative Commons BY or CC0 (see individual asset licensing)

## MVP Scope

The initial MVP includes:
- **2 zones**: Starter zone and second zone
- **3 classes**: Melee, ranged, support
- **4 player character models**
- **2 mob classes** with variants
- **10-30 concurrent players** per zone

## Support

- **Documentation**: See `docs/` directory
- **Issues**: Use GitHub Issues
- **Discussions**: Use GitHub Discussions for community questions

---

Built with ❤️ for the open-source gaming community.
