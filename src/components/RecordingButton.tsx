import "./RecordingButton.css";

interface RecordingButtonProps {
  isRecording: boolean;
  isProcessing: boolean;
  onStart: () => void;
  onStop: () => void;
}

export default function RecordingButton({
  isRecording,
  isProcessing,
  onStart,
  onStop,
}: RecordingButtonProps) {
  const handleClick = () => {
    if (isRecording) {
      onStop();
    } else if (!isProcessing) {
      onStart();
    }
  };

  return (
    <button
      className={`recording-button ${isRecording ? "recording" : ""} ${
        isProcessing ? "processing" : ""
      }`}
      onClick={handleClick}
      disabled={isProcessing}
      aria-label={isRecording ? "Stop recording" : "Start recording"}
    >
      <div className="mic-icon">{isProcessing ? "⏳" : isRecording ? "⏹️" : "🎤"}</div>
      <div className="button-text">
        {isProcessing ? "Processing..." : isRecording ? "Stop" : "Start Recording"}
      </div>
    </button>
  );
}
