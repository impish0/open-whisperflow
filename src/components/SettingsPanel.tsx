import { useState } from "react";
import { AppConfig } from "@/types";
import "./SettingsPanel.css";

interface SettingsPanelProps {
  config: AppConfig;
  onUpdate: (config: AppConfig) => void;
  onClose: () => void;
}

export default function SettingsPanel({ config, onUpdate, onClose }: SettingsPanelProps) {
  const [localConfig, setLocalConfig] = useState<AppConfig>(config);

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
            <option value="OpenAI">OpenAI Whisper API</option>
            <option value="FasterWhisper">faster-whisper (Local)</option>
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
              />
            </label>
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
            <label>
              Model:
              <input
                type="text"
                placeholder="llama3.2:3b"
                value={localConfig.llm.model}
                onChange={(e) => updateLLM("model", e.target.value)}
              />
            </label>
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
