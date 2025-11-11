# TODO - Open WhisperFlow

> **Status**: Phase 1 MVP Complete ✅ | Next: Phase 2

---

## 🐛 Known Bugs

### Critical
*None currently*

### High Priority
- [ ] **Text injection fails on Wayland** - Needs investigation, may require libei
  - Workaround: Use X11 session
  - Related: #TBD

### Medium Priority
- [ ] **No error recovery for failed API calls** - Need retry logic with exponential backoff
  - Currently shows error but doesn't retry
  - Should implement 3 retries with 2s, 4s, 8s delays

### Low Priority
- [ ] **Config validation incomplete** - Invalid configs can crash app
  - Need schema validation on load
  - Should show user-friendly error on invalid config

---

## 🚧 In Progress

*Nothing currently in progress*

**Instructions**: When starting work on a feature, move it here and add assignee + ETA

---

## 📋 Planned Features

### Phase 2: Local Whisper (Weeks 5-7) - Next Up!

#### Must Have
- [ ] **faster-whisper Docker integration**
  - Implement FasterWhisperBackend struct
  - Docker container management (start/stop/restart)
  - Volume mounting for audio files
  - Health checks and auto-restart
  - Error handling for Docker not installed

- [ ] **Model download UI**
  - Model selection dropdown (tiny/base/small/medium/large)
  - Download button with progress bar
  - Disk space calculation and warnings
  - Model size indicators
  - Cancel download functionality

- [ ] **GPU detection**
  - Detect NVIDIA GPUs (CUDA)
  - Detect AMD GPUs (ROCm)
  - Detect Intel GPUs (OpenVINO) - future
  - Automatic backend selection based on hardware
  - Fallback to CPU if no GPU

- [ ] **Model management**
  - List downloaded models
  - Delete models UI
  - Model switching without restart
  - Cache management
  - Model performance stats (speed, accuracy)

#### Nice to Have
- [ ] Model quantization support (int8, int4)
- [ ] Batch processing for multiple recordings
- [ ] Model performance benchmarking tool
- [ ] Auto-download on first use

---

### Phase 3: Local LLM & Prompts (Weeks 8-10)

#### Must Have
- [ ] **Full Ollama integration**
  - Ollama installation detection
  - Automatic Ollama startup
  - Model pull UI
  - Health monitoring

- [ ] **Multiple prompt templates**
  - Template management UI (add/edit/delete)
  - Import/export templates (.json)
  - Per-application template selection
  - Template variables support

- [ ] **Context detection**
  - Active window detection
  - Application-specific prompts
  - Per-app settings

#### Nice to Have
- [ ] Template marketplace (community templates)
- [ ] A/B testing for prompts
- [ ] Prompt performance analytics

---

### Phase 4: UX Polish (Weeks 11-13)

#### Must Have
- [ ] **Global hotkeys**
  - Customizable shortcuts
  - Conflict detection
  - Visual feedback on trigger
  - Platform-specific implementations (X11/Wayland/Windows)

- [ ] **VAD auto-stop**
  - Silero VAD integration
  - Configurable silence threshold
  - Visual indicator of voice activity
  - Manual override option

- [ ] **First-run wizard**
  - Welcome screen
  - API key setup
  - Model download
  - Test recording
  - Hotkey configuration

- [ ] **UI improvements**
  - Recording waveform visualization
  - Time limit warnings
  - Keyboard shortcuts throughout app
  - Animations and transitions
  - Toast notifications

#### Nice to Have
- [ ] System tray menu with quick actions
- [ ] Multiple color themes
- [ ] Custom window size/position memory
- [ ] Accessibility improvements (screen reader support)

---

### Phase 5: Advanced Features (Weeks 14-16)

#### Must Have
- [ ] **Voice shortcuts**
  - Trigger phrases ("insert email", "paste signature")
  - Custom text snippets
  - Variable substitution ({name}, {date}, etc.)
  - Management UI

- [ ] **Transcription history**
  - Save last 100 transcriptions
  - Search functionality
  - Re-insert previous transcription
  - Export to file

- [ ] **Multi-language support**
  - Language detection
  - Language-specific prompts
  - UI internationalization (i18n)

#### Nice to Have
- [ ] Usage statistics and analytics
- [ ] Advanced rewriting (multiple passes)
- [ ] Style presets (formal/casual/technical)
- [ ] CLI interface for automation
- [ ] HTTP API for integrations

---

### Phase 6: Production (Weeks 17-20)

#### Must Have
- [ ] **Auto-updater**
  - Check for updates on startup
  - Background downloads
  - Install and restart
  - Rollback on failure

- [ ] **Stability improvements**
  - Memory leak fixes
  - Crash recovery
  - Automatic error reporting (opt-in)
  - Comprehensive logging

- [ ] **Documentation**
  - User guide (HTML/PDF)
  - Video tutorials
  - API documentation
  - Developer guide
  - FAQ section

- [ ] **Distribution**
  - Signed installers (Windows, Linux, macOS)
  - Auto-update infrastructure
  - Website with downloads
  - Release notes automation

#### Nice to Have
- [ ] Crash reporter with privacy controls
- [ ] Performance monitoring
- [ ] Beta testing program
- [ ] Community forum

---

## 💡 Feature Ideas (Backlog)

### For Future Consideration
- [ ] Mobile apps (iOS, Android)
- [ ] Browser extension
- [ ] VS Code extension for code dictation
- [ ] Obsidian/Notion integrations
- [ ] Real-time collaborative transcription
- [ ] Speaker diarization (multi-person conversations)
- [ ] Custom Whisper model fine-tuning tools
- [ ] Plugin marketplace
- [ ] Team/enterprise features
- [ ] Cloud sync for settings (optional)
- [ ] Streaming transcription (real-time)
- [ ] Translation mode (dictate in one language, output in another)

---

## 🔧 Technical Debt

### High Priority
- [ ] **faster-whisper backend implementation** (transcription/mod.rs:109)
  - Implement FasterWhisperBackend trait
  - Docker container management
  - See Phase 2 tasks for details

- [ ] Add comprehensive error recovery for all async operations
- [ ] Implement proper logging system (file rotation, levels)
- [ ] Add integration tests for full workflow
- [ ] Security audit of API key storage

### Medium Priority
- [ ] **Global hotkey implementation** (hotkeys/mod.rs)
  - Complete tauri-plugin-global-shortcut integration
  - See Phase 4 for full implementation plan

- [ ] **Secure file deletion** (utils/mod.rs:9)
  - Implement overwrite-before-delete for paranoid mode
  - Optional feature for privacy-conscious users

- [ ] Refactor state management (consider using proper state machine)
- [ ] Add performance monitoring/profiling
- [ ] Improve audio buffer management (prevent overflow)
- [ ] Add unit tests for all modules (currently minimal)

### Low Priority
- [ ] Consider migrating to workspace for better dependency management
- [ ] Evaluate alternative audio library (cpal has some platform issues)
- [ ] Consider using native system notifications instead of custom

---

## 📊 Metrics & Goals

### Phase 1 Metrics (Achieved)
- ✅ 0 linting errors
- ✅ 100% code formatted
- ✅ Full type safety
- ✅ Comprehensive documentation

### Phase 2 Goals
- Add 20+ unit tests
- Add 5+ integration tests
- Maintain 0 lint errors
- < 500ms average transcription overhead
- Support 90%+ GPUs (NVIDIA, AMD)

### Long-term Goals (v1.0)
- < 2s end-to-end latency (balanced mode)
- < 1s latency (quick mode)
- 95%+ transcription accuracy
- 90%+ installation success rate
- 1000+ GitHub stars
- 50+ contributors
- 10k+ users

---

## 🎯 Current Sprint (Example)

*Update this section when starting new work*

**Sprint**: Phase 2 - Week 1
**Duration**: Nov 11 - Nov 18
**Goal**: faster-whisper Docker integration

**Tasks**:
- [ ] Research faster-whisper Docker images
- [ ] Implement Docker client wrapper
- [ ] Add container lifecycle management
- [ ] Test with different Whisper models
- [ ] Update UI for backend selection

**Assignee**: TBD
**Status**: Not started

---

## 📝 Notes

### Maintenance
- Review and update this file weekly
- Archive completed items monthly
- Prioritize bugs over features
- Document all architectural decisions

### Conventions
- Use checkboxes `- [ ]` for all tasks
- Add issue numbers when available: `#42`
- Mark priority: Critical/High/Medium/Low
- Include assignee when work starts
- Add ETA for in-progress items

---

**Last Updated**: 2025-11-11
**Next Review**: 2025-11-18
**Current Phase**: 1 (Complete)
**Next Phase**: 2 (Starting)
