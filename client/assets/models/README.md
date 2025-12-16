# 3D Models for OpenMMO Client

This directory contains all 3D model assets used in the OpenMMO game client. All models must be in open-license formats (CC0, CC BY, or AGENTS.md approved) with proper attribution.

## Required Formats
- **GLTF/GLB** (preferred): Modern format with embedded textures and animations
- **OBJ**: Simple format for static meshes
- **FBX**: Industry standard (ensure export settings match Godot import)
- **DAE (Collada)**: Alternative format

## Directory Structure
```
models/
├── characters/            # Character models
│   ├── player/           # Player character models
│   │   ├── base_male.gltf
│   │   ├── base_female.gltf
│   │   ├── animations/
│   │   │   ├── idle.gltf
│   │   │   ├── walk.gltf
│   │   │   ├── run.gltf
│   │   │   └── attack.gltf
│   │   └── textures/     # Character-specific textures
│   ├── mobs/             # Monster/NPC models
│   │   ├── goblin/
│   │   │   ├── goblin.gltf
│   │   │   └── textures/
│   │   └── wolf/
│   │       ├── wolf.gltf
│   │       └── textures/
│   └── npcs/             # Non-combat NPCs
│       ├── merchant/
│       └── guard/
├── environment/          # Environmental models
│   ├── terrain/          # Terrain pieces
│   │   ├── ground_tile.gltf
│   │   └── cliff_face.gltf
│   ├── props/            # Interactive objects
│   │   ├── chest.gltf
│   │   ├── tree.gltf
│   │   └── rock.gltf
│   ├── buildings/        # Structures
│   │   ├── house_small.gltf
│   │   ├── shop.gltf
│   │   └── castle_wall.gltf
│   └── vegetation/       # Plants and foliage
│       ├── grass_patch.gltf
│       └── bush.gltf
├── weapons/              # Weapon models
│   ├── swords/
│   │   ├── iron_sword.gltf
│   │   └── steel_sword.gltf
│   ├── bows/
│   └── staves/
├── items/                # Inventory items
│   ├── consumables/
│   │   ├── health_potion.gltf
│   │   └── mana_potion.gltf
│   ├── armor/
│   │   ├── leather_helmet.gltf
│   │   └── iron_chestplate.gltf
│   └── miscellaneous/
│       ├── gold_coin.gltf
│       └── key.gltf
└── effects/              # Visual effects models
    ├── particles/
    └── spell_effects/
```

## Model Specifications

### Characters
- **Triangle Count**: 5,000 - 15,000 triangles per model
- **Texture Resolution**: 1024x1024 or 2048x2048
- **Rigging**: Required for animated characters
- **LOD**: 3 levels (high, medium, low detail)
- **Animations**: Idle, walk, run, attack, death, cast (if applicable)

### Environment
- **Triangle Count**: 500 - 5,000 triangles per model
- **Modular Design**: Pieces should connect seamlessly
- **Collision Meshes**: Separate low-poly collision geometry
- **UV Unwrapping**: Efficient use of texture space

### Items/Weapons
- **Triangle Count**: 500 - 2,000 triangles
- **Scale**: Appropriate for character hand/object holding
- **Pivot Points**: Correctly positioned for attachment

## Technical Requirements
- **Units**: 1 unit = 1 meter in Godot
- **Scale**: Models should be properly scaled for the game world
- **Normals**: Properly calculated vertex normals
- **Materials**: PBR materials with albedo, normal, metallic, roughness maps
- **Textures**: Embedded in GLTF or provided separately
- **Animations**: 30 FPS, optimized keyframe reduction

## Licensing Requirements
- All 3D models must be licensed under CC0, CC BY, or other AGENTS.md approved licenses
- Create a `LICENSE.txt` file for each model with attribution
- Include source URLs, original author, and modification notes
- Document any blend files or source assets if available

## Recommended Sources
1. **BlenderKit.com** - CC0 3D models with Blender integration
2. **OpenGameArt.org** - Game-ready 3D assets
3. **Sketchfab.com** - Large collection (check licensing carefully)
4. **Kenney.nl** - Simple CC0 3D models
5. **Mixamo.com** - Character models and animations (free tier available)
6. **TurboSquid.com** - Professional models (verify CC licensing)

## Implementation Notes
- Import models into Godot with proper import settings
- Set up LOD systems for performance
- Create collision shapes for physics
- Optimize draw calls with mesh instancing
- Consider texture atlas for UI elements

## Priority Assets for MVP
1. Basic player character model with walk/idle animations
2. Simple terrain tiles (ground, walls)
3. Basic props (trees, rocks, chests)
4. Placeholder weapon models
5. Simple enemy models

## File Naming Convention
- Use lowercase with underscores: `iron_sword.gltf`
- Include variation numbers: `goblin_01.gltf`, `goblin_02.gltf`
- Separate animation files: `character_idle.gltf`, `character_walk.gltf`
- Texture files: `modelname_albedo.png`, `modelname_normal.png`