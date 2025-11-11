import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import RecordingButton from "./components/RecordingButton";
import StatusIndicator from "./components/StatusIndicator";
import SettingsPanel from "./components/SettingsPanel";
import { RecordingState, AppConfig } from "./types";
import "./App.css";

function App() {
  const [recordingState, setRecordingState] = useState<RecordingState>({ type: "Idle" });
  const [config, setConfig] = useState<AppConfig | null>(null);
  const [showSettings, setShowSettings] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [lastResult, setLastResult] = useState<{
    transcription: string;
    cleaned_text: string;
  } | null>(null);

  // Load config on mount
  useEffect(() => {
    loadConfig();
  }, []);

  const loadConfig = async () => {
    try {
      const cfg = await invoke<AppConfig>("get_config");
      setConfig(cfg);
    } catch (err) {
      console.error("Failed to load config:", err);
      setError("Failed to load configuration");
    }
  };

  const handleStartRecording = async () => {
    try {
      setError(null);
      await invoke("start_recording");
      setRecordingState({ type: "Recording", data: { started_at: Date.now() } });
    } catch (err) {
      console.error("Failed to start recording:", err);
      setError(err as string);
    }
  };

  const handleStopRecording = async () => {
    try {
      setError(null);
      setRecordingState({ type: "Processing", data: { stage: "Transcribing" } });

      const result = await invoke<{ transcription: string; cleaned_text: string }>(
        "stop_recording"
      );
      setLastResult(result);
      setRecordingState({ type: "Idle" });

      // Show success notification
      console.log("Recording processed successfully:", result);
    } catch (err) {
      console.error("Failed to stop recording:", err);
      setError(err as string);
      setRecordingState({ type: "Idle" });
    }
  };

  const handleCancelRecording = async () => {
    try {
      await invoke("cancel_recording");
      setRecordingState({ type: "Idle" });
      setError(null);
    } catch (err) {
      console.error("Failed to cancel recording:", err);
    }
  };

  const handleUpdateConfig = async (newConfig: AppConfig) => {
    try {
      await invoke("update_config", { config: newConfig });
      setConfig(newConfig);
      setError(null);
    } catch (err) {
      console.error("Failed to update config:", err);
      setError(err as string);
    }
  };

  const isRecording = recordingState.type === "Recording";
  const isProcessing = recordingState.type === "Processing";

  return (
    <div className="app">
      <header className="app-header">
        <h1>🎤 Open WhisperFlow</h1>
        <button className="settings-btn" onClick={() => setShowSettings(!showSettings)}>
          ⚙️
        </button>
      </header>

      <main className="app-main">
        {!showSettings ? (
          <div className="recording-panel">
            <StatusIndicator state={recordingState} />

            <RecordingButton
              isRecording={isRecording}
              isProcessing={isProcessing}
              onStart={handleStartRecording}
              onStop={handleStopRecording}
            />

            {isRecording && (
              <button className="cancel-btn" onClick={handleCancelRecording}>
                Cancel
              </button>
            )}

            {error && (
              <div className="error-message">
                <strong>Error:</strong> {error}
              </div>
            )}

            {lastResult && (
              <div className="result-display">
                <h3>Last Result:</h3>
                <div className="result-section">
                  <h4>Transcription:</h4>
                  <p>{lastResult.transcription}</p>
                </div>
                <div className="result-section">
                  <h4>Cleaned Text:</h4>
                  <p>{lastResult.cleaned_text}</p>
                </div>
              </div>
            )}

            <div className="instructions">
              <h3>How to use:</h3>
              <ol>
                <li>Click the microphone button to start recording</li>
                <li>Speak clearly into your microphone</li>
                <li>Click again to stop and process</li>
                <li>Text will be inserted at your cursor position</li>
              </ol>
              <p className="tip">
                <strong>Tip:</strong> Configure your API keys in settings before using.
              </p>
            </div>
          </div>
        ) : (
          config && (
            <SettingsPanel
              config={config}
              onUpdate={handleUpdateConfig}
              onClose={() => setShowSettings(false)}
            />
          )
        )}
      </main>

      <footer className="app-footer">
        <p>Open WhisperFlow v0.1.0 • Privacy-first voice-to-text</p>
      </footer>
    </div>
  );
}

export default App;
