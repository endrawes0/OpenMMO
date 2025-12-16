# OpenMMO Client Assets

This directory contains all game assets for the OpenMMO client. All assets must comply with the licensing requirements specified in `AGENTS.md` and the main project specification.

## Directory Overview

- **[audio/](audio/)** - Sound effects, music, and voice assets
- **[models/](models/)** - 3D models for characters, environment, and items
- **[textures/](textures/)** - 2D textures for materials, UI, and effects
- **[ui/](ui/)** - User interface elements, icons, and themes

## General Requirements

### Licensing
- **CC0** (public domain) - preferred for maximum compatibility
- **CC BY** - attribution required, modifications allowed
- **CC BY-SA** - attribution and share-alike required
- All assets must include licensing metadata in `LICENSE.txt` files
- No copyrighted or proprietary assets permitted

### Technical Standards
- **Formats**: Use Godot-compatible formats (GLTF, PNG, OGG, etc.)
- **Optimization**: Assets should be optimized for real-time rendering
- **Documentation**: Each asset must have clear attribution and source information
- **Consistency**: Maintain visual and audio consistency across all assets

### File Organization
- Use lowercase filenames with underscores
- Include variation numbers for similar assets (`tree_01.gltf`, `tree_02.gltf`)
- Separate textures from models when possible
- Group related assets in subdirectories

## Asset Pipeline

### Acquisition
1. Identify needed assets from the detailed READMEs in each subdirectory
2. Source from approved open-license repositories
3. Verify licensing compatibility
4. Download and organize assets

### Processing
1. Convert to Godot-compatible formats if needed
2. Optimize for performance (reduce polygon counts, compress textures)
3. Set up proper import settings in Godot
4. Test in-game integration

### Integration
1. Import assets into Godot project
2. Set up materials, scenes, and prefabs
3. Implement LOD systems where appropriate
4. Optimize for target platforms

## Quality Assurance
- Test assets on target platforms (web, desktop, mobile)
- Verify performance impact
- Ensure accessibility compliance
- Validate licensing compliance

## Priority Development Order
1. **UI Assets** - Essential for basic functionality
2. **Textures** - Required for 3D models and environment
3. **Audio** - Enhances user experience
4. **3D Models** - Core gameplay assets

## Contributing Assets
- Follow the detailed specifications in each subdirectory's README
- Include all required licensing information
- Test assets in the Godot project before submission
- Document any modifications made to original assets

## Tools and Resources
- **Godot Engine** - Primary development environment
- **Blender** - 3D model editing and optimization
- **GIMP/Krita** - 2D texture editing
- **Audacity** - Audio editing and optimization

For detailed specifications of each asset type, see the README files in the respective subdirectories.</content>
<parameter name="filePath">client/assets/README.md