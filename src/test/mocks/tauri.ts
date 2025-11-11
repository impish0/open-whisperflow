import { vi } from "vitest";

// Mock Tauri invoke function
export const mockInvoke = vi.fn();

// Mock implementations for common commands
export const mockCommands = {
  get_config: () => ({
    audio: {
      sample_rate: 16000,
      channels: 1,
      bit_depth: 16,
      device_id: "default",
      vad_enabled: false,
      max_recording_duration_seconds: 300,
    },
    transcription: {
      backend: "OpenAI",
      model: "whisper-1",
      language: null,
      openai_api_key: null,
    },
    llm: {
      backend: "OpenAI",
      model: "gpt-4o-mini",
      base_url: "https://api.openai.com/v1",
      api_key: null,
      default_template: "balanced",
      temperature: 0.7,
      max_tokens: 500,
    },
    injection: {
      method: "Hybrid",
      typing_speed_ms: 10,
      clipboard_backup: true,
    },
    hotkeys: {
      toggle_recording: "Ctrl+Shift+R",
      cancel_recording: "Escape",
    },
    ui: {
      theme: "System",
      show_notifications: true,
      minimize_to_tray: true,
    },
  }),
  get_recording_state: () => ({ type: "Idle" }),
  check_docker_status: () => ({
    available: true,
    container_running: false,
    has_nvidia_gpu: false,
    message: "Docker is running",
  }),
  check_ollama_status: () => ({
    available: false,
    base_url: "http://localhost:11434",
    message: "Ollama is not running or not installed",
  }),
  get_available_models: () => [
    {
      name: "tiny",
      size: "39 MB",
      description: "Fastest, lowest accuracy",
      recommended: false,
    },
    {
      name: "small",
      size: "244 MB",
      description: "Balanced speed and accuracy",
      recommended: true,
    },
  ],
  get_ollama_models: () => [],
  get_recommended_ollama_models: () => [
    {
      name: "llama3.2:3b",
      size: "2 GB",
      modified_at: "",
      recommended: true,
    },
  ],
};

// Configure mock invoke to return appropriate values
export const setupMockInvoke = () => {
  mockInvoke.mockImplementation((command: string, args?: unknown) => {
    const handler = mockCommands[command as keyof typeof mockCommands];
    if (handler) {
      return Promise.resolve(handler(args));
    }
    return Promise.resolve(null);
  });
};

// Reset all mocks
export const resetMocks = () => {
  mockInvoke.mockReset();
  setupMockInvoke();
};

// Export for use in tests
export default {
  mockInvoke,
  mockCommands,
  setupMockInvoke,
  resetMocks,
};
