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

### Phase 3 (Local LLM Integration) - Added 2025-11-11

#### Added - Backend
- **Ollama Status Management**
  - `check_ollama_status` - Real-time Ollama availability detection
  - `get_ollama_models` - List installed local models with metadata
  - `get_recommended_ollama_models` - Curated model suggestions
- **Ollama Model Discovery**
  - Automatic model detection via Ollama API
  - Model size and last modified tracking
  - Recommended model highlighting (llama3.2:3b, llama3:8b, mistral:7b)
- **Response Types**
  - OllamaStatus struct with availability and connection info
  - OllamaModelInfo struct with size and recommendation flags

#### Added - Frontend
- **Ollama Status UI** in Settings Panel
  - Real-time Ollama service monitoring
  - Connection status indicator
  - Base URL display
- **Model Management UI**
  - Dropdown with installed models (when available)
  - Model size display with human-readable formatting
  - Recommended model indicators
- **Installation Guidance**
  - Warning box when Ollama not running
  - Direct link to Ollama download page
  - Command instructions for model installation
  - Recommended models list with pull commands
- **Smart Model Selection**
  - Auto-populate dropdown from installed models
  - Fallback to text input for manual entry
  - Show model recommendations when no models installed

#### Changed
- Settings Panel now loads Ollama status on mount
- Model input adapts based on installed models availability
- Enhanced LLM backend section with real-time status

#### Technical Details
- TypeScript interfaces for OllamaStatus and OllamaModelInfo
- API calls to Ollama's /api/tags endpoint
- 2-second timeout for status checks to avoid UI blocking
- Graceful degradation when Ollama not available

#### Phase 3 Status
- ✅ Ollama backend (UnifiedLLMClient already implemented)
- ✅ Ollama status commands
- ✅ Ollama model management commands
- ✅ Ollama UI in Settings Panel
- ⏳ First-run wizard Ollama setup step
- ⏳ End-to-end testing (audio → faster-whisper → Ollama → injection)
- ⏳ Performance benchmarking vs cloud APIs

**Complete Local Pipeline**: Users can now run 100% offline with faster-whisper (Docker) + Ollama (local LLM) at zero cost!

---

### Phase 2.5 (Consumer-Ready UX) - Added 2025-11-11

#### Added - Onboarding
- **First-Run Wizard** - Multi-step guided setup
  - Welcome screen with feature highlights
  - Setup choice: Quick (Cloud) vs Local (Docker)
  - Cloud setup: API key entry with validation
  - Local setup: Docker installation guidance
  - Optional test recording step
  - Success confirmation
- **Progressive Disclosure** - Simple by default, advanced in settings
  - Quick setup: 5 minutes (Cloud mode)
  - Advanced setup: 15 minutes (Local mode)
  - No technical jargon in main flow
  - Clear cost information ($0.10/hour for cloud)
  - Visual feedback at every step

#### Added - Automatic Container Management
- **Background Container Prestart**
  - Automatically starts Docker container on app launch
  - Non-blocking background task
  - Instant first transcription (no 10-20s wait)
  - Graceful degradation if Docker unavailable
- **Transparent Docker Operations**
  - Users never see "Start Container" in normal flow
  - Container lifecycle fully automated
  - Manual controls only in Advanced settings

#### UX Improvements
- Animated progress indicators
- Beautiful gradient design
- Step-by-step guidance
- Error recovery with helpful messages
- Persistent setup state (localStorage)
- Responsive design for all screen sizes

#### Consumer-Ready Features
- ✅ Zero-friction cloud setup (5 min)
- ✅ Guided local setup (15 min)
- ✅ Automatic container management
- ✅ No manual Docker operations needed
- ✅ Clear next steps always shown
- ✅ Professional visual design

**User Experience Flow:**
1. First launch → Beautiful wizard appears
2. Choose Quick (API key) or Local (Docker) setup
3. Follow step-by-step guidance
4. Optional test recording
5. Success! App is ready to use

**Result**: App is now consumer-friendly while remaining powerful for technical users.

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
