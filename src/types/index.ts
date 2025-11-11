// Core application types

export type RecordingState =
  | { type: "Idle" }
  | { type: "Recording"; data: { started_at: number } }
  | { type: "Processing"; data: { stage: ProcessingStage } }
  | { type: "Error"; data: { message: string } };

export type ProcessingStage = "Transcribing" | "Rewriting" | "Injecting";

export interface AppConfig {
  audio: AudioConfig;
  transcription: TranscriptionConfig;
  llm: LLMConfig;
  injection: InjectionConfig;
  hotkeys: HotkeyConfig;
  ui: UIConfig;
}

export interface AudioConfig {
  sample_rate: number;
  channels: number;
  bit_depth: number;
  device_id: string;
  vad_enabled: boolean;
  max_recording_duration_seconds: number;
}

export interface TranscriptionConfig {
  backend: TranscriptionBackend;
  model: string;
  language: string | null;
  openai_api_key: string | null;
}

export type TranscriptionBackend = "FasterWhisper" | "OpenAI";

export interface LLMConfig {
  backend: LLMBackend;
  model: string;
  base_url: string;
  api_key: string | null;
  default_template: string;
  temperature: number;
  max_tokens: number;
}

export type LLMBackend = "Ollama" | "OpenAI" | "None";

export interface InjectionConfig {
  method: InjectionMethod;
  typing_speed_ms: number;
  clipboard_backup: boolean;
}

export type InjectionMethod = "Clipboard" | "Typing" | "Hybrid";

export interface HotkeyConfig {
  toggle_recording: string;
  cancel_recording: string;
}

export interface UIConfig {
  theme: Theme;
  show_notifications: boolean;
  minimize_to_tray: boolean;
}

export type Theme = "Light" | "Dark" | "System";

export interface BackendStatus {
  name: string;
  available: boolean;
  message: string;
}
