# Technical Architecture - Open WhisperFlow

## System Architecture Overview

This document provides detailed technical architecture for Open WhisperFlow, covering all major components and their interactions.

## Technology Stack Summary

| Layer | Technology | Purpose |
|-------|-----------|---------|
| Desktop Framework | Tauri v2 | Cross-platform app framework |
| Backend Language | Rust | Performance, safety, system integration |
| Frontend | React + TypeScript | UI development |
| Audio Recording | cpal | Cross-platform audio I/O |
| Voice Activity Detection | Silero VAD | Speech detection |
| Transcription | faster-whisper / OpenAI | Speech-to-text |
| LLM Processing | Ollama / OpenAI | Text refinement |
| Text Injection | enigo | Keyboard simulation |
| Configuration | confy + serde | Settings management |
| Async Runtime | Tokio | Asynchronous operations |

## Component Architecture

### 1. Frontend (React + TypeScript)

**Location**: `src/`

**Key Components**:

```typescript
// Main App Structure
src/
├── App.tsx                 // Main app component
├── components/
│   ├── RecordingButton.tsx // Push-to-talk button
│   ├── StatusIndicator.tsx // Recording/processing status
│   ├── SettingsPanel.tsx   // Configuration UI
│   ├── ModelSelector.tsx   // Whisper/LLM model picker
│   ├── PromptEditor.tsx    // Prompt template editor
│   ├── HistoryViewer.tsx   // Transcription history (future)
│   └── OnboardingWizard.tsx // First-run setup
├── hooks/
│   ├── useAudioRecording.ts // Audio state management
│   ├── useSettings.ts      // Settings hook
│   └── useHotkey.ts        // Hotkey registration
├── services/
│   ├── tauri.ts            // Tauri IPC wrapper
│   └── api.ts              // Type-safe API client
└── types/
    └── index.ts            // TypeScript types
```

**State Management**:
- React hooks + Context API for local state
- Tauri IPC for backend communication
- No heavy state management (Redux/Zustand) needed for MVP

**Key Interactions**:
```typescript
// Example: Trigger recording from UI
import { invoke } from '@tauri-apps/api/tauri';

async function startRecording() {
  await invoke('start_recording');
}

async function stopRecording() {
  const result = await invoke('stop_recording');
  return result; // { transcription, cleaned_text }
}
```

---

### 2. Backend Core (Rust)

**Location**: `src-tauri/src/`

#### 2.1 Main Entry Point

**File**: `main.rs`

```rust
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // Initialize app state
            let app_state = AppState::new();
            app.manage(app_state);
            
            // Setup system tray
            setup_system_tray(app)?;
            
            // Register global hotkeys
            register_hotkeys(app)?;
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            start_recording,
            stop_recording,
            get_config,
            update_config,
            list_models,
            transcribe_audio,
            rewrite_text,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

#### 2.2 Audio Module

**File**: `src-tauri/src/audio/recorder.rs`

**Purpose**: Handle audio recording with VAD

```rust
pub struct AudioRecorder {
    stream: Option<cpal::Stream>,
    buffer: Arc<Mutex<Vec<f32>>>,
    vad: SileroVAD,
    config: AudioConfig,
}

impl AudioRecorder {
    pub fn new() -> Result<Self> { /* ... */ }
    
    pub async fn start_recording(&mut self) -> Result<()> {
        // Initialize cpal stream
        // Start buffering audio
        // Apply VAD filtering
    }
    
    pub async fn stop_recording(&mut self) -> Result<PathBuf> {
        // Stop stream
        // Process buffer
        // Save to WAV file
        // Return file path
    }
    
    fn apply_vad(&self, samples: &[f32]) -> Vec<f32> {
        // Filter out silence using Silero VAD
    }
}
```

**Key Features**:
- Configurable sample rate (default: 16kHz)
- Real-time VAD filtering
- Automatic gain control (optional)
- Noise reduction (optional)

**Dependencies**:
```toml
[dependencies]
cpal = "0.15"
hound = "3.5"  # WAV file I/O
```

---

#### 2.3 Transcription Module

**File**: `src-tauri/src/transcription/mod.rs`

**Architecture**: Plugin-based with multiple backends

```rust
#[async_trait]
pub trait TranscriptionBackend: Send + Sync {
    async fn transcribe(&self, audio_path: &Path) -> Result<String>;
    async fn is_available(&self) -> bool;
    fn name(&self) -> &str;
    fn estimated_speed(&self) -> f32; // Words per minute
}

pub struct TranscriptionService {
    backends: Vec<Box<dyn TranscriptionBackend>>,
    active_backend: String,
}

impl TranscriptionService {
    pub async fn transcribe(&self, audio_path: &Path) -> Result<String> {
        let backend = self.get_active_backend()?;
        backend.transcribe(audio_path).await
    }
    
    pub async fn auto_select_backend(&mut self) -> Result<()> {
        // Check GPU availability
        // Check Docker
        // Check Python environment
        // Select best available
    }
}
```

**Backends**:

1. **FasterWhisperBackend** (Local - Docker)
```rust
pub struct FasterWhisperBackend {
    docker_client: DockerClient,
    container_id: Option<String>,
    model: WhisperModel,
}

impl TranscriptionBackend for FasterWhisperBackend {
    async fn transcribe(&self, audio_path: &Path) -> Result<String> {
        // Mount audio file into container
        // Run faster-whisper command
        // Parse JSON output
        // Return transcription
    }
}
```

2. **OpenAIWhisperBackend** (Cloud)
```rust
pub struct OpenAIWhisperBackend {
    client: reqwest::Client,
    api_key: String,
}

impl TranscriptionBackend for OpenAIWhisperBackend {
    async fn transcribe(&self, audio_path: &Path) -> Result<String> {
        let form = multipart::Form::new()
            .file("file", audio_path)?
            .text("model", "whisper-1");
            
        let resp = self.client
            .post("https://api.openai.com/v1/audio/transcriptions")
            .bearer_auth(&self.api_key)
            .multipart(form)
            .send()
            .await?;
            
        let result: TranscriptionResponse = resp.json().await?;
        Ok(result.text)
    }
}
```

**Dependencies**:
```toml
[dependencies]
async-trait = "0.1"
reqwest = { version = "0.11", features = ["multipart"] }
bollard = "0.15"  # Docker API
```

---

#### 2.4 LLM Module

**File**: `src-tauri/src/llm/mod.rs`

**Purpose**: Text rewriting with LLMs

```rust
#[async_trait]
pub trait LLMBackend: Send + Sync {
    async fn rewrite(&self, text: &str, prompt: &str) -> Result<String>;
    async fn is_available(&self) -> bool;
    fn name(&self) -> &str;
}

pub struct LLMService {
    backends: HashMap<String, Box<dyn LLMBackend>>,
    prompt_templates: HashMap<String, String>,
}

impl LLMService {
    pub async fn rewrite_text(
        &self,
        text: &str,
        template_name: &str,
        context: Option<&str>,
    ) -> Result<String> {
        let prompt = self.build_prompt(text, template_name, context)?;
        let backend = self.get_active_backend()?;
        backend.rewrite(text, &prompt).await
    }
    
    fn build_prompt(
        &self,
        text: &str,
        template_name: &str,
        context: Option<&str>,
    ) -> Result<String> {
        let template = self.prompt_templates.get(template_name)
            .ok_or(Error::TemplateNotFound)?;
            
        // Replace variables: {text}, {context}, etc.
        let prompt = template
            .replace("{text}", text)
            .replace("{context}", context.unwrap_or("general"));
            
        Ok(prompt)
    }
}
```

**Unified Client** (OpenAI-compatible):
```rust
pub struct UnifiedLLMClient {
    base_url: String,
    api_key: String,
    model: String,
    client: reqwest::Client,
}

impl LLMBackend for UnifiedLLMClient {
    async fn rewrite(&self, text: &str, prompt: &str) -> Result<String> {
        let request = ChatCompletionRequest {
            model: self.model.clone(),
            messages: vec![
                Message { role: "system", content: prompt.to_string() },
                Message { role: "user", content: text.to_string() },
            ],
            temperature: 0.7,
            ..Default::default()
        };
        
        let resp = self.client
            .post(format!("{}/chat/completions", self.base_url))
            .bearer_auth(&self.api_key)
            .json(&request)
            .send()
            .await?;
            
        let result: ChatCompletionResponse = resp.json().await?;
        Ok(result.choices[0].message.content.clone())
    }
}
```

**Usage**:
```rust
// Ollama (local)
let ollama_client = UnifiedLLMClient::new(
    "http://localhost:11434/v1",
    "ollama",  // API key (unused but required)
    "llama3.2:3b",
);

// OpenAI (cloud)
let openai_client = UnifiedLLMClient::new(
    "https://api.openai.com/v1",
    "sk-...",  // Real API key
    "gpt-4o-mini",
);
```

---

#### 2.5 Text Injection Module

**File**: `src-tauri/src/injection/mod.rs`

**Purpose**: Insert text into active application

```rust
pub enum InjectionMethod {
    Clipboard,  // Fast but replaces clipboard
    Typing,     // Slower but universal
    Hybrid,     // Clipboard + simulate paste
}

pub struct TextInjector {
    enigo: Enigo,
    clipboard: Clipboard,
    method: InjectionMethod,
}

impl TextInjector {
    pub async fn inject_text(&mut self, text: &str) -> Result<()> {
        match self.method {
            InjectionMethod::Clipboard => self.inject_via_clipboard(text).await,
            InjectionMethod::Typing => self.inject_via_typing(text).await,
            InjectionMethod::Hybrid => self.inject_hybrid(text).await,
        }
    }
    
    async fn inject_via_clipboard(&mut self, text: &str) -> Result<()> {
        // Save current clipboard
        let original = self.clipboard.get_text().ok();
        
        // Set our text
        self.clipboard.set_text(text)?;
        
        // Simulate Ctrl+V
        self.enigo.key(Key::Control, Click)?;
        self.enigo.key(Key::V, Click)?;
        self.enigo.key(Key::Control, Release)?;
        
        // Wait for paste to complete
        tokio::time::sleep(Duration::from_millis(100)).await;
        
        // Restore original clipboard
        if let Some(orig) = original {
            self.clipboard.set_text(&orig)?;
        }
        
        Ok(())
    }
    
    async fn inject_via_typing(&mut self, text: &str) -> Result<()> {
        // Type character by character
        for c in text.chars() {
            self.enigo.text(&c.to_string())?;
            tokio::time::sleep(Duration::from_millis(1)).await;
        }
        Ok(())
    }
    
    async fn inject_hybrid(&mut self, text: &str) -> Result<()> {
        // Try clipboard first, fallback to typing on error
        match self.inject_via_clipboard(text).await {
            Ok(_) => Ok(()),
            Err(_) => self.inject_via_typing(text).await,
        }
    }
}
```

**Dependencies**:
```toml
[dependencies]
enigo = "0.2"
arboard = "3.3"  # Clipboard
```

---

#### 2.6 Hotkey Module

**File**: `src-tauri/src/hotkeys/mod.rs`

**Purpose**: Global hotkey registration

```rust
use tauri_plugin_global_shortcut::{Code, Modifiers, Shortcut, ShortcutState};

pub struct HotkeyManager {
    shortcuts: HashMap<String, Shortcut>,
    handlers: HashMap<String, Box<dyn Fn() + Send>>,
}

impl HotkeyManager {
    pub fn register_recording_hotkey<F>(
        &mut self,
        app: &tauri::AppHandle,
        shortcut: &str,
        handler: F,
    ) -> Result<()>
    where
        F: Fn() + Send + 'static,
    {
        let shortcut = shortcut.parse::<Shortcut>()?;
        
        app.global_shortcut().register(shortcut, move |state| {
            if state == ShortcutState::Pressed {
                handler();
            }
        })?;
        
        self.shortcuts.insert("recording".to_string(), shortcut);
        Ok(())
    }
    
    pub fn unregister_all(&self, app: &tauri::AppHandle) -> Result<()> {
        for shortcut in self.shortcuts.values() {
            app.global_shortcut().unregister(*shortcut)?;
        }
        Ok(())
    }
}
```

**Usage**:
```rust
// Register Ctrl+Shift+Space
let mut hotkey_manager = HotkeyManager::new();
hotkey_manager.register_recording_hotkey(
    &app_handle,
    "Ctrl+Shift+Space",
    move || {
        // Toggle recording state
        state.toggle_recording();
    },
)?;
```

---

#### 2.7 Configuration Module

**File**: `src-tauri/src/config/mod.rs`

**Purpose**: Persistent configuration management

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub audio: AudioConfig,
    pub transcription: TranscriptionConfig,
    pub llm: LLMConfig,
    pub injection: InjectionConfig,
    pub hotkeys: HotkeyConfig,
    pub ui: UIConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TranscriptionConfig {
    pub backend: String,  // "faster-whisper", "openai"
    pub model: String,    // "small", "medium", etc.
    pub language: Option<String>,
    pub openai_api_key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LLMConfig {
    pub backend: String,  // "ollama", "openai"
    pub model: String,    // "llama3.2:3b", "gpt-4o-mini"
    pub base_url: String,
    pub api_key: Option<String>,
    pub default_template: String,
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        confy::load("open-whisperflow", None)
            .map_err(|e| Error::ConfigLoad(e.to_string()))
    }
    
    pub fn save(&self) -> Result<()> {
        confy::store("open-whisperflow", None, self)
            .map_err(|e| Error::ConfigSave(e.to_string()))
    }
    
    pub fn default() -> Self {
        Self {
            audio: AudioConfig::default(),
            transcription: TranscriptionConfig {
                backend: "faster-whisper".to_string(),
                model: "small".to_string(),
                language: None,
                openai_api_key: None,
            },
            llm: LLMConfig {
                backend: "ollama".to_string(),
                model: "llama3.2:3b".to_string(),
                base_url: "http://localhost:11434/v1".to_string(),
                api_key: None,
                default_template: "balanced".to_string(),
            },
            injection: InjectionConfig::default(),
            hotkeys: HotkeyConfig::default(),
            ui: UIConfig::default(),
        }
    }
}
```

**Storage Location** (automatically handled by confy):
- Linux: `~/.config/open-whisperflow/open-whisperflow.toml`
- Windows: `C:\Users\<User>\AppData\Roaming\open-whisperflow\open-whisperflow.toml`

---

### 3. State Management

**File**: `src-tauri/src/state.rs`

```rust
use tokio::sync::RwLock;

pub struct AppState {
    pub config: RwLock<AppConfig>,
    pub recording_state: RwLock<RecordingState>,
    pub audio_recorder: RwLock<AudioRecorder>,
    pub transcription_service: RwLock<TranscriptionService>,
    pub llm_service: RwLock<LLMService>,
    pub text_injector: RwLock<TextInjector>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RecordingState {
    Idle,
    Recording { started_at: Instant },
    Processing { stage: ProcessingStage },
    Error { message: String },
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProcessingStage {
    Transcribing,
    Rewriting,
    Injecting,
}

impl AppState {
    pub fn new() -> Self {
        let config = AppConfig::load().unwrap_or_default();
        
        Self {
            config: RwLock::new(config.clone()),
            recording_state: RwLock::new(RecordingState::Idle),
            audio_recorder: RwLock::new(AudioRecorder::new().unwrap()),
            transcription_service: RwLock::new(TranscriptionService::new(&config)),
            llm_service: RwLock::new(LLMService::new(&config)),
            text_injector: RwLock::new(TextInjector::new(&config)),
        }
    }
}
```

---

### 4. Core Workflow Implementation

**File**: `src-tauri/src/commands.rs`

```rust
#[tauri::command]
async fn start_recording(state: State<'_, AppState>) -> Result<(), String> {
    let mut recording_state = state.recording_state.write().await;
    
    if *recording_state != RecordingState::Idle {
        return Err("Already recording or processing".to_string());
    }
    
    *recording_state = RecordingState::Recording {
        started_at: Instant::now(),
    };
    
    let mut recorder = state.audio_recorder.write().await;
    recorder.start_recording().await
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
async fn stop_recording(state: State<'_, AppState>) -> Result<ProcessedResult, String> {
    // Stop recording
    let mut recorder = state.audio_recorder.write().await;
    let audio_path = recorder.stop_recording().await
        .map_err(|e| e.to_string())?;
    
    // Update state
    let mut recording_state = state.recording_state.write().await;
    *recording_state = RecordingState::Processing {
        stage: ProcessingStage::Transcribing,
    };
    drop(recording_state);
    
    // Transcribe
    let transcription_service = state.transcription_service.read().await;
    let transcription = transcription_service.transcribe(&audio_path).await
        .map_err(|e| e.to_string())?;
    drop(transcription_service);
    
    // Update state
    let mut recording_state = state.recording_state.write().await;
    *recording_state = RecordingState::Processing {
        stage: ProcessingStage::Rewriting,
    };
    drop(recording_state);
    
    // Rewrite with LLM
    let config = state.config.read().await;
    let llm_service = state.llm_service.read().await;
    let cleaned_text = llm_service.rewrite_text(
        &transcription,
        &config.llm.default_template,
        None,
    ).await.map_err(|e| e.to_string())?;
    drop(llm_service);
    drop(config);
    
    // Update state
    let mut recording_state = state.recording_state.write().await;
    *recording_state = RecordingState::Processing {
        stage: ProcessingStage::Injecting,
    };
    drop(recording_state);
    
    // Inject text
    let mut text_injector = state.text_injector.write().await;
    text_injector.inject_text(&cleaned_text).await
        .map_err(|e| e.to_string())?;
    drop(text_injector);
    
    // Cleanup
    tokio::fs::remove_file(&audio_path).await.ok();
    
    // Reset state
    let mut recording_state = state.recording_state.write().await;
    *recording_state = RecordingState::Idle;
    
    Ok(ProcessedResult {
        transcription,
        cleaned_text,
    })
}

#[derive(Serialize)]
struct ProcessedResult {
    transcription: String,
    cleaned_text: String,
}
```

---

## Data Flow Diagram

```
┌─────────────┐
│    USER     │
│ Presses Key │
└──────┬──────┘
       │
       ↓
┌─────────────────────┐
│  Hotkey Manager     │
│  Detects Press      │
└──────┬──────────────┘
       │
       ↓
┌─────────────────────┐
│  Recording State    │
│  → Recording        │
└──────┬──────────────┘
       │
       ↓
┌─────────────────────┐
│  Audio Recorder     │
│  • cpal stream      │
│  • VAD filtering    │
│  • Buffer to RAM    │
└──────┬──────────────┘
       │ (continuous)
       ↓
┌─────────────────────┐
│  USER Releases Key  │
└──────┬──────────────┘
       │
       ↓
┌─────────────────────┐
│  Stop Recording     │
│  • Save to WAV      │
│  • Return path      │
└──────┬──────────────┘
       │
       ↓
┌─────────────────────┐
│  Transcription Svc  │
│  • Select backend   │
│  • Call API/Docker  │
│  • Parse result     │
└──────┬──────────────┘
       │
       ↓
┌─────────────────────┐
│  Raw Transcription  │
│  "um so like I      │
│   need to uh send"  │
└──────┬──────────────┘
       │
       ↓
┌─────────────────────┐
│  LLM Service        │
│  • Load prompt      │
│  • Detect context   │
│  • Call LLM         │
└──────┬──────────────┘
       │
       ↓
┌─────────────────────┐
│  Cleaned Text       │
│  "I need to send"   │
└──────┬──────────────┘
       │
       ↓
┌─────────────────────┐
│  Text Injector      │
│  • Get active win   │
│  • Inject text      │
│  • Verify success   │
└──────┬──────────────┘
       │
       ↓
┌─────────────────────┐
│  Text Appears!      │
│  User continues     │
│  working            │
└─────────────────────┘
```

---

## Performance Considerations

### Latency Budget

**Target: <2 seconds end-to-end (balanced mode)**

| Stage | Target | Notes |
|-------|--------|-------|
| Recording | 0s (user controlled) | Push-to-talk |
| Audio Processing | 50ms | VAD filtering, save WAV |
| Transcription | 800ms | faster-whisper small + GPU |
| LLM Rewriting | 800ms | Ollama llama3.2:3b |
| Text Injection | 100ms | Clipboard paste |
| **Total** | **1.75s** | Within budget |

### Optimization Strategies

1. **Parallel Processing**:
   ```rust
   let (transcription, context) = tokio::join!(
       transcribe_audio(&path),
       detect_active_app(),
   );
   ```

2. **Model Caching**:
   - Keep models loaded in memory
   - Use model servers (Ollama keeps models warm)
   - Docker containers stay running

3. **Audio Optimization**:
   - VAD pre-filtering reduces Whisper input
   - 16kHz sampling (Whisper native)
   - Mono audio (half the data)

4. **Streaming** (future):
   - Stream audio to Whisper in chunks
   - Progressive transcription
   - Start LLM rewriting before transcription completes

### Resource Management

**Memory Usage**:
- Idle: 50MB (Tauri + React)
- Recording: 100MB (audio buffer)
- Processing: 500MB+ (model inference)
- With GPU: VRAM depends on model (1-4GB)

**CPU Usage**:
- Idle: <1%
- Recording: 5-10% (audio processing)
- CPU Transcription: 100% (one core)
- GPU Transcription: 10-20%

**Disk Usage**:
- App: 50MB
- Models: 100MB (tiny) to 3GB (large)
- Temp audio: 1-10MB per recording (auto-cleaned)

---

## Security Architecture

### 1. API Key Storage

```rust
use keyring::Entry;

pub struct SecureStorage;

impl SecureStorage {
    pub fn store_api_key(service: &str, key: &str) -> Result<()> {
        let entry = Entry::new("open-whisperflow", service)?;
        entry.set_password(key)?;
        Ok(())
    }
    
    pub fn retrieve_api_key(service: &str) -> Result<String> {
        let entry = Entry::new("open-whisperflow", service)?;
        entry.get_password().map_err(Into::into)
    }
}
```

Uses OS-native secure storage:
- Windows: Credential Manager
- Linux: Secret Service (GNOME Keyring, KWallet)
- macOS: Keychain

### 2. Audio File Cleanup

```rust
pub struct SecureAudioFile {
    path: PathBuf,
}

impl SecureAudioFile {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl Drop for SecureAudioFile {
    fn drop(&mut self) {
        // Overwrite with zeros before deleting (paranoid mode)
        if let Ok(mut file) = OpenOptions::new().write(true).open(&self.path) {
            let size = file.metadata().unwrap().len();
            let zeros = vec![0u8; size as usize];
            file.write_all(&zeros).ok();
        }
        std::fs::remove_file(&self.path).ok();
    }
}
```

### 3. Process Isolation

- Whisper and LLM run in separate processes (Docker/Python)
- No audio data sent to untrusted services without consent
- All network requests over HTTPS/TLS

---

## Error Handling Strategy

### Error Types

```rust
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Audio recording failed: {0}")]
    AudioRecording(String),
    
    #[error("Transcription failed: {0}")]
    Transcription(String),
    
    #[error("LLM rewriting failed: {0}")]
    LLMRewriting(String),
    
    #[error("Text injection failed: {0}")]
    TextInjection(String),
    
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
}
```

### Error Recovery

```rust
impl TranscriptionService {
    pub async fn transcribe_with_fallback(&self, path: &Path) -> Result<String> {
        // Try primary backend
        match self.transcribe(path).await {
            Ok(result) => return Ok(result),
            Err(e) => {
                log::warn!("Primary backend failed: {}, trying fallback", e);
            }
        }
        
        // Try OpenAI API fallback
        if let Some(fallback) = &self.fallback_backend {
            return fallback.transcribe(path).await;
        }
        
        Err(Error::NoAvailableBackend)
    }
}
```

### User-Facing Errors

```rust
pub fn format_user_error(error: &AppError) -> String {
    match error {
        AppError::AudioRecording(_) => {
            "Failed to record audio. Please check your microphone permissions."
        }
        AppError::Transcription(_) => {
            "Transcription failed. Try using cloud transcription in settings."
        }
        AppError::Network(_) => {
            "Network error. Check your internet connection or use local mode."
        }
        _ => "An unexpected error occurred. Please try again.",
    }.to_string()
}
```

---

## Testing Strategy

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_audio_recording() {
        let mut recorder = AudioRecorder::new().unwrap();
        recorder.start_recording().await.unwrap();
        tokio::time::sleep(Duration::from_secs(1)).await;
        let path = recorder.stop_recording().await.unwrap();
        assert!(path.exists());
    }
    
    #[tokio::test]
    async fn test_transcription() {
        let service = TranscriptionService::new(&Config::default());
        let result = service.transcribe(Path::new("test.wav")).await;
        assert!(result.is_ok());
    }
}
```

### Integration Tests

```rust
#[tokio::test]
async fn test_end_to_end_workflow() {
    let state = AppState::new();
    
    // Start recording
    start_recording(state.clone()).await.unwrap();
    tokio::time::sleep(Duration::from_secs(2)).await;
    
    // Stop and process
    let result = stop_recording(state.clone()).await.unwrap();
    
    assert!(!result.transcription.is_empty());
    assert!(!result.cleaned_text.is_empty());
}
```

---

## Deployment Architecture

### Development
```
Developer Machine
├── Rust Backend (src-tauri/)
├── React Frontend (src/)
├── Local Whisper (Docker)
└── Local Ollama
```

### Production (User's Machine)
```
User Machine
├── Tauri App Binary
├── Optional: Docker + faster-whisper
├── Optional: Ollama
└── Config: ~/.config/open-whisperflow/
```

### CI/CD Pipeline
```
GitHub Actions
├── Build (Windows, Linux)
├── Test (Unit + Integration)
├── Sign Binaries
├── Create Installers (NSIS, AppImage)
└── Publish Release
```

---

## Monitoring & Telemetry (Opt-in)

### Metrics Collected (with user consent)
```rust
#[derive(Serialize)]
pub struct UsageMetrics {
    pub recordings_count: u64,
    pub avg_recording_duration: f64,
    pub avg_processing_time: f64,
    pub preferred_backend: String,
    pub errors_count: HashMap<String, u64>,
}
```

### Privacy-Preserving Analytics
- No audio content collected
- No transcription text collected
- Aggregate statistics only
- Anonymous identifiers
- Fully opt-in

---

## Future Architecture Considerations

### Plugin System
```rust
pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn on_transcription(&self, text: &str) -> Result<String>;
    fn on_rewrite(&self, text: &str) -> Result<String>;
}
```

### Streaming Architecture
```
Audio Input
    ↓ (chunks)
VAD Filter
    ↓
Whisper Streaming
    ↓ (partial results)
Progressive LLM
    ↓
Incremental Text Injection
```

### Multi-Device Sync (Future)
```
Desktop App ←→ Cloud Sync Service ←→ Mobile App
    ↓               ↓                    ↓
 Settings      Prompt Templates      History
```

---

This architecture provides a solid foundation for Open WhisperFlow that balances performance, security, maintainability, and user experience. All design decisions prioritize user privacy while maintaining flexibility for future enhancements.
