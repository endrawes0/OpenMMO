# Texture Assets for OpenMMO Client

This directory contains all texture assets used in the OpenMMO game client. All textures must be in open-license formats (CC0, CC BY, or AGENTS.md approved) with proper attribution.

## Required Formats
- **PNG** (preferred): Lossless compression, supports transparency
- **JPG/JPEG**: Lossy compression for photographs/large textures
- **WebP**: Modern format with good compression
- **TGA**: Alternative format with alpha support
- **KTX/ Basis**: Compressed GPU textures (advanced optimization)

## Directory Structure
```
textures/
├── characters/           # Character textures
│   ├── player/
│   │   ├── base_male_albedo.png
│   │   ├── base_male_normal.png
│   │   ├── base_male_metallic.png
│   │   ├── base_male_roughness.png
│   │   └── base_male_ao.png
│   └── mobs/
│       ├── goblin_albedo.png
│       └── goblin_normal.png
├── environment/          # Environmental textures
│   ├── terrain/
│   │   ├── grass_albedo.png
│   │   ├── grass_normal.png
│   │   ├── dirt_albedo.png
│   │   ├── stone_albedo.png
│   │   └── stone_normal.png
│   ├── buildings/
│   │   ├── wood_wall_albedo.png
│   │   ├── wood_wall_normal.png
│   │   └── stone_wall_albedo.png
│   └── props/
│       ├── tree_bark_albedo.png
│       ├── tree_leaves_albedo.png (with alpha)
│       └── rock_albedo.png
├── weapons/              # Weapon textures
│   ├── swords/
│   │   ├── iron_sword_albedo.png
│   │   └── iron_sword_normal.png
│   └── bows/
├── items/                # Item textures
│   ├── consumables/
│   │   ├── health_potion_albedo.png
│   │   └── mana_potion_albedo.png
│   └── armor/
│       └── leather_armor_albedo.png
├── ui/                   # User interface textures
│   ├── buttons/
│   │   ├── button_normal.png
│   │   ├── button_hover.png
│   │   ├── button_pressed.png
│   │   └── button_disabled.png
│   ├── icons/
│   │   ├── inventory_icon.png
│   │   ├── health_icon.png
│   │   ├── mana_icon.png
│   │   └── sword_icon.png
│   ├── panels/
│   │   ├── inventory_panel.png
│   │   ├── character_panel.png
│   │   └── dialog_panel.png
│   ├── backgrounds/
│   │   ├── menu_background.png
│   │   └── loading_background.png
│   └── fonts/            # Font textures (if bitmap fonts)
└── effects/              # Particle and effect textures
    ├── particles/
    │   ├── smoke.png (with alpha)
    │   ├── spark.png
    │   └── dust.png
    └── decals/
        ├── blood_splat.png
        └── footprint.png
```

## Texture Specifications

### PBR Material Maps
- **Albedo/Base Color**: RGB color information
- **Normal**: XYZ normal vectors (OpenGL convention)
- **Metallic**: Grayscale metallic values (0-1)
- **Roughness**: Grayscale roughness values (0-1)
- **Ambient Occlusion**: Grayscale AO values (0-1)
- **Emission**: RGB emission color/intensity
- **Height/Displacement**: Grayscale height information

### Resolution Guidelines
- **Characters**: 1024x1024 to 2048x2048
- **Environment**: 512x512 to 1024x1024 (tileable)
- **Props**: 256x256 to 512x512
- **UI Elements**: 64x64 to 512x512
- **Icons**: 32x32 to 128x128
- **Particles**: 64x64 to 256x256

### Technical Requirements
- **Power of 2**: Dimensions should be powers of 2 (256, 512, 1024, etc.)
- **Square Aspect**: Prefer square textures for consistency
- **Mipmaps**: Generate mipmaps for distance rendering
- **Compression**: Use appropriate compression (BC7 for color, BC5 for normal)
- **Color Space**: sRGB for albedo, linear for other maps

### UI Texture Requirements
- **Consistent Style**: Maintain visual consistency across all UI elements
- **Scalability**: Design for multiple resolutions
- **Accessibility**: Consider colorblind-friendly palettes
- **Transparency**: Use alpha channels appropriately

## Licensing Requirements
- All textures must be licensed under CC0, CC BY, or other AGENTS.md approved licenses
- Create a `LICENSE.txt` file in each subdirectory with attribution
- Include source URLs, original author, and modification details
- Document texture creation process if procedural

## Recommended Sources
1. **AmbientCG.com** - CC0 PBR texture library
2. **Polyhaven.com** - CC0 HDR textures and PBR materials
3. **Textures.com** - Professional textures (check licensing)
4. **OpenGameArt.org** - Game-specific textures
5. **Kenney.nl** - CC0 game textures and UI packs
6. **Itch.io** - CC licensed texture packs

## Implementation Notes
- Import textures into Godot with proper compression settings
- Set up texture atlases for UI elements to reduce draw calls
- Use texture streaming for large environments
- Consider texture arrays for variants
- Optimize for target platforms (mobile, web, desktop)

## Priority Assets for MVP
1. Basic terrain textures (grass, dirt, stone)
2. Simple UI elements (buttons, panels, icons)
3. Basic character textures (placeholder)
4. Particle effects (smoke, sparks)
5. Basic prop textures (trees, rocks)

## File Naming Convention
- Use lowercase with underscores: `grass_albedo.png`
- Include map type: `sword_albedo.png`, `sword_normal.png`, `sword_metallic.png`
- Use suffixes for variants: `button_normal.png`, `button_hover.png`
- Avoid special characters except underscores