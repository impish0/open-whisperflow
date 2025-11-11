# Changelog

All notable changes to Open WhisperFlow will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Phase 2 (v0.2.0) - In Progress

#### Added - Backend
- **Docker Integration** for faster-whisper local transcription
  - Complete Docker container lifecycle management (start, stop, health checks)
  - Automatic NVIDIA GPU detection and CUDA passthrough
  - CPU fallback for systems without GPU
  - Uses fedirz/faster-whisper-server with OpenAI-compatible API
  - Container runs on http://127.0.0.1:8000
- **FasterWhisperBackend** trait implementation
  - Async container initialization
  - Automatic container startup on first transcription
  - 5-minute timeout for long audio files
  - Health checks and readiness detection
- **GPU Detection System**
  - NVIDIA GPU detection via nvidia-smi
  - Automatic CUDA container selection
  - Runtime information exposed to frontend
- **Model Management**
  - Support for 5 Whisper models (tiny, base, small, medium, large)
  - Model information with sizes and descriptions
  - Recommended model highlighting (small)
- **New Tauri Commands**
  - `check_docker_status` - Docker and GPU status
  - `start_whisper_container` - Container management
  - `stop_whisper_container` - Graceful shutdown
  - `get_available_models` - Model list with metadata

#### Added - Frontend
- **Docker Status UI** in Settings Panel
  - Real-time Docker availability monitoring
  - Container running status indicator
  - NVIDIA GPU detection display
  - Status badges (success/warning/error states)
- **Container Management UI**
  - One-click Start/Stop container buttons
  - Loading states during operations
  - User-friendly error messages
- **Model Selection UI**
  - Dropdown with all Whisper models
  - Model size and description display
  - Recommended model indicator
- **Enhanced UX**
  - Warning box when Docker not available
  - Direct link to Docker Desktop download
  - Dark/light mode support for all new components
  - Responsive layout

#### Added - Dependencies
- **bollard 0.17** - Docker API client for Rust
- **futures-util 0.3** - Async stream handling

#### Changed
- TranscriptionService::new() is now async to support Docker initialization
- Settings Panel now loads Docker status on mount
- Enhanced backend selection to show "Cloud" vs "Local - Docker"

#### Technical Details
- OpenAI API-compatible interface for seamless backend switching
- Arc<Mutex<DockerClient>> for safe concurrent access
- Async-first throughout Docker operations
- Type-safe communication between Rust and TypeScript
- Pluggable backend architecture preserved

#### Still TODO for Phase 2
- End-to-end testing with local Docker setup
- Model download progress tracking improvements
- AMD GPU detection (ROCm support)
- Performance benchmarking

---

## [0.1.0] - 2025-11-11

### Added
- **Initial MVP Release** - Complete Phase 1 implementation
- Audio recording system using cpal (cross-platform)
- OpenAI Whisper API transcription integration
- OpenAI GPT API for text rewriting
- Ollama API framework (ready for local LLM usage)
- Text injection with three methods: Clipboard, Typing, Hybrid
- Type-safe configuration system with platform-specific storage
- Modern React UI with recording button and status indicator
- Comprehensive settings panel for all configurations
- Three built-in prompt templates: Minimal, Balanced, Professional
- Real-time recording state management
- Error handling with user-friendly messages
- Dark/light mode support with system theme detection
- Development infrastructure:
  - ESLint + Prettier configuration
  - TypeScript strict mode
  - Rust clippy + rustfmt
  - Vitest testing framework
  - GitHub Actions CI/CD pipeline
- Comprehensive documentation:
  - README.md with user guide
  - ARCHITECTURE.md with technical details
  - COMPREHENSIVE_PLAN.md with 20-week roadmap
  - PROMPT_TEMPLATES.md with template guide
  - CLAUDE.md with AI development guidelines
  - config.example.toml with full config reference

### Technical Details
- **Backend**: Tauri v2.1, Rust 1.91
- **Frontend**: React 18, TypeScript 5, Vite 5
- **Audio**: cpal 0.15, hound 3.5
- **HTTP Client**: reqwest 0.12
- **Keyboard**: enigo 0.2
- **Clipboard**: arboard 3
- **Config**: confy 0.6
- **Async**: tokio 1.x

### Architecture
- Pluggable backend system for easy extensibility
- Unified LLM client supporting OpenAI and Ollama APIs
- Type-safe end-to-end (Rust + TypeScript)
- Async throughout for non-blocking operations
- Clean module separation (11 Rust modules, 5 React components)

### Known Limitations
- Global hotkeys not implemented (planned for Phase 4)
- faster-whisper not implemented (planned for Phase 2)
- VAD auto-stop not implemented (planned for Phase 4)
- No transcription history (planned for Phase 5)
- Single prompt template active at a time (multiple planned for Phase 3)
- Requires internet for transcription/rewriting in MVP

---

## Version History

- **v0.1.0** (2025-11-11) - Phase 1 MVP Complete
- *Future versions will be listed here*

---

## Categories Explained

### Added
New features or functionality added to the project.

### Changed
Changes to existing functionality or behavior.

### Deprecated
Features that are being phased out (will be removed in future).

### Removed
Features that have been completely removed.

### Fixed
Bug fixes and corrections.

### Security
Security-related improvements or fixes.

---

## Development Phases

- **Phase 1 (v0.1.0)**: MVP with cloud APIs ✅
- **Phase 2 (v0.2.0)**: Local Whisper support 🚧
- **Phase 3 (v0.3.0)**: Local LLM and prompt templates
- **Phase 4 (v0.4.0)**: UX polish and hotkeys
- **Phase 5 (v0.5.0)**: Advanced features
- **Phase 6 (v1.0.0)**: Production release

---

**Note**: This changelog is maintained manually. All changes should be documented here before release.
