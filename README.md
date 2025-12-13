# OpenMMO

A fully open-source MMORPG framework built with modern technologies, designed to be extensible and community-driven.

## Overview

OpenMMO is a comprehensive MMORPG framework that provides:

- **Server-authoritative multiplayer** with anti-cheat guarantees
- **Extensible architecture** for easy content addition
- **Modern technology stack**: Rust (server) + Godot 4.x (client) + PostgreSQL
- **Open licensing**: AGPL for code, CC BY/CC0 for assets
- **Cross-platform support**: Windows, Linux, macOS

## Architecture

### Server (Rust)
- **Runtime**: Tokio async runtime
- **Web Framework**: Axum for HTTP endpoints
- **Database**: PostgreSQL with SQLx for compile-time checked queries
- **Protocol**: WebSocket + Protobuf for real-time communication
- **Logging**: Structured logging with tracing

### Client (Godot 4.x)
- **Engine**: Godot 4.x with 3D rendering
- **Style**: Low-poly, stylized 3D graphics
- **Target**: 1080p at 60 fps on mid-range hardware
- **Logic**: Engine-agnostic game logic with Godot for UI/rendering only

### Networking
- **Transport**: WebSocket over TLS (WSS)
- **Serialization**: Protobuf for compact binary protocol
- **Model**: Server-authoritative with client-side prediction for movement

## Quick Start

### Prerequisites
- Rust 1.70+ with cargo, rustfmt, clippy
- Godot 4.x engine
- Docker and Docker Compose
- PostgreSQL client tools

### Development Setup

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

## Project Structure

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

## Development Phases

This project follows a structured development approach:

1. **Phase 0** - Repository & Infrastructure Setup ✅
2. **Phase 1** - Networking & Protocol Skeleton
3. **Phase 2** - Core Server Gameplay Loop
4. **Phase 3** - Persistence, Accounts, and Characters
5. **Phase 4** - Inventory, Items, Classes, and NPCs
6. **Phase 5** - Second Zone and Content Pass
7. **Phase 6** - Admin Tools, Packaging, and Release Prep

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