# Quick Start Guide

This guide will help you get your Kingdom Two Crowns Save Editor project ready for distribution.

## Current Status

- Application built successfully for macOS (Apple Silicon)
- Multi-language support implemented (English/French)
- All features implemented and tested
- GitHub Actions workflow ready for cross-platform builds
- Documentation complete

## Built Application Location

Your macOS build is available at:
```
src-tauri/target/release/bundle/macos/ktcedit.app
src-tauri/target/release/bundle/dmg/ktcedit_0.1.0_aarch64.dmg
```

## Next Steps

### 1. Test the Application Locally

Test the macOS build:
```bash
open "src-tauri/target/release/bundle/macos/ktcedit.app"
```

### 2. Initialize Git Repository (if not already done)

```bash
# Initialize git
git init

# Add all files
git add .

# Create initial commit
git commit -m "feat: initial release of Kingdom Two Crowns Save Editor

- Resource management (coins, gems)
- Island navigation and fast travel
- Unit spawning system
- Combat formations
- Construction upgrades
- Multi-language support (EN/FR)
- Automatic backup creation
- Cross-platform support"
```

### 3. Create GitHub Repository

1. Go to https://github.com/new
2. Create a new repository (name it `ktcedit` or similar)
3. Do NOT initialize with README (we already have one)
4. Copy the repository URL

### 4. Push to GitHub

```bash
# Add remote
git remote add origin https://github.com/YOUR_USERNAME/ktcedit.git

# Push code
git branch -M main
git push -u origin main
```

### 5. Create Your First Release

Create and push a version tag to trigger automated builds:

```bash
# Create version tag
git tag v0.1.0

# Push tag to GitHub
git push origin v0.1.0
```

This will automatically:
- Build for macOS (Universal Binary)
- Build for Windows (MSI + NSIS installer)
- Build for Linux (DEB + AppImage)
- Create a draft release with changelog
- Upload all build artifacts

### 6. Review and Publish Release

1. Go to your GitHub repository
2. Click on "Releases" tab
3. You'll see a draft release
4. Review the changelog and build artifacts
5. Click "Publish release"

## Manual Build for Other Platforms

### Windows Build

From a Windows machine:
```bash
git clone https://github.com/YOUR_USERNAME/ktcedit.git
cd ktcedit
npm install
npm run tauri build
```

### Linux Build

From Ubuntu/Debian:
```bash
# Install dependencies
sudo apt-get update
sudo apt-get install -y libwebkit2gtk-4.1-dev \
    build-essential curl wget file libxdo-dev \
    libssl-dev libayatana-appindicator3-dev \
    librsvg2-dev patchelf

# Build
git clone https://github.com/YOUR_USERNAME/ktcedit.git
cd ktcedit
npm install
npm run tauri build
```

## Updating the README

Don't forget to update the placeholder in README.md:
- Replace `YOUR_USERNAME` with your actual GitHub username

## Version Numbering

For future releases, follow semantic versioning:

- **Patch** (v0.1.1): Bug fixes
  ```bash
  git tag v0.1.1
  git push origin v0.1.1
  ```

- **Minor** (v0.2.0): New features
  ```bash
  git tag v0.2.0
  git push origin v0.2.0
  ```

- **Major** (v1.0.0): Breaking changes
  ```bash
  git tag v1.0.0
  git push origin v1.0.0
  ```

## Troubleshooting

### GitHub Actions Fails

If the automated build fails:
1. Check the Actions tab in your repository
2. Click on the failed workflow
3. Review the error logs
4. Common issues:
   - Missing secrets: GITHUB_TOKEN is automatically provided
   - Platform-specific dependencies: Check the workflow logs
   - Build errors: Fix locally first, then push

### Build Artifacts Missing

If some build artifacts are missing from the release:
1. Check that all three platform builds completed
2. The workflow creates a draft - artifacts may take a few minutes
3. Refresh the release page

### Tag Already Exists

If you need to recreate a tag:
```bash
# Delete local tag
git tag -d v0.1.0

# Delete remote tag
git push origin :refs/tags/v0.1.0

# Create new tag
git tag v0.1.0

# Push new tag
git push origin v0.1.0
```

## Getting Help

- Check [README.md](README.md) for full documentation
- Check [CONTRIBUTING.md](CONTRIBUTING.md) for development guidelines
- Open an issue on GitHub if you encounter problems

## What's Next?

After your first release:
1. Share your project with the community
2. Gather user feedback
3. Plan new features
4. Keep the changelog updated
5. Respond to issues and pull requests

Good luck with your release!
