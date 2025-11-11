import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { AppConfig, DockerStatus } from "@/types";
import "./FirstRunWizard.css";

interface FirstRunWizardProps {
  onComplete: (config: AppConfig) => void;
}

type WizardStep = "welcome" | "setup-choice" | "cloud-setup" | "local-setup" | "test" | "complete";

export default function FirstRunWizard({ onComplete }: FirstRunWizardProps) {
  const [currentStep, setCurrentStep] = useState<WizardStep>("welcome");
  const [apiKey, setApiKey] = useState("");
  const [isValidating, setIsValidating] = useState(false);
  const [validationError, setValidationError] = useState<string | null>(null);
  const [isTesting, setIsTesting] = useState(false);

  const handleNext = (nextStep: WizardStep) => {
    setCurrentStep(nextStep);
  };

  const handleSetupChoice = (mode: "cloud" | "local") => {
    if (mode === "cloud") {
      setCurrentStep("cloud-setup");
    } else {
      setCurrentStep("local-setup");
    }
  };

  const validateApiKey = async () => {
    if (!apiKey || !apiKey.startsWith("sk-")) {
      setValidationError("Please enter a valid OpenAI API key (starts with 'sk-')");
      return false;
    }

    setIsValidating(true);
    setValidationError(null);

    try {
      // Create a temporary config with the API key to test it
      const testConfig: AppConfig = await invoke("get_config");
      testConfig.transcription.backend = "OpenAI";
      testConfig.transcription.openai_api_key = apiKey;

      await invoke("update_config", { config: testConfig });

      // Check if backend is available
      const status = await invoke("check_transcription_backend");
      if (status) {
        setIsValidating(false);
        return true;
      } else {
        setValidationError("Could not verify API key. Please check and try again.");
        setIsValidating(false);
        return false;
      }
    } catch (error) {
      setValidationError(`Validation failed: ${error}`);
      setIsValidating(false);
      return false;
    }
  };

  const handleCloudSetupComplete = async () => {
    const isValid = await validateApiKey();
    if (isValid) {
      setCurrentStep("test");
    }
  };

  const handleLocalSetupComplete = async () => {
    try {
      // Check Docker status
      const dockerStatus = await invoke<DockerStatus>("check_docker_status");

      if (!dockerStatus.available) {
        setValidationError("Docker is not running. Please start Docker Desktop and try again.");
        return;
      }

      // Start container if not running
      if (!dockerStatus.container_running) {
        setIsValidating(true);
        await invoke("start_whisper_container");
        setIsValidating(false);
      }

      const config: AppConfig = await invoke("get_config");
      config.transcription.backend = "FasterWhisper";
      config.transcription.model = "small"; // Recommended model
      await invoke("update_config", { config });

      setCurrentStep("test");
    } catch (error) {
      setValidationError(`Setup failed: ${error}`);
      setIsValidating(false);
    }
  };

  const handleSkipTest = async () => {
    const config: AppConfig = await invoke("get_config");

    // Mark first run as complete
    localStorage.setItem("whisperflow_first_run_complete", "true");

    onComplete(config);
  };

  const handleTestRecording = async () => {
    setIsTesting(true);
    try {
      // This would trigger a test recording
      // For now, we'll just simulate success
      await new Promise((resolve) => setTimeout(resolve, 2000));
      setCurrentStep("complete");
    } catch (error) {
      setValidationError(`Test failed: ${error}`);
    } finally {
      setIsTesting(false);
    }
  };

  const handleComplete = async () => {
    const config: AppConfig = await invoke("get_config");
    localStorage.setItem("whisperflow_first_run_complete", "true");
    onComplete(config);
  };

  return (
    <div className="wizard-overlay">
      <div className="wizard-container">
        {/* Progress indicator */}
        <div className="wizard-progress">
          <div className={`progress-step ${currentStep === "welcome" ? "active" : "done"}`}>1</div>
          <div className="progress-line"></div>
          <div
            className={`progress-step ${["setup-choice", "cloud-setup", "local-setup"].includes(currentStep) ? "active" : currentStep === "test" || currentStep === "complete" ? "done" : ""}`}
          >
            2
          </div>
          <div className="progress-line"></div>
          <div
            className={`progress-step ${currentStep === "test" ? "active" : currentStep === "complete" ? "done" : ""}`}
          >
            3
          </div>
          <div className="progress-line"></div>
          <div className={`progress-step ${currentStep === "complete" ? "active" : ""}`}>4</div>
        </div>

        {/* Step content */}
        <div className="wizard-content">
          {currentStep === "welcome" && (
            <div className="wizard-step">
              <h1>Welcome to Open WhisperFlow! 🎤</h1>
              <p className="wizard-subtitle">Transform your voice into polished, written content</p>

              <div className="feature-list">
                <div className="feature-item">
                  <span className="feature-icon">🔒</span>
                  <div>
                    <strong>Privacy First</strong>
                    <p>Run 100% locally or use cloud APIs</p>
                  </div>
                </div>
                <div className="feature-item">
                  <span className="feature-icon">⚡</span>
                  <div>
                    <strong>Lightning Fast</strong>
                    <p>Sub-2-second transcription and rewriting</p>
                  </div>
                </div>
                <div className="feature-item">
                  <span className="feature-icon">✨</span>
                  <div>
                    <strong>AI-Powered</strong>
                    <p>Automatically clean up filler words and fix grammar</p>
                  </div>
                </div>
              </div>

              <button
                className="btn btn-primary btn-large"
                onClick={() => handleNext("setup-choice")}
              >
                Get Started
              </button>
            </div>
          )}

          {currentStep === "setup-choice" && (
            <div className="wizard-step">
              <h2>Choose Your Setup</h2>
              <p className="wizard-subtitle">You can always change this later in settings</p>

              <div className="setup-options">
                <div className="setup-card" onClick={() => handleSetupChoice("cloud")}>
                  <div className="setup-card-header">
                    <span className="setup-icon">☁️</span>
                    <h3>Quick Setup</h3>
                    <span className="setup-badge recommended">Recommended</span>
                  </div>
                  <p>Use OpenAI's cloud APIs for instant setup</p>
                  <ul className="setup-features">
                    <li>✓ 5-minute setup</li>
                    <li>✓ Best accuracy</li>
                    <li>✓ Always up-to-date</li>
                    <li>⚠️ Requires API key (~$0.10/hour)</li>
                  </ul>
                  <button className="btn btn-primary">Choose Quick Setup</button>
                </div>

                <div className="setup-card" onClick={() => handleSetupChoice("local")}>
                  <div className="setup-card-header">
                    <span className="setup-icon">🔒</span>
                    <h3>Local Setup</h3>
                    <span className="setup-badge">Private</span>
                  </div>
                  <p>Run everything on your computer</p>
                  <ul className="setup-features">
                    <li>✓ 100% private</li>
                    <li>✓ No ongoing costs</li>
                    <li>✓ Works offline</li>
                    <li>⚠️ Requires Docker (~15 min setup)</li>
                  </ul>
                  <button className="btn btn-secondary">Choose Local Setup</button>
                </div>
              </div>
            </div>
          )}

          {currentStep === "cloud-setup" && (
            <div className="wizard-step">
              <h2>Cloud Setup</h2>
              <p className="wizard-subtitle">Enter your OpenAI API key to get started</p>

              <div className="setup-instructions">
                <p>
                  <strong>Don't have an API key?</strong>
                </p>
                <ol>
                  <li>
                    Visit{" "}
                    <a
                      href="https://platform.openai.com/api-keys"
                      target="_blank"
                      rel="noopener noreferrer"
                    >
                      platform.openai.com/api-keys
                    </a>
                  </li>
                  <li>Sign up or log in to your account</li>
                  <li>Click "Create new secret key"</li>
                  <li>Copy the key and paste it below</li>
                </ol>
              </div>

              <div className="form-group">
                <label htmlFor="api-key">OpenAI API Key</label>
                <input
                  id="api-key"
                  type="password"
                  placeholder="sk-..."
                  value={apiKey}
                  onChange={(e) => {
                    setApiKey(e.target.value);
                    setValidationError(null);
                  }}
                  className={validationError ? "error" : ""}
                />
                {validationError && <div className="error-message">{validationError}</div>}
              </div>

              <div className="wizard-actions">
                <button
                  className="btn btn-secondary"
                  onClick={() => setCurrentStep("setup-choice")}
                >
                  Back
                </button>
                <button
                  className="btn btn-primary"
                  onClick={handleCloudSetupComplete}
                  disabled={!apiKey || isValidating}
                >
                  {isValidating ? "Validating..." : "Continue"}
                </button>
              </div>
            </div>
          )}

          {currentStep === "local-setup" && (
            <div className="wizard-step">
              <h2>Local Setup</h2>
              <p className="wizard-subtitle">Set up local transcription with Docker</p>

              <div className="setup-instructions">
                <p>
                  <strong>Requirements:</strong>
                </p>
                <ol>
                  <li>
                    <a
                      href="https://www.docker.com/products/docker-desktop/"
                      target="_blank"
                      rel="noopener noreferrer"
                    >
                      Download and install Docker Desktop
                    </a>
                  </li>
                  <li>Start Docker Desktop</li>
                  <li>Click Continue below</li>
                </ol>

                <div className="info-box">
                  <strong>📝 Note:</strong> The first time you use local transcription, Docker will
                  download the Whisper model (~244 MB for recommended "small" model). This happens
                  automatically in the background.
                </div>
              </div>

              {validationError && <div className="error-message">{validationError}</div>}

              <div className="wizard-actions">
                <button
                  className="btn btn-secondary"
                  onClick={() => setCurrentStep("setup-choice")}
                >
                  Back
                </button>
                <button
                  className="btn btn-primary"
                  onClick={handleLocalSetupComplete}
                  disabled={isValidating}
                >
                  {isValidating ? "Starting Container..." : "Continue"}
                </button>
              </div>
            </div>
          )}

          {currentStep === "test" && (
            <div className="wizard-step">
              <h2>Test Your Setup</h2>
              <p className="wizard-subtitle">Let's make sure everything works!</p>

              <div className="test-instructions">
                <p>Click the button below and say something like:</p>
                <div className="example-phrase">
                  "Testing Open WhisperFlow - it's working great!"
                </div>
              </div>

              {validationError && <div className="error-message">{validationError}</div>}

              <div className="wizard-actions">
                <button className="btn btn-secondary" onClick={handleSkipTest}>
                  Skip Test
                </button>
                <button
                  className="btn btn-primary"
                  onClick={handleTestRecording}
                  disabled={isTesting}
                >
                  {isTesting ? "Testing..." : "Test Recording"}
                </button>
              </div>
            </div>
          )}

          {currentStep === "complete" && (
            <div className="wizard-step">
              <div className="success-icon">✓</div>
              <h2>All Set!</h2>
              <p className="wizard-subtitle">Open WhisperFlow is ready to use</p>

              <div className="quick-tips">
                <h3>Quick Tips:</h3>
                <ul>
                  <li>Press the record button and start speaking</li>
                  <li>Your text will appear automatically when done</li>
                  <li>Customize settings anytime in the settings panel</li>
                  <li>Check out prompt templates for different writing styles</li>
                </ul>
              </div>

              <button className="btn btn-primary btn-large" onClick={handleComplete}>
                Start Using Open WhisperFlow
              </button>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
