import { RecordingState } from "@/types";
import "./StatusIndicator.css";

interface StatusIndicatorProps {
  state: RecordingState;
}

export default function StatusIndicator({ state }: StatusIndicatorProps) {
  const getStatusText = () => {
    switch (state.type) {
      case "Idle":
        return "Ready";
      case "Recording": {
        const elapsed = Math.floor((Date.now() - state.data.started_at) / 1000);
        return `Recording... (${elapsed}s)`;
      }
      case "Processing":
        return `Processing: ${state.data.stage}`;
      case "Error":
        return `Error: ${state.data.message}`;
    }
  };

  const getStatusClass = () => {
    switch (state.type) {
      case "Idle":
        return "status-idle";
      case "Recording":
        return "status-recording";
      case "Processing":
        return "status-processing";
      case "Error":
        return "status-error";
    }
  };

  return (
    <div className={`status-indicator ${getStatusClass()}`}>
      <div className="status-dot"></div>
      <div className="status-text">{getStatusText()}</div>
    </div>
  );
}
