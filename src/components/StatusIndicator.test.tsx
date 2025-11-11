import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/react";
import StatusIndicator from "./StatusIndicator";
import { RecordingState } from "@/types";

describe("StatusIndicator", () => {
  it("shows idle status when idle", () => {
    const state: RecordingState = { type: "Idle" };
    render(<StatusIndicator state={state} />);

    expect(screen.getByText("Ready to record")).toBeInTheDocument();
  });

  it("shows recording status when recording", () => {
    const state: RecordingState = {
      type: "Recording",
      data: { started_at: Date.now() },
    };
    render(<StatusIndicator state={state} />);

    expect(screen.getByText("Recording...")).toBeInTheDocument();
  });

  it("shows transcribing status when transcribing", () => {
    const state: RecordingState = {
      type: "Processing",
      data: { stage: "Transcribing" },
    };
    render(<StatusIndicator state={state} />);

    expect(screen.getByText("Transcribing...")).toBeInTheDocument();
  });

  it("shows rewriting status when rewriting", () => {
    const state: RecordingState = {
      type: "Processing",
      data: { stage: "Rewriting" },
    };
    render(<StatusIndicator state={state} />);

    expect(screen.getByText("Rewriting...")).toBeInTheDocument();
  });

  it("shows injecting status when injecting", () => {
    const state: RecordingState = {
      type: "Processing",
      data: { stage: "Injecting" },
    };
    render(<StatusIndicator state={state} />);

    expect(screen.getByText("Injecting text...")).toBeInTheDocument();
  });

  it("shows error status when error", () => {
    const state: RecordingState = {
      type: "Error",
      data: { message: "Test error" },
    };
    render(<StatusIndicator state={state} />);

    expect(screen.getByText("Error: Test error")).toBeInTheDocument();
  });

  it("applies correct CSS class for each status", () => {
    const idleState: RecordingState = { type: "Idle" };
    const { rerender, container } = render(<StatusIndicator state={idleState} />);
    expect(container.querySelector(".status-idle")).toBeInTheDocument();

    const recordingState: RecordingState = {
      type: "Recording",
      data: { started_at: Date.now() },
    };
    rerender(<StatusIndicator state={recordingState} />);
    expect(container.querySelector(".status-recording")).toBeInTheDocument();

    const processingState: RecordingState = {
      type: "Processing",
      data: { stage: "Transcribing" },
    };
    rerender(<StatusIndicator state={processingState} />);
    expect(container.querySelector(".status-processing")).toBeInTheDocument();
  });
});
