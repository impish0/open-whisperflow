# CLAUDE.md - AI Assistant Development Guide

> **For AI Assistants**: This file contains critical context about Open WhisperFlow's architecture, development patterns, and documentation standards. Read this FIRST before making any changes.

---

## 🎯 Project Vision & Goals

### What We're Building
**Open WhisperFlow** - An open-source, privacy-first voice-to-text application with AI-powered text refinement. A competitor to WisprFlow that prioritizes:
- **100% Privacy**: Everything can run locally
- **Zero Cost**: No subscriptions, ever
- **Full Control**: Users own their data and configuration
- **Cross-Platform**: Windows, Linux, macOS (future)
- **Professional Quality**: Production-ready code

### Final Goal
A desktop application where users can:
1. Press a hotkey (or click button)
2. Speak naturally into their microphone
3. Receive cleaned, formatted text inserted at cursor position
4. Choose between local (private) or cloud (fast) processing
5. Customize everything via prompt templates

---

## 📁 Project Structure

```
open-whisperflow/
├── .github/
│   ├── workflows/          # CI/CD pipelines
│   └── ISSUE_TEMPLATE/     # Bug/feature templates
├── docs/                   # Additional documentation
│   ├── api/               # API documentation
│   ├── guides/            # User guides
│   └── development/       # Developer guides
├── src/                   # React frontend (TypeScript)
│   ├── components/        # UI components
│   ├── hooks/            # React hooks
│   ├── services/         # API clients
│   ├── types/            # TypeScript definitions
│   └── test/             # Frontend tests
├── src-tauri/            # Rust backend
│   ├── src/
│   │   ├── audio/        # Audio recording (cpal)
│   │   ├── transcription/# Whisper backends
│   │   ├── llm/          # LLM integration
│   │   ├── injection/    # Text insertion
│   │   ├── config/       # Configuration
│   │   ├── hotkeys/      # Global shortcuts
│   │   ├── utils/        # Utilities
│   │   ├── commands.rs   # Tauri commands (API)
│   │   ├── state.rs      # App state
│   │   ├── error.rs      # Error types
│   │   └── main.rs       # Entry point
│   └── Cargo.toml        # Rust dependencies
├── ARCHITECTURE.md       # Technical architecture
├── CHANGELOG.md          # Version history
├── CLAUDE.md            # This file (AI assistant guide)
├── CONTRIBUTING.md       # Contribution guidelines
├── README.md            # User-facing docs
├── TODO.md              # Task tracking
└── package.json         # Node dependencies
```

---

## 🏗️ Architecture Overview

### Tech Stack
- **Backend**: Rust (Tauri v2, tokio, cpal, reqwest, enigo)
- **Frontend**: React 18 + TypeScript 5
- **Build**: Vite 5
- **APIs**: OpenAI Whisper/GPT, Ollama (local)

### Key Design Patterns

#### 1. **Pluggable Backend Architecture**
```rust
// All backends implement trait
#[async_trait]
pub trait TranscriptionBackend: Send + Sync {
    async fn transcribe(&self, audio_path: &Path) -> Result<String>;
    async fn is_available(&self) -> bool;
    fn name(&self) -> &str;
}
```
- Easy to add new providers (faster-whisper, Azure, etc.)
- Testable in isolation
- Runtime backend selection

#### 2. **Unified LLM Client**
```rust
// Works with both OpenAI and Ollama
pub struct UnifiedLLMClient {
    base_url: String,  // "https://api.openai.com/v1" or "http://localhost:11434/v1"
    // ...
}
```
- Single client for all OpenAI-compatible APIs
- Seamless switching between cloud and local

#### 3. **Type-Safe Configuration**
- Rust: `serde` with strong typing
- TypeScript: Strict mode enabled
- Config changes saved to platform-specific locations
- Schema validation on both ends

#### 4. **Async Throughout**
- All I/O operations are async (tokio)
- Non-blocking UI
- Concurrent API calls where possible

---

## 🔨 Build Instructions

### Prerequisites
```bash
# Required
- Node.js 22+
- Rust 1.91+
- Git

# Linux System Dependencies
sudo apt-get update
sudo apt-get install -y \
  libwebkit2gtk-4.1-dev \
  libappindicator3-dev \
  librsvg2-dev \
  patchelf \
  libasound2-dev

# Windows
# Install Visual Studio Build Tools
# https://visualstudio.microsoft.com/downloads/

# macOS
# Xcode Command Line Tools
xcode-select --install
```

### Development
```bash
# Install dependencies
npm install

# Run in dev mode (hot reload)
npm run tauri:dev

# Run linters
npm run lint
npm run type-check
cd src-tauri && cargo clippy

# Format code
npm run format
cd src-tauri && cargo fmt

# Run tests
npm test
```

### Production Build
```bash
# Build for current platform
npm run tauri:build

# Output locations:
# Linux: src-tauri/target/release/bundle/appimage/
# Windows: src-tauri/target/release/bundle/nsis/
# macOS: src-tauri/target/release/bundle/dmg/
```

---

## 📝 Documentation Standards

### When to Document

**ALWAYS document when you:**
1. Add a new feature
2. Fix a bug
3. Change architecture
4. Add/remove dependencies
5. Update APIs
6. Change configuration format

### Where to Document

#### 1. **CHANGELOG.md** (Required)
```markdown
## [0.2.0] - 2025-11-12

### Added
- faster-whisper Docker integration
- Model download UI with progress bars
- GPU detection and automatic backend selection

### Fixed
- Audio buffer overflow on long recordings
- Race condition in state management

### Changed
- Updated default model from base to small
- Improved error messages for network failures
```

#### 2. **TODO.md** (Required)
```markdown
## 🐛 Known Bugs
- [ ] #42 - Recording fails on some USB microphones
- [ ] #38 - Text injection doesn't work in VS Code on Wayland

## 🚀 In Progress
- [ ] Phase 2: faster-whisper integration (@username, ETA: Week 7)

## 📋 Planned Features
- [ ] Global hotkeys (Phase 4)
- [ ] Voice shortcuts (Phase 5)
```

#### 3. **Code Comments** (Required for complex logic)
```rust
// GOOD: Explains WHY
// We use a hybrid injection method because some apps block clipboard paste
// but typing simulation works everywhere (just slower)
pub async fn inject_text(&mut self, text: &str) -> Result<()> {
    match self.config.method {
        InjectionMethod::Hybrid => {
            // Try clipboard first for speed, fallback to typing
            self.inject_via_clipboard(text).await
                .or_else(|_| self.inject_via_typing(text).await)
        }
    }
}

// BAD: Explains WHAT (obvious from code)
// This function injects text
pub async fn inject_text(&mut self, text: &str) -> Result<()> { ... }
```

#### 4. **Git Commit Messages** (Required)
```bash
# Format: <type>(<scope>): <short summary>
#
# <detailed description>
#
# - Bullet points for multiple changes
# - Reference issues: Fixes #42

# Examples:
git commit -m "feat(audio): add VAD-based auto-stop detection

Implements voice activity detection using Silero VAD to automatically
stop recording when user finishes speaking.

- Add silero-vad dependency
- Implement VadProcessor struct
- Add configurable silence threshold
- Update UI to show VAD status

Fixes #15"

git commit -m "fix(injection): handle special characters in clipboard mode

Some apps were rejecting pasted text with newlines. Now we escape
special characters before clipboard injection.

Fixes #42"
```

**Commit Types:**
- `feat`: New feature
- `fix`: Bug fix
- `refactor`: Code restructuring (no behavior change)
- `perf`: Performance improvement
- `docs`: Documentation only
- `test`: Adding/updating tests
- `chore`: Maintenance (deps, build config)

#### 5. **API Documentation** (Required for public functions)
```rust
/// Transcribe audio file to text using configured backend.
///
/// # Arguments
/// * `audio_path` - Path to WAV file (16kHz, 16-bit, mono)
///
/// # Returns
/// * `Ok(String)` - Transcribed text
/// * `Err(AppError)` - If transcription fails or backend unavailable
///
/// # Example
/// ```rust
/// let service = TranscriptionService::new(&config)?;
/// let text = service.transcribe(Path::new("audio.wav")).await?;
/// ```
pub async fn transcribe(&self, audio_path: &Path) -> Result<String> {
    // ...
}
```

---

## 🎨 Code Style Guidelines

### Rust
- **Follow Rust conventions**: Use `cargo fmt` and `cargo clippy`
- **Error handling**: Use `Result<T>` everywhere, never `.unwrap()` in production
- **Async**: Mark all I/O functions as `async`
- **Documentation**: All public items need doc comments (`///`)

```rust
// GOOD
pub async fn process_audio(path: &Path) -> Result<String> {
    let data = tokio::fs::read(path).await?;
    // ...
}

// BAD
pub fn process_audio(path: &Path) -> String {
    let data = std::fs::read(path).unwrap();  // Can panic!
    // ...
}
```

### TypeScript
- **Strict mode**: Enabled (no `any` without good reason)
- **Functional components**: Use hooks, not classes
- **Props typing**: Always type component props

```typescript
// GOOD
interface RecordingButtonProps {
  isRecording: boolean;
  onStart: () => void;
  onStop: () => void;
}

export default function RecordingButton({
  isRecording,
  onStart,
  onStop
}: RecordingButtonProps) {
  // ...
}

// BAD
export default function RecordingButton(props: any) {  // No any!
  // ...
}
```

### File Organization
- **One component per file**: `RecordingButton.tsx` + `RecordingButton.css`
- **Module structure**: Use `mod.rs` for Rust modules
- **Barrel exports**: Use index files for clean imports

---

## 🧪 Testing Strategy

### What to Test

**Required:**
- All Tauri commands (unit tests)
- Audio recording edge cases
- Configuration loading/saving
- Error handling paths

**Nice to Have:**
- React component tests
- Integration tests
- E2E tests (future)

### Example Test
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_transcription_with_valid_audio() {
        let config = TranscriptionConfig::default();
        let service = TranscriptionService::new(&config).unwrap();

        let result = service.transcribe(Path::new("test_audio.wav")).await;
        assert!(result.is_ok());
        assert!(!result.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_transcription_with_missing_file() {
        let config = TranscriptionConfig::default();
        let service = TranscriptionService::new(&config).unwrap();

        let result = service.transcribe(Path::new("nonexistent.wav")).await;
        assert!(result.is_err());
    }
}
```

---

## 🐛 Bug Reporting & Tracking

### When You Find a Bug
1. **Check TODO.md** - Is it already known?
2. **Add to TODO.md** under "Known Bugs" section
3. **Create GitHub issue** (if user-facing)
4. **Document workaround** if available

### Bug Report Template
```markdown
## Bug: [Brief description]
**Severity**: Critical / High / Medium / Low
**Platform**: Windows / Linux / macOS
**Version**: 0.1.0

### Steps to Reproduce
1. Open app
2. Click record
3. Speak for 30+ seconds
4. App crashes

### Expected Behavior
Recording should work for any duration up to configured limit

### Actual Behavior
App crashes after ~30 seconds of recording

### Error Messages
```
thread 'main' panicked at 'buffer overflow'
```

### Workaround
None currently

### Proposed Fix
Increase buffer size in audio/mod.rs line 42
```

---

## 🚀 Feature Development Process

### Before Starting
1. **Read TODO.md** - Check planned features
2. **Update TODO.md** - Add your feature to "In Progress"
3. **Branch naming**: `feature/short-description` or `fix/bug-description`

### While Developing
1. **Small commits** - One logical change per commit
2. **Test as you go** - Don't wait until the end
3. **Update docs** - Write docs alongside code

### Before Committing
```bash
# Checklist
npm run lint          # Pass
npm run type-check    # Pass
npm run format        # Applied
cd src-tauri && cargo clippy  # Pass
cd src-tauri && cargo fmt     # Applied
npm test              # Pass (if tests exist)

# Update docs
vim CHANGELOG.md      # Add entry
vim TODO.md           # Update progress
```

### Commit Message
Follow the format above. Good commit message = good project history.

---

## 📋 Current Status & Priorities

### ✅ Completed (Phase 1)
- Basic audio recording
- OpenAI Whisper API transcription
- OpenAI GPT rewriting
- Ollama framework (ready for Phase 3)
- Text injection (3 methods)
- Settings UI
- Configuration system

### 🚧 In Progress
- None currently (Phase 1 complete)

### 🎯 Next Up (Phase 2)
1. faster-whisper Docker integration
2. Model download UI
3. GPU detection
4. Offline mode

### ⚠️ Known Issues
- No global hotkeys yet (Phase 4)
- No VAD auto-stop (Phase 4)
- faster-whisper not implemented (Phase 2)
- Text injection may fail on Wayland (investigating)

---

## 💡 Best Practices

### Adding a New Backend

```rust
// 1. Implement the trait
pub struct NewBackend {
    client: SomeClient,
}

#[async_trait]
impl TranscriptionBackend for NewBackend {
    async fn transcribe(&self, audio_path: &Path) -> Result<String> {
        // Implementation
    }

    async fn is_available(&self) -> bool {
        // Check if backend is configured
    }

    fn name(&self) -> &str {
        "New Backend"
    }
}

// 2. Add to config enum
pub enum TranscriptionBackend {
    OpenAI,
    FasterWhisper,
    NewBackend,  // Add here
}

// 3. Update TranscriptionService::new()
match config.backend {
    TranscriptionBackend::NewBackend => {
        Box::new(NewBackend::new(/* ... */))
    }
    // ...
}

// 4. Update UI (SettingsPanel.tsx)
<option value="NewBackend">New Backend</option>

// 5. Document in CHANGELOG.md
```

### Adding a New Tauri Command

```rust
// 1. Define in commands.rs
#[tauri::command]
pub async fn new_command(
    state: State<'_, AppState>,
    param: String,
) -> Result<ReturnType, String> {
    // Implementation
    Ok(result)
}

// 2. Register in main.rs
.invoke_handler(tauri::generate_handler![
    // ...existing commands...
    commands::new_command,  // Add here
])

// 3. Call from frontend
import { invoke } from "@tauri-apps/api/core";

const result = await invoke<ReturnType>("new_command", {
    param: "value"
});

// 4. Add TypeScript types (types/index.ts)
export interface ReturnType {
    // ...
}
```

---

## 🔍 Debugging Tips

### Frontend
```typescript
// Use React DevTools
// Enable verbose logging
console.log("[RecordingButton] Starting recording...");

// Check Tauri IPC
import { invoke } from "@tauri-apps/api/core";
console.log("Calling backend...");
const result = await invoke("command_name");
console.log("Result:", result);
```

### Backend
```rust
// Enable logging
RUST_LOG=debug cargo run

// Add debug prints
log::debug!("Processing audio: {:?}", audio_path);
log::info!("Transcription complete: {} chars", text.len());
log::error!("Failed to connect: {}", err);
```

### Common Issues
1. **"Command not found"** - Did you register in main.rs?
2. **Type mismatch** - Check Rust types match TypeScript
3. **Audio recording fails** - Check microphone permissions
4. **Build fails** - Run `cargo clean` and rebuild

---

## 🎓 Learning Resources

- **Tauri**: https://tauri.app/v2/guides/
- **Rust Async**: https://tokio.rs/tokio/tutorial
- **React + TypeScript**: https://react-typescript-cheatsheet.netlify.app/
- **Whisper**: https://github.com/openai/whisper
- **Ollama**: https://github.com/ollama/ollama

---

## 🤝 Getting Help

1. Check **ARCHITECTURE.md** for technical details
2. Check **TODO.md** for known issues
3. Search GitHub issues
4. Ask in discussions (when set up)

---

## ✨ Remember

- **Document as you code** - Future you will thank present you
- **Test your changes** - Bugs caught early are bugs fixed easily
- **Small PRs** - Easier to review, faster to merge
- **Ask questions** - Better to ask than assume
- **Have fun!** - We're building something incredible! 🚀

---

**Last Updated**: 2025-11-11
**Current Version**: 0.1.0 (Phase 1 MVP)
**Next Milestone**: Phase 2 - Local Whisper (faster-whisper)
