# Kingdom Two Crowns Save Editor

A cross-platform save file editor for Kingdom Two Crowns built with Tauri and SvelteKit.

## Features

### Resource Management
- Edit player coins and gems
- Adjust resource amounts for any player (1 or 2)
- Real-time display of current values

### Island Navigation
- Fast travel between islands with custom resource loadout
- Configure coins, gems, pikemen, farmers, and boats for travel
- Take control of islands with custom spawn configurations
- Destroy all enemy portals
- Exterminate all enemies
- Mark trees for removal with coin rewards

### Combat
- Deploy battle formations at specific positions
- Spawn archers and pikemen in formation
- Customizable formation offset and position

### Construction
- Upgrade islands to maximum level
- Automatically upgrade castle and walls
- Spawn units and mark trees simultaneously

### Unit Recruitment
- Spawn individual units: archers, workers, farmers, pikemen
- Combined recruitment for multiple unit types
- Configurable quantities for each unit type
- Real-time unit count display

### Safety Features
- Automatic backup creation before saving
- Backup files stored with timestamp
- Campaign and island context selector
- Clear status messages for all operations

### Internationalization
- Automatic language detection
- Supported languages: English, French
- Fallback to English for unsupported languages

## Installation

### Download Pre-built Binaries

Download the latest release for your platform from the [Releases](https://github.com/YOUR_USERNAME/ktcedit/releases) page:

- **macOS**: Download the `.dmg` file
- **Windows**: Download the `.msi` or `.exe` installer
- **Linux**: Download the `.deb` or `.AppImage` file

### Build from Source

See [Build Instructions](#building-from-source) below.

## Usage

### Opening a Save File

1. Launch the application
2. The default save file location for your operating system will be displayed:
   - **macOS**: `~/Library/Application Support/nl.noio.kingdom-two-crowns/Release/global-v35`
   - **Windows**: `C:\Users\USERNAME\AppData\LocalLow\noio\KingdomTwoCrowns\Release\global-v35`
   - **Linux**: `~/.config/unity3d/noio/KingdomTwoCrowns/Release/global-v35`
3. Click "Open global-v35" to select your save file
4. The file picker will automatically open to the default location

### Editing Save Data

1. Select the campaign and island you want to edit
2. Navigate through the tabs: Resources, Navigation, Combat, Construction, Recruitment
3. Modify the values as desired
4. Click "Save" to apply changes
5. A backup will be created automatically before saving

### Backup Management

Backups are created automatically in the same directory as your save file with the format:
```
global-v35.backup.YYYY-MM-DD_HH-MM-SS
```

To restore a backup, simply rename it to `global-v35` (after backing up your current save).

## Building from Source

### Prerequisites

- Node.js 20 or higher
- Rust toolchain (install from [rustup.rs](https://rustup.rs/))
- Platform-specific dependencies (see below)

#### macOS
No additional dependencies required.

#### Windows
- Windows 10 SDK
- Visual Studio Build Tools with C++ development tools
- WebView2 Runtime (usually pre-installed on Windows 10/11)

#### Linux (Ubuntu/Debian)
```bash
sudo apt-get update
sudo apt-get install -y \
    libwebkit2gtk-4.1-dev \
    libappindicator3-dev \
    librsvg2-dev \
    patchelf
```

### Build Steps

1. Clone the repository:
   ```bash
   git clone https://github.com/YOUR_USERNAME/ktcedit.git
   cd ktcedit
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

3. Build the application:
   ```bash
   npm run tauri build
   ```

4. Find the built application in `src-tauri/target/release/bundle/`

### Development Mode

Run the application in development mode with hot reload:

```bash
npm run tauri dev
```

### Type Checking

Run TypeScript and Svelte type checking:

```bash
npm run check
```

## Creating Releases with GitHub Actions

This project includes an automated build workflow that creates releases for all platforms.

### Automatic Release Process

1. Ensure all changes are committed and pushed to the main branch

2. Create and push a version tag:
   ```bash
   git tag v0.1.0
   git push origin v0.1.0
   ```

3. GitHub Actions will automatically:
   - Build for macOS (Universal Binary)
   - Build for Windows (MSI and NSIS installer)
   - Build for Linux (DEB and AppImage)
   - Create a draft release with all artifacts
   - Generate changelog from commits since last tag

4. Review the draft release on GitHub and publish when ready

### Manual Release Trigger

You can also trigger builds manually without creating a tag:

1. Go to the "Actions" tab in your GitHub repository
2. Select the "Build and Release" workflow
3. Click "Run workflow"
4. Choose the branch and click "Run workflow"

### Version Numbering

Follow [Semantic Versioning](https://semver.org/):
- **MAJOR** version for incompatible API changes (v1.0.0, v2.0.0)
- **MINOR** version for new functionality in a backward compatible manner (v1.1.0, v1.2.0)
- **PATCH** version for backward compatible bug fixes (v1.0.1, v1.0.2)

Examples:
```bash
# New feature release
git tag v1.1.0
git push origin v1.1.0

# Bug fix release
git tag v1.0.1
git push origin v1.0.1

# Breaking changes
git tag v2.0.0
git push origin v2.0.0
```

## Project Structure

```
ktcedit/
├── src/                        # Frontend source code
│   ├── lib/
│   │   ├── components/        # Svelte components
│   │   ├── i18n.ts           # Internationalization
│   │   └── saveEditActions.ts # Save file manipulation
│   └── routes/
│       └── +page.svelte      # Main application page
├── src-tauri/                 # Rust backend
│   ├── src/
│   │   └── main.rs           # Tauri application entry
│   └── tauri.conf.json       # Tauri configuration
├── .github/
│   └── workflows/
│       └── build.yml         # CI/CD workflow
└── package.json              # Node.js dependencies
```

## Technical Details

### Stack

- **Frontend Framework**: SvelteKit 2.x with Svelte 5
- **Language**: TypeScript 5.x
- **Desktop Framework**: Tauri 2.x
- **Backend Language**: Rust
- **Build Tool**: Vite 6.x
- **Styling**: Custom CSS (dark theme)

### Save File Format

Kingdom Two Crowns uses JSON-based save files with the following structure:
- Campaign data with biome information
- Island data with game objects
- Player data with resources
- Entity spawning system
- Component-based architecture

This editor manipulates the save file structure directly while maintaining compatibility with the game.

## Supported Game Versions

- Kingdom Two Crowns (v35 save format)

Other versions may work but are not officially tested.

## Troubleshooting

### macOS: "App cannot be opened because it is from an unidentified developer"

Right-click the app, select "Open", then click "Open" in the dialog.

Alternatively, go to System Preferences > Security & Privacy and allow the app.

### Windows: "Windows protected your PC" warning

Click "More info" then "Run anyway".

### Linux: AppImage won't run

Make the file executable:
```bash
chmod +x ktcedit_*.AppImage
./ktcedit_*.AppImage
```

### Save file not found

Ensure Kingdom Two Crowns is installed and has been run at least once. The save file is only created after playing the game.

### Changes not appearing in game

Make sure to:
1. Close Kingdom Two Crowns before editing
2. Save changes in the editor
3. Verify the backup was created
4. Restart Kingdom Two Crowns

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development Guidelines

1. Follow the existing code style
2. Add type definitions for new functions
3. Test changes on your platform before submitting
4. Update documentation for user-facing changes
5. Include clear commit messages

### Commit Message Format

Use conventional commits:
```
feat: add support for knight spawning
fix: correct coin calculation for player 2
docs: update installation instructions
chore: update dependencies
```

## License

MIT License - see LICENSE file for details

## Credits

- Original Python implementation: [bitwitch/kingdom-edit](https://github.com/bitwitch/kingdom-edit)
- Kingdom Two Crowns by Noio and Raw Fury

## Disclaimer

This is an unofficial tool. Use at your own risk. Always backup your save files before editing.

The developers of this tool are not affiliated with Noio or Raw Fury.
