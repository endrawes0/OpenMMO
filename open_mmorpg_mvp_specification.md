# Open MMORPG MVP – Full Specification

## 1. Goals and Constraints

### Goals
- Deliver a **fully open-source MMORPG framework** (code + assets) with limited initial content.
- Provide a **clean, extensible architecture** so new zones, classes, mobs, and items can be added primarily through data, not code.
- Support **persistent multiplayer play** with server authority and basic anti-cheat guarantees.

### Non-goals (Initial Phase)
- Large-scale concurrency (hundreds+ per zone).
- Advanced PvP systems.
- Sophisticated quest scripting or branching narratives.
- High-fidelity visuals or cinematic presentation.

### Constraints
- Server-authoritative simulation for all gameplay-relevant state.
- Entirely open licensing for code and assets.
- Limited content:
  - 2 zones
  - 3 classes
  - 4 player character models
  - 2 mob classes (each with variants)

---

## 2. High-Level Architecture

### Engine and Implementation Overview
- **Client engine (initial)**: Existing open-source 3D engine, pragmatic default of **Godot 4.x**.
- **Client logic placement**:
  - Core game/domain logic, networking, and state management live in engine-agnostic modules.
  - Godot is primarily responsible for scenes, rendering, input binding, and UI wiring.
- **Visual target**:
  - Low-poly, stylized 3D.
  - 1080p at 60 fps on a mid-range integrated GPU laptop, with simple lighting and particle effects.
- **Server implementation**:
  - Rust-based dedicated server.
  - Async runtime: Tokio (default ecosystem choice for async Rust networking).
  - Web/API layer: Axum or similar Tokio-based framework for HTTP endpoints (patch manifests, health checks) alongside the custom WebSocket server.
- **Architecture priority**: Clean separation between client engine concerns and shared simulation/domain logic, so server and core client logic can scale and evolve independently.

### Authoritative Model

### Authoritative Model
- **Server is the sole authority** for:
  - Position validation
  - Combat resolution
  - Loot generation
  - Inventory, equipment, and progression
  - Quest state
- Client sends **intent only** (movement input, ability activation, interaction request).

### Logical Components
- Client (rendering, input, prediction, UI)
- Login/Auth Service
- World Service (global state, routing)
- Zone Service(s)
- Persistence Layer (database)

Initial deployment may collapse services into a single process, but interfaces must be designed as if they are separate.

---

## 3. Networking Model

### Transport & Protocol Stack
- **Transport**: WebSocket over TLS (WSS), using standard HTTPS ports for NAT/firewall friendliness.
- **Serialization**: Protobuf (or a similar compact binary protocol) for all gameplay and auth messages to minimize bandwidth and parsing overhead.
- **Session model**: One persistent WebSocket connection per logged-in client.

### Message Principles
- Explicit, versioned message types defined in `.proto` files shared between server and client.
- Server timestamps authoritative events; clients reconcile server updates with local prediction.
- Clear separation between:
  - Authentication/session messages
  - World/gameplay messages
  - Auxiliary APIs (e.g., patch manifests) which may use JSON/HTTP out-of-band.

### Core Message Categories
- Authentication & session
- Movement intent
- Ability cast intent
- Combat results (server → client)
- Inventory updates
- Chat/social
- Zone transfer

---

## 4. Persistence & Data Storage

### Database Technology
- **Primary store**: PostgreSQL.
- **Access layer**: Rust async stack with SQLx (compile-time checked queries, async/await compatible with the chosen runtime).
- **Migration strategy**: Versioned migrations checked into the repository and applied in order on deployment.

### Schema (MVP)
Minimum tables:
- Accounts
- Characters
- CharacterStats
- InventoryItems
- EquippedItems
- Progression (XP, level, skill unlocks)
- QuestState

### Save Strategy
- Periodic snapshot of active characters (e.g., every N seconds per zone).
- Immediate save on critical events:
  - Login/logout
  - Zone transitions
  - Item acquisition/loss
  - Trades
  - Level-ups and quest completions

---

## 5. Entity System

### Entity Types
- PlayerCharacter
- Mob
- NPC (non-hostile)
- WorldObject (chests, portals, vendors)

### Shared Components
- Position / movement
- Health / resources
- Faction / aggro rules
- Ability container

Entities are instantiated by the server and replicated to clients as needed.

---

## 6. Combat System

### Simulation Tick Model
- **World tick rate**: 20 ticks per second (50 ms per tick) for core simulation (movement resolution, combat, AI updates).
- Latency-sensitive actions (movement, basic ability activations) are predicted client-side and reconciled against server results.
- Non-real-time subsystems (chat, some persistence actions, certain quest updates) may be event-driven rather than tied strictly to the world tick.

### Design Principles
- Deterministic, server-only resolution for all combat outcomes.
- No client-side authority over damage, hit chance, or loot.
- Data-driven abilities and effects (definitions loaded from content files on server startup).

### Core Concepts
- Tab-targeted abilities with optional ground or cone-based AoE.
- Cooldowns, resource costs, and cast times.
- Damage, healing, and status effects.

### Status Effects
- Defined as data:
  - Duration
  - Stack rules
  - Periodic effects
  - Modifiers (damage taken, speed, etc.)

---

## 7. Classes (Initial)

### Class Structure
- Each class defined entirely in data:
  - Base stats
  - Resource type
  - Ability list

### Initial Classes (Placeholder)
- Melee-oriented class
- Ranged physical or magical class
- Support or hybrid class

Each class:
- 4–6 abilities
- Clear mechanical identity

---

## 8. Items & Inventory

### Item Categories
- Weapons
- Armor
- Consumables
- Miscellaneous (quest items)

### Item Properties
- Slot
- Stats/modifiers
- Rarity
- Bind rules (if any)

### Loot System
- Mob drop tables defined in data
- Server rolls loot on mob death

---

## 9. Zones & World Structure

### Zone Definition
Each zone defines:
- Terrain/geometry reference
- Spawn tables
- Mob density rules
- Entry/exit portals
- Ambient settings

### Initial Zones
- Starter zone (safe, low difficulty)
- Second zone (higher difficulty, introduces mechanics)

Zones may initially be static (no instancing), with instancing support designed but deferred.

---

## 10. AI System

### Mob AI Profiles
- Idle / patrol
- Aggro detection
- Combat behavior
- Leashing / retreat

AI behavior is parameterized via data profiles, not hardcoded logic per mob.

---

## 11. Social Systems

### Chat
- Global
- Zone-local
- Private messages

### Party System (Phase 1+)
- Invite / leave
- Shared XP
- Party chat

---

## 12. Client Responsibilities

- Rendering and animation
- Input handling
- Client-side prediction (movement only)
- UI (inventory, abilities, chat)
- Asset loading and validation

Client must tolerate authoritative corrections without desync or crashes.

---

## 13. Content Pipeline

### Data-Driven Content
All gameplay-relevant content lives in versioned data files:
- abilities.yaml/json
- items.yaml/json
- mobs.yaml/json
- zones.yaml/json

### Asset Management
- Assets stored separately from logic
- All assets include license metadata
- One shared humanoid rig (if 3D) for players and humanoid mobs

---

## 14. Anti-Cheat & Integrity (Baseline)

- Server validates:
  - Movement speed
  - Ability cooldowns
  - Range checks
- Client input rate limiting
- No trust in client-supplied state see or calculations

---

## 15. Tooling & Dev Utilities

- Server console/admin commands:
  - Spawn mob
  - Give item
  - Teleport
  - Reload data files
- Structured logging
- Deterministic simulation where possible

---

## 16. Licensing & Governance

- Code: OSI-approved license (to be selected)
- Assets: CC BY or CC0 preferred
- Contributor guidelines required for asset and code submissions

---

## 17. Milestones

### Target Scale Assumptions
- MVP tuned for **"friends-and-community" scale** (≈10–30 concurrent players per zone) with architectural patterns that can evolve toward **"small public server" scale** (≈50–100 concurrent players per zone) through optimization and horizontal scaling.

### Milestone 1: Vertical Slice
- End-to-end networking in place (WebSocket + Protobuf)
- Login → enter zone → move → basic tab-target combat → death → respawn
- One shared zone, one mob type, no persistence yet.

### Milestone 2: MMO Core
- Persistence with PostgreSQL/SQLx for accounts and characters
- Inventory, items, loot tables, and equipment
- Three basic classes (3 abilities each)
- Basic NPCs and chat

### Milestone 3: Content Expansion
- Second zone with higher difficulty
- Full class kits (4–6 abilities per class)
- Basic quest system and quest data for both zones
- NPC vendors with buy/sell flows

### Milestone 4: Operations & Polish
- In-game admin commands (teleport, spawn, give item, kick)
- Structured logging for auth, character, item, and combat events
- Initial packaging/export for Windows, Linux, and macOS clients
- Hardened account system with password hashing

---

## 18. Locked Design Decisions

The following early design decisions are now locked and reflected throughout the architecture:

1. **Client presentation**: 3D
2. **Camera model**: Classic semi-locked third-person camera (EverQuest / Asheron's Call / Project: Gorgon style)
3. **Combat targeting**: Tab-target
4. **Networking priority**: Responsiveness over strict visual correctness (with server authority preserved)
5. **Persistence cadence**: Periodic snapshots, with immediate saves on critical events (zone transitions, combat resolution, item pickup, trades)
6. **Zone instancing philosophy**: Shared overworld zones (no per-group instancing initially)
7. **Modding boundaries**: No client-side modding initially
8. **Licensing preference**: Code under AGPL; assets under a compatible mix of CC licenses (e.g., CC BY / CC0) with clear documentation
9. **Platform focus**: Windows first, with Linux and macOS supported as part of the export and packaging pipeline
10. **Input**: Keyboard + mouse only for MVP

These decisions should be treated as architectural constraints unless explicitly revisited.

---

## 19. Project Plan for Coding Agents

This section describes the implementation roadmap as a sequence of phases and concrete tasks, written to be directly actionable by coding agents.

### Phase 0 – Repository & Infrastructure Setup

**Goals**: Establish a clean, reproducible development environment and baseline project layout.

**Tasks**:
1. **Initialize GitHub repository**
   - Create main branch with AGPL license file and initial README describing project scope and goals.
   - Configure branch protection: feature branches + mandatory PR reviews into `main`.
2. **Set up Rust workspace**
   - Create a Cargo workspace with at least one crate for the server (`server/`).
   - Configure Rust formatting and linting (rustfmt, Clippy) and add basic CI checks.
3. **Set up Godot client project**
   - Initialize a Godot 4.x project under `client/`.
   - Configure platform exports for Windows, Linux, macOS (even if only Windows is used initially).
4. **Basic PostgreSQL integration (dev)**
   - Provide Docker or local dev instructions for running PostgreSQL.
   - Add a simple connection test from the Rust server to verify DB connectivity.
5. **Logging baseline**
   - Choose a structured logging crate (e.g., `tracing`) and configure JSON or key-value logs as a default.

### Phase 1 – Networking & Protocol Skeleton

**Goals**: Define the wire protocol and establish a minimal client/server communication loop.

**Tasks**:
1. **Define Protobuf schemas**
   - Create `.proto` definitions for core message types: handshake, auth, ping/pong, error, basic world snapshots.
   - Generate Rust and client-side bindings.
2. **Implement WebSocket server**
   - Using Tokio + chosen WebSocket library (and Axum if used), implement a server that accepts client connections, performs a simple handshake, and echoes or logs messages.
3. **Implement minimal Godot network client**
   - Connect to the server over WSS.
   - Send a ping message at intervals and display responses in a debug overlay.
4. **Session management**
   - Add a simple in-memory session store keyed by connection ID, with graceful handling of disconnects.

### Phase 2 – Core Server Gameplay Loop

**Goals**: Implement the authoritative world loop, entities, movement, and basic combat.

**Tasks**:
1. **World tick loop**
   - Implement a 20 Hz tick loop that processes movement, combat, and AI for active zones.
2. **Entity system**
   - Define server-side entity structures for players, mobs, and simple NPCs.
   - Implement spawn/despawn management and per-zone entity registries.
3. **Movement handling**
   - Define movement intent messages and server-side validation (speed checks, simple collision boundaries).
   - Broadcast periodic position updates to relevant clients.
4. **Combat MVP**
   - Implement a simple tab-target combat model with auto-attack or one basic ability per class.
   - Handle damage, death, and respawn at a fixed spawn point.
5. **Minimal AI**
   - Implement basic mob AI: idle, aggro on proximity, chase, attack, and leash back.

### Phase 3 – Persistence, Accounts, and Characters

**Goals**: Add real accounts, character records, and persistent world state for players.

**Tasks**:
1. **Schema and migrations**
   - Define SQLx-backed schemas for accounts, characters, and core stats.
   - Implement migration tooling and document how to apply migrations.
2. **Account system**
   - Implement simple email/username + password registration and login.
   - Use secure password hashing (Argon2 or bcrypt) and ensure no plaintext passwords are logged.
3. **Character creation & selection**
   - Implement character creation flow (name, class, basic appearance).
   - Support multiple characters per account and a selection screen on the client.
4. **Persistent character state**
   - Save/load level, XP, position (or spawn on login), inventory snapshot, and equipped items.

### Phase 4 – Inventory, Items, Classes, and NPCs

**Goals**: Flesh out core MMO systems and first pass of game content.

**Tasks**:
1. **Item system**
   - Implement items table and server-side inventory management.
   - Define item categories, equip rules, and stat modifications.
2. **Loot tables**
   - Add data-driven mob drop tables and loot generation on mob death.
3. **Class abilities**
   - Expand each class to at least 3 abilities, defined in data files (e.g., `abilities.yaml/json`).
   - Implement server-side cooldowns, costs, and basic status effects.
4. **NPC vendors**
   - Implement vendor NPCs with buy/sell interactions and price data.
5. **Basic quest system**
   - Implement minimal quest definitions, state tracking, and progression for kill/collect quests.
   - Wire up a handful of quests in the starter zone.

### Phase 5 – Second Zone and Content Pass

**Goals**: Add a second zone and enough content to demonstrate the extensibility of the framework.

**Tasks**:
1. **Second zone data**
   - Define a new zone with its own environment, spawn tables, and level range.
2. **Mob variants**
   - Create additional variants of the two base mob classes with different stats/abilities.
3. **Zone transitions**
   - Implement portals or transition points between the starter zone and the second zone.
4. **Additional quests and vendors**
   - Add zone-specific quests and at least one vendor in the second zone.

### Phase 6 – Admin Tools, Packaging, and Release Prep

**Goals**: Make the game manageable as a live service and shippable to players.

**Tasks**:
1. **In-game admin commands**
   - Teleport player, spawn mob, give item, kick/ban.
2. **Structured logging coverage**
   - Ensure logs capture auth events, character lifecycle, item creation/deletion, and significant combat events.
3. **Client packaging**
   - Configure Godot exports for Windows, Linux, macOS.
   - Provide a simple launcher or startup script.
4. **Documentation**
   - Write developer setup docs.
   - Document the wire protocol, data schemas, and content pipeline at a high level.

This project plan is intended as the baseline roadmap for implementation; tasks can be refined or split further by coding agents as work begins.

