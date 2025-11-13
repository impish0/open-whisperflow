# Open WhisperFlow 🎤✨

**Privacy-first, open-source voice-to-text with AI-powered text refinement.**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Version](https://img.shields.io/badge/version-0.1.0--dev-blue.svg)](CHANGELOG.md)
[![Phase](https://img.shields.io/badge/phase-3%20complete-green.svg)](TODO.md)
[![Build Status](https://img.shields.io/badge/build-source--only-orange.svg)](#installation)

Transform your speech into polished, written content across any application on Windows and Linux. No subscriptions, no cloud lock-in, complete control over your data.

> **📖 Documentation**: [User Guide](#-quick-start) • [Contributing](CONTRIBUTING.md) • [Roadmap](COMPREHENSIVE_PLAN.md) • [TODO](TODO.md) • [Changelog](CHANGELOG.md)

---

## 📚 Documentation Hub

### For Users
- **[Installation Guide](#installation)** - Get up and running
- **[Configuration](#%EF%B8%8F-configuration)** - Set up local or cloud backends
- **[Usage](#-usage)** - How to use the app
- **[Troubleshooting](#-troubleshooting)** - Common issues and solutions
- **[CHANGELOG.md](CHANGELOG.md)** - Version history

### For Contributors & Developers
- **[CONTRIBUTING.md](CONTRIBUTING.md)** - How to contribute
- **[CLAUDE.md](CLAUDE.md)** - **START HERE for AI/developers** - Architecture, patterns, standards
- **[TODO.md](TODO.md)** - Tasks, bugs, and features
- **[ARCHITECTURE.md](ARCHITECTURE.md)** - Technical architecture
- **[Issue Templates](.github/ISSUE_TEMPLATE/)** - Report bugs or request features

### Planning & Vision
- **[COMPREHENSIVE_PLAN.md](COMPREHENSIVE_PLAN.md)** - 20-week development roadmap
- **[PROMPT_TEMPLATES.md](PROMPT_TEMPLATES.md)** - Prompt engineering guide
- **[IMPLEMENTATION_SUMMARY.md](IMPLEMENTATION_SUMMARY.md)** - What's been built

> **🤖 For AI Assistants**: Read [CLAUDE.md](CLAUDE.md) first! It contains critical context about project structure, development patterns, and documentation standards.

---

## ✨ Features

- **🔒 Privacy First**: 100% local processing option - your voice never leaves your machine
- **🚀 Fast & Accurate**: Sub-2-second end-to-end processing with quality transcription
- **🎯 Smart Rewriting**: AI-powered text cleanup removes filler words, fixes grammar, adjusts tone
- **⌨️ Universal Input**: Works in ANY application - email, documents, chat, code editors
- **🔧 Fully Customizable**: Custom prompt templates, multiple models, flexible configurations
- **💰 Zero Cost**: No subscriptions, no usage limits (optional cloud APIs at your discretion)
- **🌐 Cross-Platform**: Windows and Linux support with first-class Linux integration
- **🔓 Open Source**: Fully transparent, auditable, and extensible

---

## 🎯 Quick Start

### Prerequisites

Choose your path:

**Option A: Cloud (Easiest - Start in 5 minutes)**
- OpenAI API key ([get one here](https://platform.openai.com/api-keys))
- No local setup needed

**Option B: Local (Privacy-focused - 15-30 minutes setup)**
- Docker Desktop (recommended) OR Python 3.9+
- Ollama (for local LLM) - [install here](https://ollama.ai)
- Optional: NVIDIA GPU with CUDA for faster processing

### Installation

> **⚠️ Note**: Pre-built installers are not yet available. Please build from source using the instructions below.
>
> **Coming Soon**: `.exe` (Windows), `.AppImage` / `.deb` (Linux) installers will be available in future releases.

#### Build from Source

**Prerequisites:**
- Node.js 22+ ([download here](https://nodejs.org/))
- Rust 1.91+ ([install here](https://rustup.rs/))
- System dependencies (see below)

**Linux System Dependencies:**
```bash
# Debian/Ubuntu
sudo apt-get update
sudo apt-get install -y \
  libwebkit2gtk-4.1-dev \
  libappindicator3-dev \
  librsvg2-dev \
  patchelf \
  libasound2-dev \
  libgtk-3-dev \
  libxdo-dev

# Fedora/RHEL
sudo dnf install webkit2gtk4.1-devel \
  gtk3-devel \
  librsvg2-devel \
  alsa-lib-devel \
  libxdo-devel

# Arch Linux
sudo pacman -S webkit2gtk \
  gtk3 \
  librsvg \
  patchelf \
  alsa-lib \
  xdotool
```

**Windows Prerequisites:**
1. Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/) (2022 or later)
   - Select "Desktop development with C++"
   - Include "MSVC v143" and "Windows 10/11 SDK"
2. Install [WebView2 Runtime](https://developer.microsoft.com/en-us/microsoft-edge/webview2/) (usually pre-installed on Windows 11)

**Build Steps:**
```bash
# Clone the repository
git clone https://github.com/yourusername/open-whisperflow.git
cd open-whisperflow

# Install dependencies
npm install

# Run in development mode (with hot reload)
npm run tauri:dev

# OR build for production
npm run tauri:build
```

**Built installers will be in:**
- Linux: `src-tauri/target/release/bundle/appimage/` or `src-tauri/target/release/bundle/deb/`
- Windows: `src-tauri/target/release/bundle/nsis/`

For detailed development setup, see [DEV_SETUP.md](DEV_SETUP.md).

---

## 🚀 Usage

### Basic Workflow

1. **Press Hotkey** (default: `Ctrl+Shift+Space`)
2. **Speak** (indicator shows you're recording)
3. **Press Hotkey Again** to stop
4. **Wait** (~2 seconds) while AI transcribes and refines
5. **Text Appears** at your cursor position automatically

### Example

You say:
> "um so like I need to uh schedule a meeting for next Tuesday"

You get:
> "I need to schedule a meeting for next Tuesday"

---

## ⚙️ Configuration

### Local Setup (Privacy Mode)

#### 1. Install Whisper Backend

**Option A: Docker (Recommended)**

```bash
# Pull faster-whisper with GPU support
docker pull ghcr.io/ggml-org/whisper.cpp:main-cuda

# Or CPU-only
docker pull ghcr.io/ggml-org/whisper.cpp:main

# The app will auto-detect and use Docker
```

**Option B: Python**

```bash
# Create virtual environment
python -m venv whisper-env
source whisper-env/bin/activate  # On Windows: whisper-env\Scripts\activate

# Install faster-whisper
pip install faster-whisper

# With GPU support (NVIDIA)
pip install faster-whisper[gpu]
```

#### 2. Install Ollama (Local LLM)

```bash
# Linux
curl -fsSL https://ollama.com/install.sh | sh

# Windows
# Download from https://ollama.com/download

# Pull a model (recommended: llama3.2 3B)
ollama pull llama3.2:3b

# Or for higher quality (slower)
ollama pull mistral:7b
```

#### 3. Configure App

Open Settings in Open WhisperFlow:

- **Transcription Backend**: `faster-whisper (Docker)` or `faster-whisper (Python)`
- **Whisper Model**: `small` (recommended balance) or `tiny` (faster) or `medium` (more accurate)
- **LLM Backend**: `Ollama`
- **LLM Model**: `llama3.2:3b`

### Cloud Setup (Quick Start)

Open Settings:

- **Transcription Backend**: `OpenAI Whisper API`
- **OpenAI API Key**: `sk-...` (your key)
- **LLM Backend**: `OpenAI`
- **LLM Model**: `gpt-4o-mini` (cost-effective) or `gpt-4o` (best quality)

**Cost Estimate**: ~$0.10 per hour of transcription with GPT-4o-mini

---

## 🎨 Customization

### Prompt Templates

Open WhisperFlow includes several built-in templates:

- **Minimal**: Just remove filler words, preserve natural speech
- **Balanced** (default): Light cleanup and formatting
- **Professional**: Formal tone for business communication
- **Casual**: Conversational tone for chat/messaging
- **Technical**: For code comments and technical writing

### Custom Templates

Create your own in Settings > Prompt Templates:

```
You are a text refinement assistant.

Task: Clean up the following voice transcription.

Rules:
- Remove filler words (um, uh, like)
- Fix obvious grammar mistakes
- Preserve the original meaning
- Keep the casual tone
- Do NOT summarize or rewrite completely

Transcription: {text}

Cleaned text:
```

Variables available:
- `{text}` - The raw transcription
- `{context}` - Detected application name
- `{language}` - Detected language

### Hotkeys

Customize in Settings > Hotkeys:

- **Recording Toggle**: Default `Ctrl+Shift+Space`
- **Cancel Recording**: Default `Esc`
- **Open Settings**: Default `Ctrl+Shift+,`

---

## 🔍 Troubleshooting

### Microphone Not Working

**Windows**:
1. Settings > Privacy > Microphone
2. Ensure "Allow apps to access your microphone" is ON
3. Allow Open WhisperFlow specifically

**Linux**:
```bash
# Check microphone devices
arecord -l

# Test recording
arecord -d 5 test.wav
aplay test.wav
```

### Docker Not Detected

```bash
# Check Docker is running
docker ps

# If not running, start Docker Desktop (Windows)
# Or start Docker service (Linux)
sudo systemctl start docker
```

### Ollama Not Responding

```bash
# Check if Ollama is running
curl http://localhost:11434

# Start Ollama service (Linux)
ollama serve

# Windows: Ensure Ollama is running from system tray
```

### Text Injection Fails

- **Wayland Users**: Some apps may block automation. Try:
  - Settings > Text Injection > Method: `Clipboard`
  - Or use X11 session temporarily
  
- **Windows**: Run as administrator if injecting into elevated apps

### Slow Processing

- Use smaller Whisper model (`tiny` or `base`)
- Ensure GPU acceleration is working:
  ```bash
  # Check NVIDIA GPU
  nvidia-smi
  
  # Check Docker can access GPU
  docker run --gpus all nvidia/cuda:11.8.0-base-ubuntu22.04 nvidia-smi
  ```

---

## 📊 Model Comparison

| Model | Size | Speed (GPU) | Accuracy | Recommended For |
|-------|------|-------------|----------|-----------------|
| tiny | 39M | ~2000 WPM | Good | Testing, very fast responses |
| base | 74M | ~1500 WPM | Better | Quick drafts, notes |
| small | 244M | ~1000 WPM | Great | **Default - best balance** |
| medium | 769M | ~600 WPM | Excellent | Professional use, accuracy critical |
| large | 1550M | ~400 WPM | Best | Maximum accuracy, longer audio |

**Recommendation**: Start with `small` - it provides excellent accuracy with good speed. Upgrade to `medium` only if you need higher accuracy and have a good GPU.

---

## 🤝 Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

**Ways to Contribute**:
- 🐛 Report bugs and issues
- 💡 Suggest features
- 📝 Improve documentation
- 🔧 Submit pull requests
- 🎨 Share prompt templates
- 🌍 Add translations

---

## 🛣️ Roadmap

### Phase 1 ✅ Complete
- [x] Basic voice recording with cpal
- [x] OpenAI Whisper API transcription
- [x] GPT-4 text rewriting
- [x] Text injection (clipboard, typing, hybrid)
- [x] Basic settings UI
- [x] Configuration system

### Phase 2 ✅ Complete
- [x] Docker integration for local transcription
- [x] faster-whisper backend
- [x] GPU detection (NVIDIA CUDA)
- [x] Model selection UI
- [x] Automatic container management

### Phase 2.5 ✅ Complete
- [x] First-run wizard
- [x] Consumer-friendly onboarding
- [x] Cloud vs Local setup paths

### Phase 3 ✅ Complete
- [x] Ollama integration
- [x] Local LLM backend
- [x] Model management UI
- [x] 100% offline pipeline

### Phase 4 (Next)
- [ ] Global hotkeys
- [ ] VAD auto-stop
- [ ] System tray integration
- [ ] Notification system

### Phase 5 (Future)
- [ ] Voice shortcuts
- [ ] Transcription history
- [ ] Multi-language support
- [ ] Context detection

### v1.0 (Future)
- [ ] Pre-built installers
- [ ] Auto-updater
- [ ] CLI interface
- [ ] Plugin system

**Current Status**: Phase 3 complete - Full local pipeline working!

See [COMPREHENSIVE_PLAN.md](COMPREHENSIVE_PLAN.md) for detailed roadmap.

---

## 📖 Documentation

- [Comprehensive Plan](COMPREHENSIVE_PLAN.md) - Full project plan and research
- [Architecture](ARCHITECTURE.md) - Technical architecture details
- [Prompt Templates](docs/PROMPT_TEMPLATES.md) - Guide to creating templates
- [Development Guide](docs/DEVELOPMENT.md) - For contributors

---

## 💬 Community & Support

- **GitHub Issues**: [Report bugs or request features](https://github.com/impish0/open-whisperflow/issues)
- **GitHub Discussions**: [Ask questions, share ideas](https://github.com/impish0/open-whisperflow/discussions)

> **Note**: Discord and social media channels coming soon!

---

## 📜 License

Open WhisperFlow is released under the [MIT License](LICENSE).

**Key Points**:
- ✅ Free for personal and commercial use
- ✅ Modify and distribute freely
- ✅ No warranty provided
- ⚠️ Include license and copyright notice

---

## 🙏 Acknowledgments

Built with these incredible open-source projects:

- [Tauri](https://tauri.app/) - Cross-platform app framework
- [OpenAI Whisper](https://github.com/openai/whisper) - Speech recognition
- [faster-whisper](https://github.com/SYSTRAN/faster-whisper) - Optimized Whisper
- [Ollama](https://ollama.ai/) - Local LLM runtime
- [Silero VAD](https://github.com/snakers4/silero-vad) - Voice activity detection
- And many more - see [ACKNOWLEDGMENTS.md](ACKNOWLEDGMENTS.md)

---

## ⚠️ Disclaimer

Open WhisperFlow is an independent open-source project and is not affiliated with, endorsed by, or sponsored by Wispr AI Inc. or their product WisprFlow. All trademarks are property of their respective owners.

---

**Made with ❤️ by the Open WhisperFlow community**

If you find this project useful, please star the repo! ⭐
