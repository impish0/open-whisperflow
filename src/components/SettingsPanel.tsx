import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { AppConfig, DockerStatus, ModelInfo, OllamaStatus, OllamaModelInfo } from "@/types";
import "./SettingsPanel.css";

interface SettingsPanelProps {
  config: AppConfig;
  onUpdate: (config: AppConfig) => void;
  onClose: () => void;
}

export default function SettingsPanel({ config, onUpdate, onClose }: SettingsPanelProps) {
  const [localConfig, setLocalConfig] = useState<AppConfig>(config);
  const [dockerStatus, setDockerStatus] = useState<DockerStatus | null>(null);
  const [availableModels, setAvailableModels] = useState<ModelInfo[]>([]);
  const [ollamaStatus, setOllamaStatus] = useState<OllamaStatus | null>(null);
  const [ollamaModels, setOllamaModels] = useState<OllamaModelInfo[]>([]);
  const [recommendedOllamaModels, setRecommendedOllamaModels] = useState<OllamaModelInfo[]>([]);
  const [loading, setLoading] = useState(false);

  // Load Docker status and models on mount
  useEffect(() => {
    loadDockerStatus();
    loadAvailableModels();
    loadOllamaStatus();
    loadOllamaModels();
    loadRecommendedOllamaModels();
  }, []);

  const loadDockerStatus = async () => {
    try {
      const status = await invoke<DockerStatus>("check_docker_status");
      setDockerStatus(status);
    } catch (error) {
      console.error("Failed to load Docker status:", error);
    }
  };

  const loadAvailableModels = async () => {
    try {
      const models = await invoke<ModelInfo[]>("get_available_models");
      setAvailableModels(models);
    } catch (error) {
      console.error("Failed to load models:", error);
    }
  };

  const loadOllamaStatus = async () => {
    try {
      const status = await invoke<OllamaStatus>("check_ollama_status");
      setOllamaStatus(status);
    } catch (error) {
      console.error("Failed to load Ollama status:", error);
    }
  };

  const loadOllamaModels = async () => {
    try {
      const models = await invoke<OllamaModelInfo[]>("get_ollama_models");
      setOllamaModels(models);
    } catch (error) {
      console.error("Failed to load Ollama models:", error);
      setOllamaModels([]);
    }
  };

  const loadRecommendedOllamaModels = async () => {
    try {
      const models = await invoke<OllamaModelInfo[]>("get_recommended_ollama_models");
      setRecommendedOllamaModels(models);
    } catch (error) {
      console.error("Failed to load recommended Ollama models:", error);
    }
  };

  const handleStartContainer = async () => {
    setLoading(true);
    try {
      await invoke("start_whisper_container");
      await loadDockerStatus();
      alert("Container started successfully!");
    } catch (error) {
      alert(`Failed to start container: ${error}`);
    } finally {
      setLoading(false);
    }
  };

  const handleStopContainer = async () => {
    setLoading(true);
    try {
      await invoke("stop_whisper_container");
      await loadDockerStatus();
      alert("Container stopped successfully!");
    } catch (error) {
      alert(`Failed to stop container: ${error}`);
    } finally {
      setLoading(false);
    }
  };

  const handleSave = () => {
    onUpdate(localConfig);
    onClose();
  };

  const updateTranscription = (field: string, value: string | null) => {
    setLocalConfig({
      ...localConfig,
      transcription: { ...localConfig.transcription, [field]: value },
    });
  };

  const updateLLM = (field: string, value: string | null) => {
    setLocalConfig({
      ...localConfig,
      llm: { ...localConfig.llm, [field]: value },
    });
  };

  return (
    <div className="settings-panel">
      <h2>Settings</h2>

      <section className="settings-section">
        <h3>Transcription</h3>

        <label>
          Backend:
          <select
            value={localConfig.transcription.backend}
            onChange={(e) => updateTranscription("backend", e.target.value)}
          >
            <option value="OpenAI">OpenAI Whisper API (Cloud)</option>
            <option value="FasterWhisper">faster-whisper (Local - Docker)</option>
          </select>
        </label>

        {localConfig.transcription.backend === "OpenAI" && (
          <>
            <label>
              OpenAI API Key:
              <input
                type="password"
                placeholder="sk-..."
                value={localConfig.transcription.openai_api_key || ""}
                onChange={(e) => updateTranscription("openai_api_key", e.target.value || null)}
              />
            </label>
            <label>
              Model:
              <input
                type="text"
                value={localConfig.transcription.model}
                onChange={(e) => updateTranscription("model", e.target.value)}
                placeholder="whisper-1"
              />
            </label>
          </>
        )}

        {localConfig.transcription.backend === "FasterWhisper" && (
          <>
            {/* Docker Status */}
            <div className="docker-status">
              <h4>Docker Status</h4>
              {dockerStatus ? (
                <>
                  <div
                    className={`status-badge ${dockerStatus.available ? "status-ok" : "status-error"}`}
                  >
                    Docker: {dockerStatus.available ? "✓ Running" : "✗ Not Running"}
                  </div>
                  {dockerStatus.available && (
                    <>
                      <div
                        className={`status-badge ${dockerStatus.container_running ? "status-ok" : "status-warning"}`}
                      >
                        Container: {dockerStatus.container_running ? "✓ Running" : "Stopped"}
                      </div>
                      {dockerStatus.has_nvidia_gpu && (
                        <div className="status-badge status-ok">GPU: ✓ NVIDIA CUDA Available</div>
                      )}
                    </>
                  )}
                  <p className="status-message">{dockerStatus.message}</p>
                </>
              ) : (
                <div>Loading Docker status...</div>
              )}
            </div>

            {/* Container Management */}
            {dockerStatus?.available && (
              <div className="container-actions">
                {!dockerStatus.container_running ? (
                  <button
                    className="btn btn-primary"
                    onClick={handleStartContainer}
                    disabled={loading}
                  >
                    {loading ? "Starting..." : "Start Container"}
                  </button>
                ) : (
                  <button
                    className="btn btn-secondary"
                    onClick={handleStopContainer}
                    disabled={loading}
                  >
                    {loading ? "Stopping..." : "Stop Container"}
                  </button>
                )}
              </div>
            )}

            {/* Model Selection */}
            <label>
              Whisper Model:
              <select
                value={localConfig.transcription.model}
                onChange={(e) => updateTranscription("model", e.target.value)}
              >
                {availableModels.map((model) => (
                  <option key={model.name} value={model.name}>
                    {model.name} - {model.size} {model.recommended ? "(Recommended)" : ""} -{" "}
                    {model.description}
                  </option>
                ))}
              </select>
            </label>

            {!dockerStatus?.available && (
              <div className="info-box warning">
                <strong>Docker Required:</strong> Please install and start Docker Desktop to use
                local transcription.
                <br />
                <a
                  href="https://www.docker.com/products/docker-desktop/"
                  target="_blank"
                  rel="noopener noreferrer"
                >
                  Download Docker Desktop
                </a>
              </div>
            )}
          </>
        )}
      </section>

      <section className="settings-section">
        <h3>LLM Rewriting</h3>

        <label>
          Backend:
          <select
            value={localConfig.llm.backend}
            onChange={(e) => updateLLM("backend", e.target.value)}
          >
            <option value="OpenAI">OpenAI</option>
            <option value="Ollama">Ollama (Local)</option>
            <option value="None">None (Skip rewriting)</option>
          </select>
        </label>

        {localConfig.llm.backend === "OpenAI" && (
          <>
            <label>
              OpenAI API Key:
              <input
                type="password"
                placeholder="sk-..."
                value={localConfig.llm.api_key || ""}
                onChange={(e) => updateLLM("api_key", e.target.value || null)}
              />
            </label>
            <label>
              Model:
              <select
                value={localConfig.llm.model}
                onChange={(e) => updateLLM("model", e.target.value)}
              >
                <option value="gpt-4o-mini">GPT-4o Mini (Recommended)</option>
                <option value="gpt-4o">GPT-4o</option>
                <option value="gpt-3.5-turbo">GPT-3.5 Turbo</option>
              </select>
            </label>
          </>
        )}

        {localConfig.llm.backend === "Ollama" && (
          <>
            {/* Ollama Status */}
            <div className="docker-status">
              <h4>Ollama Status</h4>
              {ollamaStatus ? (
                <>
                  <div
                    className={`status-badge ${ollamaStatus.available ? "status-ok" : "status-error"}`}
                  >
                    Ollama: {ollamaStatus.available ? "✓ Running" : "✗ Not Running"}
                  </div>
                  <p className="status-message">{ollamaStatus.message}</p>
                  <p className="status-url">URL: {ollamaStatus.base_url}</p>
                </>
              ) : (
                <div>Loading Ollama status...</div>
              )}
            </div>

            {!ollamaStatus?.available && (
              <div className="info-box warning">
                <strong>Ollama Required:</strong> Please install and start Ollama to use local LLM
                rewriting.
                <br />
                <a href="https://ollama.com/download" target="_blank" rel="noopener noreferrer">
                  Download Ollama
                </a>
                <br />
                <small>
                  After installation, run: <code>ollama serve</code>
                </small>
              </div>
            )}

            {/* Model Selection */}
            <label>
              Model:
              {ollamaModels.length > 0 ? (
                <select
                  value={localConfig.llm.model}
                  onChange={(e) => updateLLM("model", e.target.value)}
                >
                  {ollamaModels.map((model) => (
                    <option key={model.name} value={model.name}>
                      {model.name} - {model.size} {model.recommended ? "(Recommended)" : ""}
                    </option>
                  ))}
                </select>
              ) : (
                <input
                  type="text"
                  placeholder="llama3.2:3b"
                  value={localConfig.llm.model}
                  onChange={(e) => updateLLM("model", e.target.value)}
                />
              )}
            </label>

            {ollamaStatus?.available && ollamaModels.length === 0 && (
              <div className="info-box info">
                <strong>No Models Installed</strong>
                <p>Recommended models to install:</p>
                <ul>
                  {recommendedOllamaModels.map((model) => (
                    <li key={model.name}>
                      <strong>{model.name}</strong> ({model.size})
                      <br />
                      <code>ollama pull {model.name}</code>
                    </li>
                  ))}
                </ul>
              </div>
            )}

            <label>
              Base URL:
              <input
                type="text"
                value={localConfig.llm.base_url}
                onChange={(e) => updateLLM("base_url", e.target.value)}
              />
            </label>
          </>
        )}

        <label>
          Prompt Template:
          <select
            value={localConfig.llm.default_template}
            onChange={(e) => updateLLM("default_template", e.target.value)}
          >
            <option value="minimal">Minimal</option>
            <option value="balanced">Balanced (Default)</option>
            <option value="professional">Professional</option>
          </select>
        </label>
      </section>

      <section className="settings-section">
        <h3>Text Injection</h3>

        <label>
          Method:
          <select
            value={localConfig.injection.method}
            onChange={(e) =>
              setLocalConfig({
                ...localConfig,
                injection: {
                  ...localConfig.injection,
                  method: e.target.value as "Hybrid" | "Clipboard" | "Typing",
                },
              })
            }
          >
            <option value="Hybrid">Hybrid (Recommended)</option>
            <option value="Clipboard">Clipboard</option>
            <option value="Typing">Typing</option>
          </select>
        </label>
      </section>

      <div className="settings-actions">
        <button className="btn btn-secondary" onClick={onClose}>
          Cancel
        </button>
        <button className="btn btn-primary" onClick={handleSave}>
          Save
        </button>
      </div>
    </div>
  );
}
