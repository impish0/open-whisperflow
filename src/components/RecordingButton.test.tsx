import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import RecordingButton from "./RecordingButton";

describe("RecordingButton", () => {
  it("renders record button when not recording", () => {
    const onStart = vi.fn();
    const onStop = vi.fn();

    render(
      <RecordingButton
        isRecording={false}
        isProcessing={false}
        onStart={onStart}
        onStop={onStop}
      />
    );

    const button = screen.getByRole("button");
    expect(button).toBeInTheDocument();
    expect(button).toHaveClass("recording-button");
  });

  it("calls onStart when clicked while idle", () => {
    const onStart = vi.fn();
    const onStop = vi.fn();

    render(
      <RecordingButton
        isRecording={false}
        isProcessing={false}
        onStart={onStart}
        onStop={onStop}
      />
    );

    const button = screen.getByRole("button");
    fireEvent.click(button);

    expect(onStart).toHaveBeenCalledTimes(1);
    expect(onStop).not.toHaveBeenCalled();
  });

  it("calls onStop when clicked while recording", () => {
    const onStart = vi.fn();
    const onStop = vi.fn();

    render(
      <RecordingButton
        isRecording={true}
        isProcessing={false}
        onStart={onStart}
        onStop={onStop}
      />
    );

    const button = screen.getByRole("button");
    fireEvent.click(button);

    expect(onStop).toHaveBeenCalledTimes(1);
    expect(onStart).not.toHaveBeenCalled();
  });

  it("applies recording class when recording", () => {
    const onStart = vi.fn();
    const onStop = vi.fn();

    render(
      <RecordingButton
        isRecording={true}
        isProcessing={false}
        onStart={onStart}
        onStop={onStop}
      />
    );

    const button = screen.getByRole("button");
    expect(button).toHaveClass("recording");
  });

  it("applies processing class when processing", () => {
    const onStart = vi.fn();
    const onStop = vi.fn();

    render(
      <RecordingButton
        isRecording={false}
        isProcessing={true}
        onStart={onStart}
        onStop={onStop}
      />
    );

    const button = screen.getByRole("button");
    expect(button).toHaveClass("processing");
  });

  it("disables button when processing", () => {
    const onStart = vi.fn();
    const onStop = vi.fn();

    render(
      <RecordingButton
        isRecording={false}
        isProcessing={true}
        onStart={onStart}
        onStop={onStop}
      />
    );

    const button = screen.getByRole("button");
    expect(button).toBeDisabled();
  });
});
