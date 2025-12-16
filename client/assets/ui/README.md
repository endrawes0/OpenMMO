# UI Assets for OpenMMO Client

This directory contains all user interface assets used in the OpenMMO game client. All UI assets must be in open-license formats (CC0, CC BY, or AGENTS.md approved) with proper attribution.

## Required Formats
- **PNG** (preferred): Lossless with transparency support
- **SVG**: Vector format for scalable elements (converted to PNG for Godot)
- **WebP**: Modern compressed format
- **TTF/OTF**: Font files
- **JSON**: Theme configuration files

## Directory Structure
```
ui/
├── icons/                # Icon assets
│   ├── inventory/        # Inventory-related icons
│   │   ├── weapon_icon.png
│   │   ├── armor_icon.png
│   │   ├── consumable_icon.png
│   │   └── miscellaneous_icon.png
│   ├── abilities/        # Ability/skill icons
│   │   ├── sword_attack_icon.png
│   │   ├── fireball_icon.png
│   │   ├── heal_icon.png
│   │   └── buff_icon.png
│   ├── status/           # Status effect icons
│   │   ├── poison_icon.png
│   │   ├── bleed_icon.png
│   │   ├── buff_icon.png
│   │   └── debuff_icon.png
│   ├── navigation/       # UI navigation icons
│   │   ├── menu_icon.png
│   │   ├── settings_icon.png
│   │   ├── map_icon.png
│   │   └── quest_icon.png
│   └── currency/         # Currency and resource icons
│       ├── gold_coin_icon.png
│       ├── health_icon.png
│       └── mana_icon.png
├── backgrounds/          # Background textures
│   ├── panels/           # UI panel backgrounds
│   │   ├── inventory_panel.png
│   │   ├── character_panel.png
│   │   ├── dialog_panel.png
│   │   └── tooltip_panel.png
│   ├── buttons/          # Button backgrounds
│   │   ├── button_normal.png
│   │   ├── button_hover.png
│   │   ├── button_pressed.png
│   │   └── button_disabled.png
│   ├── windows/          # Window backgrounds
│   │   ├── main_menu_bg.png
│   │   ├── loading_screen_bg.png
│   │   └── popup_bg.png
│   └── borders/          # Border and frame elements
│       ├── panel_border.png
│       └── window_frame.png
├── cursors/              # Mouse cursor assets
│   ├── default_cursor.png
│   ├── hover_cursor.png
│   ├── attack_cursor.png
│   └── interact_cursor.png
├── fonts/                # Font assets
│   ├── main_font.ttf     # Primary UI font
│   ├── title_font.ttf    # Headers and titles
│   ├── monospace_font.ttf # Code/console font
│   └── bitmap_fonts/     # Bitmap fonts if needed
├── themes/               # Theme configuration
│   ├── default_theme.json
│   └── color_palettes.json
├── animations/           # UI animation assets
│   ├── button_press.png  # Sprite sheet for button animations
│   └── loading_spinner.png
└── placeholders/         # Temporary placeholder assets
    ├── missing_icon.png
    ├── loading_image.png
    └── default_avatar.png
```

## UI Asset Specifications

### Icons
- **Resolution**: 32x32 to 128x128 pixels
- **Color Depth**: 32-bit RGBA
- **Style**: Consistent art style across all icons
- **Padding**: 2-4 pixel transparent border
- **Scalability**: Design for 2x and 3x scaling

### Backgrounds & Panels
- **Resolution**: 256x256 to 1024x1024 pixels
- **Tileable**: 9-slice scaling support
- **Transparency**: Alpha channels for overlays
- **Resolution Independent**: Use 9-slice or vector elements

### Buttons
- **States**: Normal, hover, pressed, disabled, focused
- **Consistency**: Uniform sizing and styling
- **Accessibility**: Clear visual feedback for interactions

### Fonts
- **Formats**: TTF, OTF (web-safe fallbacks)
- **Character Sets**: Extended Latin, numbers, symbols
- **Sizes**: Multiple weights (regular, bold, light)
- **Readability**: Clear at small sizes (8-12pt)

## Technical Requirements
- **Color Space**: sRGB for all UI elements
- **DPI Awareness**: Design for multiple screen densities
- **Contrast**: WCAG AA compliance for text readability
- **Localization**: Support for text expansion in other languages
- **Performance**: Optimize texture atlases and batching

## Design Guidelines
- **Consistent Spacing**: Use a grid system (8px base unit)
- **Color Palette**: Limited, accessible color scheme
- **Typography**: 2-3 font families maximum
- **Iconography**: Clear, recognizable symbols
- **Responsive**: Adaptable to different screen sizes

## Licensing Requirements
- All UI assets must be licensed under CC0, CC BY, or other AGENTS.md approved licenses
- Create a `LICENSE.txt` file in each subdirectory with attribution
- Include source URLs, original author, and modification details
- Document design system and style guide

## Recommended Sources
1. **Kenney.nl** - CC0 UI packs and icon sets
2. **OpenGameArt.org** - UI assets and icon collections
3. **Game-Icons.net** - CC BY icon library
4. **Material Design Icons** - Open-source icon set
5. **Google Fonts** - Open-source font library
6. **Itch.io** - UI asset packs (verify licensing)

## Implementation Notes
- Import UI textures with appropriate compression in Godot
- Create texture atlases for icon sets to improve performance
- Set up Godot themes for consistent styling
- Use Godot's UI scaling system for different resolutions
- Consider localization expansion for text elements

## Priority Assets for MVP
1. Basic button styles (normal, hover, pressed)
2. Essential icons (inventory, health, mana, sword)
3. Simple panel backgrounds
4. Primary font (readable, fantasy-appropriate)
5. Loading and placeholder graphics

## File Naming Convention
- Use lowercase with underscores: `inventory_icon.png`
- Include state suffixes: `button_normal.png`, `button_hover.png`
- Use descriptive names: `health_potion_icon.png`
- Group related assets: `inventory_panel.png`, `inventory_border.png`