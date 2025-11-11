# Testing Guide for Open WhisperFlow

This document describes the testing strategy, tools, and procedures for Open WhisperFlow.

## Overview

Open WhisperFlow uses a multi-layered testing approach:
- **Unit Tests**: Test individual components and functions
- **Integration Tests**: Test interactions between components
- **End-to-End Tests**: Test complete user workflows
- **Manual Testing**: GUI and real-world usage testing

---

## Frontend Testing (React + TypeScript)

### Tools
- **Vitest**: Fast unit test runner (Vite-native)
- **React Testing Library**: Component testing utilities
- **jsdom**: DOM environment for tests

### Running Frontend Tests

```bash
# Run all tests
npm test

# Run tests in watch mode
npm test -- --watch

# Run tests with coverage
npm test -- --coverage

# Run specific test file
npm test RecordingButton.test.tsx
```

### Test Files Location
```
src/
├── components/
│   ├── RecordingButton.tsx
│   ├── RecordingButton.test.tsx     # ✅ Component tests
│   ├── StatusIndicator.tsx
│   ├── StatusIndicator.test.tsx     # ✅ Component tests
│   └── ...
└── test/
    ├── setup.ts                      # Test configuration
    └── mocks/
        └── tauri.ts                  # Mock Tauri API
```

### Writing Component Tests

**Example: Testing a Button Component**

```typescript
import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import RecordingButton from "./RecordingButton";

describe("RecordingButton", () => {
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
  });
});
```

### Mocking Tauri APIs

For tests that invoke Tauri commands:

```typescript
import { vi } from "vitest";

// Mock the Tauri invoke function
vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn().mockImplementation((command) => {
    if (command === "get_config") {
      return Promise.resolve({ /* mock config */ });
    }
    return Promise.resolve(null);
  }),
}));
```

---

## Backend Testing (Rust)

### Tools
- **cargo test**: Built-in Rust test runner
- **tokio::test**: Async test runtime

### Running Backend Tests

```bash
# Run all Rust tests
cd src-tauri && cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_default_config_creation

# Run tests with coverage (requires cargo-tarpaulin)
cargo tarpaulin --out Html
```

### Test Files Location
```
src-tauri/src/
├── config/
│   └── mod.rs                        # ✅ Includes #[cfg(test)] mod tests
├── audio/
│   └── mod.rs                        # TODO: Add tests
├── transcription/
│   └── mod.rs                        # TODO: Add tests
└── ...
```

### Writing Rust Tests

**Example: Config Tests**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_creation() {
        let config = AppConfig::default();

        assert_eq!(config.audio.sample_rate, 16000);
        assert_eq!(config.audio.channels, 1);
    }

    #[tokio::test]
    async fn test_async_function() {
        let result = some_async_function().await;
        assert!(result.is_ok());
    }
}
```

### Testing Guidelines

1. **Unit Tests**: Test each function in isolation
2. **Error Handling**: Test both success and failure cases
3. **Edge Cases**: Test boundary conditions
4. **Async Code**: Use `#[tokio::test]` for async tests
5. **Mock External Services**: Don't make real API calls in tests

---

## Integration Testing

Integration tests verify that different parts of the system work together.

### Frontend Integration Tests

Test complete user workflows:

```typescript
describe("Recording Workflow", () => {
  it("completes full recording cycle", async () => {
    const { container } = render(<App />);

    // 1. Start recording
    const recordButton = screen.getByRole("button", { name: /record/i });
    fireEvent.click(recordButton);

    // 2. Verify recording state
    expect(screen.getByText("Recording...")).toBeInTheDocument();

    // 3. Stop recording
    fireEvent.click(recordButton);

    // 4. Verify processing
    expect(screen.getByText("Transcribing...")).toBeInTheDocument();
  });
});
```

### Backend Integration Tests

Test command interactions:

```rust
#[tokio::test]
async fn test_full_transcription_pipeline() {
    let state = AppState::new().unwrap();
    let config = state.config.read().await;

    // Test transcription service initialization
    let service = TranscriptionService::new(&config.transcription).await;
    assert!(service.is_ok());

    // Test backend availability check
    let backend_available = service.unwrap().is_available().await;
    assert!(backend_available);
}
```

---

## End-to-End Testing

E2E tests simulate real user interactions with the application.

### Manual E2E Testing Checklist

**Phase 1: Setup**
- [ ] Fresh install
- [ ] First-run wizard appears
- [ ] Can complete cloud setup with API key
- [ ] Can complete local setup with Docker

**Phase 2: Core Functionality**
- [ ] Start recording
- [ ] Stop recording
- [ ] Audio transcription works
- [ ] Text rewriting works
- [ ] Text injection works (clipboard, typing, hybrid)
- [ ] Cancel recording works

**Phase 3: Settings**
- [ ] Change transcription backend (OpenAI ↔ faster-whisper)
- [ ] Change LLM backend (OpenAI ↔ Ollama ↔ None)
- [ ] Update API keys
- [ ] Change injection method
- [ ] Save and reload settings

**Phase 4: Docker Integration**
- [ ] Check Docker status
- [ ] Start/stop faster-whisper container
- [ ] Select different Whisper models
- [ ] GPU detection works (if NVIDIA GPU available)

**Phase 5: Ollama Integration**
- [ ] Check Ollama status
- [ ] List installed models
- [ ] See recommended models
- [ ] Test with local LLM

**Phase 6: Error Handling**
- [ ] Invalid API key error message
- [ ] Docker not running warning
- [ ] Ollama not available warning
- [ ] Recording timeout handling
- [ ] Network failure recovery

---

## Continuous Integration (CI)

### GitHub Actions Workflow

The CI pipeline runs on every push and pull request:

```yaml
name: CI

on: [push, pull_request]

jobs:
  test-frontend:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-node@v3
      - run: npm ci
      - run: npm run lint
      - run: npm run type-check
      - run: npm test
      - run: npm run build

  test-backend:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
      - run: cd src-tauri && cargo test
      - run: cd src-tauri && cargo clippy
```

---

## Test Coverage Goals

### Current Coverage
- **Frontend**: Unit tests for core components (RecordingButton, StatusIndicator)
- **Backend**: Unit tests for config module

### Target Coverage
- **Frontend**: 80% line coverage
- **Backend**: 70% line coverage
- **Critical Paths**: 100% coverage (recording, transcription, injection)

### Measuring Coverage

**Frontend:**
```bash
npm test -- --coverage
# Opens coverage/index.html
```

**Backend:**
```bash
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
# Opens tarpaulin-report.html
```

---

## Testing Best Practices

### General Principles
1. **Test Behavior, Not Implementation**: Focus on what the code does, not how it does it
2. **Arrange-Act-Assert**: Structure tests with setup, action, and verification
3. **One Assertion per Test**: Keep tests focused and easy to debug
4. **Descriptive Test Names**: Use clear, descriptive test names
5. **Fast Tests**: Tests should run quickly (< 1s per test)

### React Component Testing
```typescript
// ✅ Good: Test user-facing behavior
it("shows error message when API key is invalid", () => {
  render(<ApiKeyInput value="invalid" />);
  expect(screen.getByText(/invalid api key/i)).toBeInTheDocument();
});

// ❌ Bad: Test implementation details
it("sets error state to true", () => {
  const wrapper = shallow(<ApiKeyInput value="invalid" />);
  expect(wrapper.state("hasError")).toBe(true);
});
```

### Rust Testing
```rust
// ✅ Good: Test public API
#[test]
fn test_config_loads_defaults() {
    let config = AppConfig::default();
    assert_eq!(config.audio.sample_rate, 16000);
}

// ❌ Bad: Test private implementation
#[test]
fn test_internal_parser_state() {
    let parser = ConfigParser::new();
    assert_eq!(parser.internal_state, InternalState::Ready);
}
```

---

## Debugging Tests

### Frontend
```bash
# Run single test with detailed output
npm test -- RecordingButton.test.tsx --reporter=verbose

# Debug in Chrome DevTools
npm test -- --inspect-brk

# Use console.log (visible with --reporter=verbose)
console.log("Debug value:", someVariable);
```

### Backend
```bash
# Show test output
cargo test -- --nocapture

# Run single test with backtrace
RUST_BACKTRACE=1 cargo test test_name

# Use dbg! macro
dbg!(&config);
```

---

## Common Issues and Solutions

### Issue: Tests Fail in CI but Pass Locally
**Solution**: Check for:
- Timing issues (use proper async/await)
- File system differences (use platform-agnostic paths)
- Environment variables not set in CI

### Issue: Async Tests Timeout
**Solution**: Increase timeout or check for unresolved promises
```typescript
it("async operation", async () => {
  // Increase timeout to 10s
  vi.setConfig({ testTimeout: 10000 });
  await someAsyncOperation();
});
```

### Issue: Flaky Tests
**Solution**:
- Avoid time-dependent assertions
- Use proper mocking for external dependencies
- Wait for async operations to complete

---

## Contributing Tests

When adding new features:
1. **Write tests first** (TDD approach recommended)
2. **Test happy path** (normal usage)
3. **Test error cases** (what can go wrong?)
4. **Test edge cases** (boundary conditions)
5. **Update this guide** (document new testing patterns)

### Pull Request Checklist
- [ ] All new code has tests
- [ ] All tests pass locally
- [ ] Test coverage hasn't decreased
- [ ] Tests are documented
- [ ] CI pipeline passes

---

## Resources

- **Vitest**: https://vitest.dev/
- **React Testing Library**: https://testing-library.com/react
- **Rust Testing**: https://doc.rust-lang.org/book/ch11-00-testing.html
- **Tauri Testing**: https://tauri.app/v1/guides/testing/

---

**Last Updated**: 2025-11-11
**Maintainer**: Open WhisperFlow Team
