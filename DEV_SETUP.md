# Development Setup Guide

Complete guide for setting up Open WhisperFlow development environment on Windows (WSL or native) and Linux.

---

## Table of Contents

- [Prerequisites](#prerequisites)
- [Option 1: WSL Setup (Recommended for Windows)](#option-1-wsl-setup-recommended-for-windows)
- [Option 2: Native Windows Setup](#option-2-native-windows-setup)
- [Option 3: Native Linux Setup](#option-3-native-linux-setup)
- [Post-Installation Steps](#post-installation-steps)
- [Development Workflow](#development-workflow)
- [Troubleshooting](#troubleshooting)

---

## Prerequisites

### Required Knowledge

- Basic command line usage
- Git basics
- Text editor or IDE (VS Code recommended)

### Time Requirements

- **WSL Setup**: 30-45 minutes
- **Native Windows**: 45-60 minutes (Visual Studio takes time)
- **Native Linux**: 20-30 minutes

---

## Option 1: WSL Setup (Recommended for Windows)

**Best for**: Development, testing, CI/CD pipeline work

**Pros**:
- Better tooling support
- Faster builds
- Cleaner dependencies
- Closer to production Linux environment

**Cons**:
- Requires Windows 10/11
- UI apps need display server setup (WSL2 handles this automatically on Windows 11)

### Step 1: Install WSL2

```powershell
# Run in PowerShell as Administrator
wsl --install -d Ubuntu-22.04

# Restart your computer when prompted
```

After restart, Ubuntu will open automatically:
- Create a username (lowercase, no spaces)
- Create a password (you won't see it typing, this is normal)

### Step 2: Install System Dependencies

Open Ubuntu from Start Menu and run:

```bash
# Update package lists
sudo apt-get update

# Install Tauri system dependencies
sudo apt-get install -y \
  libwebkit2gtk-4.1-dev \
  libappindicator3-dev \
  librsvg2-dev \
  patchelf \
  build-essential \
  curl \
  wget \
  file \
  libssl-dev \
  libgtk-3-dev \
  libayatana-appindicator3-dev \
  pkg-config

# Install audio dependencies
sudo apt-get install -y \
  libasound2-dev \
  portaudio19-dev

# Install text injection dependency
sudo apt-get install -y \
  libxdo-dev
```

### Step 3: Install Rust

```bash
# Install Rust via rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Choose option 1 (default installation)

# Load Rust into current session
source "$HOME/.cargo/env"

# Verify installation
rustc --version  # Should show 1.91+ or newer
cargo --version
```

### Step 4: Install Node.js 22+

```bash
# Install nvm (Node Version Manager)
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash

# Reload shell configuration
source ~/.bashrc

# Install Node.js 22
nvm install 22
nvm use 22
nvm alias default 22

# Verify installation
node --version  # Should show v22.x.x
npm --version   # Should show 10.x.x+
```

### Step 5: Clone and Setup Project

```bash
# Navigate to your preferred directory
cd ~

# Clone the repository
git clone https://github.com/impish0/open-whisperflow.git
cd open-whisperflow

# Checkout the development branch (if working on a feature)
git checkout claude/superthink-competitor-plan-011CV1dSDm2PktPYbBsbM28U

# Install Node dependencies
npm install

# This will take a few minutes...
```

### Step 6: Run Development Server

```bash
# Start the app in development mode
npm run tauri:dev

# First build will take 5-10 minutes (compiling Rust dependencies)
# Subsequent builds are much faster (30-60 seconds)
```

### WSL Display Setup (Windows 10 only)

If you're on **Windows 11**, WSLg handles display automatically. Skip this section.

If you're on **Windows 10** and get display errors:

**Option A: Install VcXsrv (Free)**

1. Download [VcXsrv](https://sourceforge.net/projects/vcxsrv/)
2. Install with defaults
3. Run XLaunch:
   - Multiple windows
   - Start no client
   - **Disable access control** (important!)

4. In WSL terminal:
```bash
export DISPLAY=$(cat /etc/resolv.conf | grep nameserver | awk '{print $2}'):0
```

**Option B: Install X410 (Paid, $10)**

1. Install from Microsoft Store
2. Run X410
3. In WSL:
```bash
export DISPLAY=:0
```

Add to `~/.bashrc` to make permanent:
```bash
echo 'export DISPLAY=$(cat /etc/resolv.conf | grep nameserver | awk "{print \$2}"):0' >> ~/.bashrc
```

---

## Option 2: Native Windows Setup

**Best for**: UI development, releases, avoiding WSL complexity

**Pros**:
- No display server needed
- Native Windows performance
- Simpler for GUI apps

**Cons**:
- More dependencies to install manually
- Larger installation size (~10GB)

### Step 1: Install Visual Studio Build Tools

Required for compiling native Node modules and Rust dependencies.

1. Download [Visual Studio Build Tools 2022](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022)
2. Run `vs_BuildTools.exe`
3. In the installer:
   - Select **"Desktop development with C++"** workload
   - Ensure these are checked:
     - MSVC v143 - VS 2022 C++ x64/x86 build tools
     - Windows 10/11 SDK
     - C++ CMake tools for Windows
4. Click **Install** (requires ~7GB disk space)
5. Wait 20-30 minutes for installation
6. **Restart your computer**

### Step 2: Install Rust

1. Download [rustup-init.exe](https://win.rustup.rs/)
2. Run the installer
3. Press Enter to accept defaults
4. Wait for installation to complete
5. Close and reopen your terminal

**Verify installation**:
```powershell
rustc --version  # Should show 1.91.0 or newer
cargo --version  # Should show 1.91.0 or newer
```

### Step 3: Install Node.js

1. Download [Node.js 22 LTS](https://nodejs.org/en/download/) (Windows Installer .msi)
2. Run installer
3. Accept license
4. Accept default installation location
5. **Check "Automatically install necessary tools"** (optional but helpful)
6. Click Install
7. Complete installation

**Verify installation**:
```powershell
node --version  # Should show v22.x.x
npm --version   # Should show 10.x.x+
```

### Step 4: Install Git (if not already installed)

1. Download [Git for Windows](https://git-scm.com/download/win)
2. Run installer
3. Accept all defaults (Git Bash + PATH integration)
4. Complete installation

**Verify installation**:
```powershell
git --version  # Should show git version 2.x.x
```

### Step 5: Clone and Setup Project

Open **PowerShell** or **Git Bash**:

```powershell
# Navigate to your projects directory
cd C:\Users\YourUsername\Projects  # or wherever you prefer
# If the directory doesn't exist:
mkdir C:\Users\YourUsername\Projects
cd C:\Users\YourUsername\Projects

# Clone the repository
git clone https://github.com/impish0/open-whisperflow.git
cd open-whisperflow

# Checkout development branch (if applicable)
git checkout claude/superthink-competitor-plan-011CV1dSDm2PktPYbBsbM28U

# Install Node dependencies
npm install

# This will take 3-5 minutes...
```

### Step 6: Run Development Server

```powershell
# Start development server
npm run tauri:dev

# First build takes 5-10 minutes (compiling Rust)
# App window will open automatically when ready
```

---

## Option 3: Native Linux Setup

**Best for**: Linux users, production builds, CI/CD

### Step 1: Install System Dependencies

#### Ubuntu/Debian:

```bash
sudo apt-get update

sudo apt-get install -y \
  libwebkit2gtk-4.1-dev \
  libappindicator3-dev \
  librsvg2-dev \
  patchelf \
  libasound2-dev \
  build-essential \
  curl \
  wget \
  file \
  libssl-dev \
  libgtk-3-dev \
  libayatana-appindicator3-dev \
  pkg-config \
  portaudio19-dev \
  libxdo-dev
```

#### Fedora:

```bash
sudo dnf install -y \
  webkit2gtk4.1-devel \
  openssl-devel \
  curl \
  wget \
  file \
  libappindicator-gtk3-devel \
  librsvg2-devel \
  alsa-lib-devel \
  gtk3-devel \
  patchelf \
  libxdo-devel
```

#### Arch Linux:

```bash
sudo pacman -Syu

sudo pacman -S --needed \
  webkit2gtk-4.1 \
  base-devel \
  curl \
  wget \
  file \
  openssl \
  gtk3 \
  libappindicator-gtk3 \
  librsvg \
  alsa-lib \
  patchelf \
  xdotool
```

### Step 2: Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Choose option 1 (default)

source "$HOME/.cargo/env"

# Verify
rustc --version
cargo --version
```

### Step 3: Install Node.js

Using nvm (recommended):

```bash
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash

source ~/.bashrc  # or ~/.zshrc if using zsh

nvm install 22
nvm use 22
nvm alias default 22

# Verify
node --version
npm --version
```

Or using system package manager:

```bash
# Ubuntu/Debian
curl -fsSL https://deb.nodesource.com/setup_22.x | sudo -E bash -
sudo apt-get install -y nodejs

# Fedora
sudo dnf install -y nodejs npm

# Arch
sudo pacman -S nodejs npm
```

### Step 4: Clone and Setup

```bash
cd ~
git clone https://github.com/impish0/open-whisperflow.git
cd open-whisperflow

# Checkout development branch if needed
git checkout claude/superthink-competitor-plan-011CV1dSDm2PktPYbBsbM28U

npm install
```

### Step 5: Run Development Server

```bash
npm run tauri:dev
```

---

## Post-Installation Steps

### 1. Verify Installation

After running `npm run tauri:dev`, you should see:

1. Terminal output showing compilation progress
2. A window opening with the Open WhisperFlow UI
3. Settings panel, recording button, and status indicators

### 2. Configure API Keys (Optional for Testing)

To test with OpenAI APIs:

1. Get an API key from [OpenAI Platform](https://platform.openai.com/api-keys)
2. Open the app settings
3. Enter your API key
4. Select "OpenAI Whisper API" as transcription backend
5. Select "OpenAI" as LLM backend

**Cost estimate**: ~$0.10 per hour of audio with GPT-4o-mini

### 3. Set Up Local Backends (Optional - for Privacy Mode)

#### Install Ollama (Local LLM):

**Windows**:
1. Download from [ollama.com/download](https://ollama.com/download)
2. Run installer
3. Ollama runs in system tray

**Linux/WSL**:
```bash
curl -fsSL https://ollama.com/install.sh | sh
```

**Pull a model**:
```bash
ollama pull llama3.2:3b  # Fast, 3B parameters
# OR
ollama pull mistral:7b   # Better quality, slower
```

**In app settings**:
- LLM Backend: Ollama
- LLM Model: llama3.2:3b

#### Install faster-whisper (Local Transcription):

Coming in Phase 2! For now, use OpenAI Whisper API or wait for upcoming Docker integration.

---

## Development Workflow

### Daily Development

```bash
# Start dev server (hot reload enabled)
npm run tauri:dev

# In another terminal, run linters as you work:
npm run lint          # Check for issues
npm run type-check    # TypeScript validation
```

### Before Committing

```bash
# Run full checks
npm run lint
npm run type-check
npm run format

# Rust checks
cd src-tauri
cargo fmt
cargo clippy
cd ..

# Run tests (when available)
npm test
```

### Building for Production

```bash
# Build release version
npm run tauri:build

# Output locations:
# Windows: src-tauri/target/release/bundle/nsis/
# Linux: src-tauri/target/release/bundle/appimage/
# macOS: src-tauri/target/release/bundle/dmg/
```

### Useful Commands

```bash
# Frontend only (no Tauri)
npm run dev

# Type checking without building
npm run type-check

# Fix linting issues automatically
npm run lint:fix

# Format all code
npm run format

# Clean Rust build cache
cd src-tauri && cargo clean && cd ..

# Update dependencies
npm update
cd src-tauri && cargo update && cd ..
```

---

## Troubleshooting

### `npm install` fails with gyp errors

**Windows**:
```powershell
npm install --global windows-build-tools
npm install
```

**Linux/WSL**:
```bash
sudo apt-get install -y build-essential
npm install
```

### Rust compilation fails with "linker not found"

**Windows**: Ensure Visual Studio Build Tools are installed with C++ workload

**Linux**:
```bash
sudo apt-get install build-essential
# or on Fedora
sudo dnf groupinstall "Development Tools"
```

### "WebView2 not found" (Windows only)

Install [Microsoft Edge WebView2 Runtime](https://developer.microsoft.com/en-us/microsoft-edge/webview2/)

### App window doesn't open (WSL)

Check display server:
```bash
echo $DISPLAY  # Should show something like "172.x.x.x:0"

# If empty, set it:
export DISPLAY=$(cat /etc/resolv.conf | grep nameserver | awk '{print $2}'):0

# Try again
npm run tauri:dev
```

### Audio recording doesn't work

**Windows**:
1. Settings > Privacy & Security > Microphone
2. Enable "Let apps access your microphone"
3. Enable for your terminal/app

**Linux**:
```bash
# Check microphone devices
arecord -l

# Test recording
arecord -d 5 test.wav
aplay test.wav
```

### Build is very slow

**First build**: 5-10 minutes is normal (compiling all Rust dependencies)

**Subsequent builds**: Should be 30-60 seconds

**If always slow**:
```bash
# Clean and rebuild
cd src-tauri
cargo clean
cd ..
npm run tauri:dev
```

### "error: failed to run custom build command"

Usually means missing system dependencies.

**Check you installed all dependencies from Step 1/2 above**

### Port already in use

```bash
# Find and kill process using port 1420 (Vite default)
# Windows:
netstat -ano | findstr :1420
taskkill /PID <PID> /F

# Linux/WSL:
lsof -ti:1420 | xargs kill -9
```

### TypeScript errors in IDE

```bash
# Rebuild TypeScript cache
npm run type-check

# If using VS Code, reload window:
# Ctrl+Shift+P > "Developer: Reload Window"
```

### Import errors for Tauri APIs

```bash
# Ensure Tauri packages are installed
npm install @tauri-apps/api @tauri-apps/plugin-shell

# Clear node_modules and reinstall
rm -rf node_modules package-lock.json
npm install
```

---

## IDE Setup Recommendations

### VS Code (Recommended)

**Install extensions**:
- rust-analyzer (Rust language support)
- Tauri (Tauri app development)
- ESLint (JavaScript linting)
- Prettier (Code formatting)
- Error Lens (Inline errors)

**Settings** (`.vscode/settings.json`):
```json
{
  "rust-analyzer.checkOnSave.command": "clippy",
  "editor.formatOnSave": true,
  "editor.defaultFormatter": "esbenp.prettier-vscode",
  "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer"
  }
}
```

### Other IDEs

- **IntelliJ IDEA / WebStorm**: Install Rust plugin
- **Vim/Neovim**: Install rust-analyzer LSP
- **Sublime Text**: Install Rust Enhanced package

---

## Next Steps

1. **Read Documentation**:
   - [CLAUDE.md](CLAUDE.md) - Architecture & development guide
   - [CONTRIBUTING.md](CONTRIBUTING.md) - Contribution guidelines
   - [TODO.md](TODO.md) - Current tasks and roadmap

2. **Explore the Codebase**:
   - `src/` - React frontend (TypeScript)
   - `src-tauri/` - Rust backend
   - `src-tauri/src/commands.rs` - Tauri commands (API layer)

3. **Make Your First Change**:
   - Try modifying the UI in `src/App.tsx`
   - Hot reload will show changes instantly

4. **Join the Community**:
   - Check GitHub issues
   - Read the roadmap in [TODO.md](TODO.md)
   - Submit your first PR!

---

## Getting Help

- **Documentation Issues**: Open an issue on GitHub
- **Build Problems**: Check this guide's troubleshooting section
- **Feature Questions**: See [CLAUDE.md](CLAUDE.md) for architecture details
- **Contributing**: Read [CONTRIBUTING.md](CONTRIBUTING.md)

---

## Summary

You should now have:
- ✅ All dependencies installed (Rust, Node.js, system libraries)
- ✅ Project cloned and dependencies installed
- ✅ Development server running (`npm run tauri:dev`)
- ✅ App window displaying Open WhisperFlow UI

**Happy coding!** 🚀

---

**Last Updated**: 2025-11-11
**Maintained by**: Open WhisperFlow Community
