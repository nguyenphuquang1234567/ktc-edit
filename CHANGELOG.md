# Changelog

All notable changes to Kingdom Two Crowns Save Editor will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial release with save file editing capabilities
- Resource management (coins and gems)
- Island navigation and fast travel
- Unit spawning system
- Combat formation deployment
- Construction upgrades
- Automatic save file location detection
- Multi-language support (English and French)
- Automatic backup creation before saving

### Features

#### Resource Management
- Edit player coins with real-time value display
- Edit player gems with real-time value display
- Support for both Player 1 and Player 2

#### Island Navigation
- Fast travel with customizable resource loadout
- Take over islands with custom spawn configuration
- Destroy all enemy portals
- Exterminate all enemies
- Mark trees for removal with coin rewards

#### Combat
- Deploy battle formations at specific positions
- Spawn archers and pikemen in formation
- Configurable formation offset

#### Construction
- Pimp islands to maximum level
- Upgrade castle and walls automatically
- Spawn units and mark trees simultaneously

#### Unit Recruitment
- Spawn archers
- Spawn workers
- Spawn farmers
- Spawn pikemen
- Combined recruitment for multiple unit types
- Real-time unit count display

#### Safety & UX
- Automatic backup creation with timestamp
- Campaign and island context selector
- Clear status messages for all operations
- Automatic system language detection
- Default save file location detection per platform

### Technical
- Built with Tauri 2.x and SvelteKit 2.x
- TypeScript for type safety
- Svelte 5 with runes API
- Cross-platform support (macOS, Windows, Linux)
- Internationalization system with automatic language detection

## Version History

### [0.1.0] - YYYY-MM-DD

Initial release.

---

## Commit Message Convention

This project follows [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` New features
- `fix:` Bug fixes
- `docs:` Documentation changes
- `style:` Code style changes (formatting, etc.)
- `refactor:` Code refactoring
- `perf:` Performance improvements
- `test:` Test additions or modifications
- `chore:` Build process or auxiliary tool changes

Examples:
```
feat: add support for knight spawning
fix: correct coin calculation for player 2
docs: update installation instructions for Linux
chore: update Tauri to v2.8.5
```
