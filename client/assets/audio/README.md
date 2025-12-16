# Audio Assets for OpenMMO Client

This directory contains all audio assets used in the OpenMMO game client. All audio files must be in open-license formats (CC0, CC BY, or AGENTS.md approved) with proper attribution.

## Required Formats
- **OGG** (preferred): Compressed format, good for longer sounds and music
- **WAV**: Uncompressed format, suitable for short sound effects
- **MP3**: Alternative compressed format (ensure patent-free implementation)

## Directory Structure
```
audio/
├── sfx/                    # Sound effects
│   ├── ui/                # User interface sounds
│   │   ├── button_click.ogg
│   │   ├── button_hover.ogg
│   │   ├── notification.ogg
│   │   └── error.ogg
│   ├── movement/          # Player movement sounds
│   │   ├── footstep_grass_01.ogg
│   │   ├── footstep_stone_01.ogg
│   │   ├── jump.ogg
│   │   └── land.ogg
│   ├── combat/            # Combat-related sounds
│   │   ├── sword_swing_01.ogg
│   │   ├── sword_hit_01.ogg
│   │   ├── damage_taken.ogg
│   │   ├── death.ogg
│   │   └── level_up.ogg
│   └── environment/       # World interaction sounds
│       ├── door_open.ogg
│       ├── chest_open.ogg
│       └── item_pickup.ogg
├── music/                 # Background music
│   ├── ambient/           # Background tracks
│   │   ├── forest_ambient.ogg
│   │   ├── town_ambient.ogg
│   │   └── cave_ambient.ogg
│   ├── menu/              # Menu music
│   │   └── main_menu.ogg
│   └── combat/            # Combat music
│       └── battle_theme.ogg
└── voice/                 # Voice acting (if applicable)
    ├── player/            # Player voice lines
    └── npcs/              # NPC dialogue
```

## Audio Specifications

### Sound Effects (SFX)
- **Sample Rate**: 44.1kHz or 48kHz
- **Bit Depth**: 16-bit minimum
- **Length**: < 5 seconds for most effects
- **Channels**: Mono (stereo only for special effects)
- **Volume**: Normalized to -6dB to -12dB RMS

### Music
- **Sample Rate**: 44.1kHz or 48kHz
- **Bit Depth**: 16-bit minimum
- **Length**: Variable (loopable sections preferred)
- **Channels**: Stereo
- **Format**: OGG with quality setting 6-8

### Voice
- **Sample Rate**: 44.1kHz
- **Bit Depth**: 16-bit
- **Channels**: Mono
- **Language**: English (US accent preferred)

## Licensing Requirements
- All audio files must be licensed under CC0, CC BY, or other AGENTS.md approved licenses
- Create a `LICENSE.txt` file in each subdirectory with attribution information
- Include source URLs and original author credits

## Recommended Sources
1. **Freesound.org** - Large collection of CC0 and CC BY audio samples
2. **OpenGameArt.org** - Game-specific audio assets
3. **Zapsplat.com** - Professional sound effects (check licensing)
4. **Incompetech.com** - CC BY music by Kevin MacLeod
5. **YouTube Audio Library** - Some CC licensed tracks

## Implementation Notes
- Audio files should be imported into Godot with appropriate bus assignments
- Use Godot's audio streaming for large music files
- Consider audio compression settings in export templates
- Test audio on target platforms (web, desktop, mobile)

## Priority Assets for MVP
1. UI sound effects (button clicks, notifications)
2. Basic movement sounds (footsteps)
3. Simple ambient music loop
4. Basic combat sounds (hit/miss)

## File Naming Convention
- Use lowercase with underscores: `button_click.ogg`
- Include variation numbers: `footstep_grass_01.ogg`, `footstep_grass_02.ogg`
- Avoid special characters except underscores