# Contributing to Open WhisperFlow

First off, thank you for considering contributing to Open WhisperFlow! 🎉

It's people like you that make Open WhisperFlow such a great tool. We welcome contributions from everyone, whether you're fixing a typo, adding a feature, or helping with documentation.

---

## 📋 Table of Contents

1. [Code of Conduct](#code-of-conduct)
2. [Getting Started](#getting-started)
3. [Development Process](#development-process)
4. [Coding Standards](#coding-standards)
5. [Documentation Standards](#documentation-standards)
6. [Commit Message Guidelines](#commit-message-guidelines)
7. [Pull Request Process](#pull-request-process)
8. [Testing Guidelines](#testing-guidelines)
9. [Community](#community)

---

## Code of Conduct

This project and everyone participating in it is governed by our commitment to:
- **Be respectful** and inclusive
- **Be patient** with newcomers
- **Be collaborative** and help each other
- **Focus on what is best** for the community
- **Show empathy** towards other community members

Please report unacceptable behavior to [maintainer email - TBD].

---

## Getting Started

### Prerequisites

Before you begin, ensure you have:
- **Node.js 22+** installed
- **Rust 1.91+** installed
- **Git** for version control
- **System dependencies** (see README.md for platform-specific requirements)

### First Time Setup

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/open-whisperflow.git
   cd open-whisperflow
   ```

3. **Add upstream remote**:
   ```bash
   git remote add upstream https://github.com/open-whisperflow/open-whisperflow.git
   ```

4. **Install dependencies**:
   ```bash
   npm install
   ```

5. **Run in development mode**:
   ```bash
   npm run tauri:dev
   ```

6. **Verify tests pass**:
   ```bash
   npm run lint
   npm run type-check
   npm test
   cd src-tauri && cargo clippy
   ```

---

## Development Process

### 1. Find Something to Work On

- Check [TODO.md](TODO.md) for planned features and known bugs
- Look at [open issues](https://github.com/open-whisperflow/open-whisperflow/issues)
- Check the roadmap in [COMPREHENSIVE_PLAN.md](COMPREHENSIVE_PLAN.md)
- Have your own idea? Create an issue first to discuss it!

### 2. Create a Branch

```bash
# Sync with upstream
git fetch upstream
git checkout main
git merge upstream/main

# Create your feature branch
git checkout -b feature/your-feature-name
# or
git checkout -b fix/bug-description
```

**Branch Naming:**
- `feature/` - New features
- `fix/` - Bug fixes
- `refactor/` - Code refactoring
- `docs/` - Documentation changes
- `test/` - Test additions/changes

### 3. Make Your Changes

- **Read [CLAUDE.md](CLAUDE.md)** first - contains critical architecture info
- **Write tests** for your changes
- **Update documentation** as you code
- **Follow coding standards** (see below)
- **Commit frequently** with clear messages

### 4. Test Your Changes

```bash
# Frontend linting
npm run lint
npm run type-check
npm test

# Backend linting
cd src-tauri
cargo clippy -- -D warnings
cargo fmt --check
cargo test

# Integration test
cd ..
npm run tauri:dev
# Manually test your feature
```

### 5. Document Your Changes

- **Update CHANGELOG.md** with your changes
- **Update TODO.md** if applicable
- **Update relevant .md files** in docs/
- **Add inline code comments** for complex logic
- **Update API documentation** if you changed interfaces

---

## Coding Standards

### Rust

**Follow Rust conventions:**
```rust
// Good: Clear naming, proper error handling
pub async fn transcribe_audio(path: &Path) -> Result<String> {
    let data = tokio::fs::read(path).await?;
    // ... process data ...
    Ok(transcription)
}

// Bad: Unclear naming, uses unwrap
pub async fn process(p: &Path) -> String {
    let data = tokio::fs::read(p).await.unwrap();  // Can panic!
    // ...
}
```

**Key Points:**
- Use `cargo fmt` before committing
- Run `cargo clippy` and fix all warnings
- Never use `.unwrap()` in production code
- All public items need doc comments (`///`)
- Use `Result<T>` for error handling
- Mark I/O functions as `async`

### TypeScript

**Follow TypeScript conventions:**
```typescript
// Good: Typed props, functional component
interface ButtonProps {
  onClick: () => void;
  disabled?: boolean;
}

export default function Button({ onClick, disabled = false }: ButtonProps) {
  return <button onClick={onClick} disabled={disabled}>Click</button>;
}

// Bad: any types, unclear
export default function Button(props: any) {
  return <button onClick={props.onClick}>Click</button>;
}
```

**Key Points:**
- Enable strict mode (already configured)
- Avoid `any` types - use proper TypeScript types
- Use functional components with hooks
- Extract reusable logic into custom hooks
- Props must be typed with interfaces

### File Organization

```
src/
  components/
    ComponentName.tsx       # Component logic
    ComponentName.css       # Component styles
    ComponentName.test.tsx  # Component tests (if exists)
```

```
src-tauri/src/
  module_name/
    mod.rs        # Public API
    types.rs      # Type definitions
    backend.rs    # Implementation
    tests.rs      # Tests
```

---

## Documentation Standards

### Code Comments

**When to comment:**
- Complex algorithms
- Non-obvious design decisions
- Workarounds for known issues
- Public APIs (always)

**When NOT to comment:**
- Obvious code (let code speak for itself)
- Redundant information

```rust
// GOOD: Explains WHY
// We use exponential backoff because the API rate limits are strict
// and linear retry would hit the limit too quickly
let delay = base_delay * 2_u64.pow(attempt);

// BAD: Explains WHAT (obvious)
// Set delay to base delay times 2 to the power of attempt
let delay = base_delay * 2_u64.pow(attempt);
```

### API Documentation

```rust
/// Transcribe audio file to text using configured backend.
///
/// This function handles backend selection, error recovery, and
/// automatic retries with exponential backoff.
///
/// # Arguments
/// * `audio_path` - Path to WAV file (16kHz, 16-bit, mono)
///
/// # Returns
/// * `Ok(String)` - Successfully transcribed text
/// * `Err(AppError::AudioRecording)` - Invalid audio file
/// * `Err(AppError::Transcription)` - Transcription failed
/// * `Err(AppError::Network)` - Network error (with automatic retry)
///
/// # Example
/// ```rust
/// let service = TranscriptionService::new(&config)?;
/// let text = service.transcribe(Path::new("recording.wav")).await?;
/// println!("Transcribed: {}", text);
/// ```
pub async fn transcribe(&self, audio_path: &Path) -> Result<String> {
    // Implementation
}
```

### README and Markdown Files

- Use clear headings (`#`, `##`, `###`)
- Include code examples
- Add screenshots for UI changes
- Keep line length reasonable (<100 chars when possible)
- Use relative links for internal docs
- Always include a table of contents for long docs

---

## Commit Message Guidelines

We follow [Conventional Commits](https://www.conventionalcommits.org/).

### Format

```
<type>(<scope>): <short summary>

<optional detailed description>

<optional footer>
```

### Types

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only
- `style`: Code style (formatting, no logic change)
- `refactor`: Code restructuring (no feature/bug change)
- `perf`: Performance improvement
- `test`: Adding or updating tests
- `chore`: Maintenance (deps, build config, etc.)

### Examples

```bash
# Simple feature
git commit -m "feat(audio): add support for MP3 input files"

# Bug fix with details
git commit -m "fix(injection): handle special characters in clipboard mode

Some applications reject pasted text containing newlines or special
characters. Now we properly escape these before clipboard injection.

Fixes #42"

# Breaking change
git commit -m "feat(config)!: change config format to TOML

BREAKING CHANGE: Configuration format has changed from JSON to TOML.
Users must migrate their config files manually.

Migration guide: docs/migration-v1-to-v2.md"

# Multiple changes
git commit -m "refactor(llm): improve error handling

- Add retry logic with exponential backoff
- Better error messages for common failures
- Add timeout handling
- Improve logging

Related to #38"
```

### Scope Examples

- `audio` - Audio recording
- `transcription` - Whisper integration
- `llm` - LLM/text rewriting
- `injection` - Text insertion
- `ui` - User interface
- `config` - Configuration
- `build` - Build system
- `deps` - Dependencies

---

## Pull Request Process

### Before Submitting

**Checklist:**
- [ ] Code follows style guidelines
- [ ] All tests pass
- [ ] New tests added for new features
- [ ] Documentation updated
- [ ] CHANGELOG.md updated
- [ ] TODO.md updated (if applicable)
- [ ] No linting errors
- [ ] Commit messages follow guidelines

### Creating a Pull Request

1. **Push your branch** to your fork:
   ```bash
   git push origin feature/your-feature-name
   ```

2. **Open a Pull Request** on GitHub:
   - Use the PR template
   - Reference related issues (`Fixes #42`, `Related to #38`)
   - Describe what changed and why
   - Add screenshots for UI changes
   - List breaking changes (if any)

3. **Respond to feedback**:
   - Address review comments promptly
   - Make requested changes
   - Update docs if needed
   - Push changes to same branch

### PR Template

```markdown
## Description
Brief description of what this PR does.

## Type of Change
- [ ] Bug fix (non-breaking change)
- [ ] New feature (non-breaking change)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] Documentation update

## Related Issues
Fixes #42
Related to #38

## Changes Made
- Added X feature
- Fixed Y bug
- Refactored Z module

## Testing
- [ ] Tested on Windows
- [ ] Tested on Linux
- [ ] Tested on macOS
- [ ] Added unit tests
- [ ] Added integration tests

## Screenshots (if applicable)
[Add screenshots here]

## Breaking Changes
None / [Describe breaking changes]

## Checklist
- [ ] Code follows style guidelines
- [ ] Tests pass locally
- [ ] Documentation updated
- [ ] CHANGELOG.md updated
```

### Review Process

1. **Automated checks** must pass (CI/CD)
2. **At least one approval** from maintainer
3. **No unresolved conversations**
4. **Up to date** with main branch

---

## Testing Guidelines

### Unit Tests

**Rust:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_transcription_success() {
        let service = create_test_service();
        let result = service.transcribe(Path::new("test.wav")).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_transcription_invalid_file() {
        let service = create_test_service();
        let result = service.transcribe(Path::new("invalid.wav")).await;
        assert!(matches!(result, Err(AppError::AudioRecording(_))));
    }
}
```

**TypeScript:**
```typescript
import { render, fireEvent } from "@testing-library/react";
import RecordingButton from "./RecordingButton";

describe("RecordingButton", () => {
  it("calls onStart when clicked while idle", () => {
    const onStart = vi.fn();
    const { getByRole } = render(
      <RecordingButton isRecording={false} isProcessing={false} onStart={onStart} onStop={vi.fn()} />
    );

    fireEvent.click(getByRole("button"));
    expect(onStart).toHaveBeenCalledOnce();
  });
});
```

### Integration Tests

Test full workflows:
- Record audio → Transcribe → Rewrite → Inject
- Configuration save/load
- Backend switching
- Error recovery

### Manual Testing

For UI changes, test on:
- Different screen sizes
- Dark and light themes
- Different operating systems
- With and without network access

---

## Community

### Getting Help

- **Documentation**: Start with README.md and CLAUDE.md
- **GitHub Discussions**: For questions and ideas
- **GitHub Issues**: For bugs and feature requests
- **Discord** (future): For real-time chat

### Reporting Bugs

Use the **bug report template** in GitHub issues:

```markdown
**Describe the bug**
A clear description of what the bug is.

**To Reproduce**
Steps to reproduce:
1. Go to '...'
2. Click on '...'
3. See error

**Expected behavior**
What you expected to happen.

**Actual behavior**
What actually happened.

**Screenshots**
If applicable, add screenshots.

**Environment:**
- OS: [e.g. Ubuntu 22.04]
- Version: [e.g. 0.1.0]
- Installation method: [e.g. built from source]

**Additional context**
Any other relevant information.
```

### Requesting Features

Use the **feature request template**:

```markdown
**Is your feature request related to a problem?**
A clear description of the problem.

**Describe the solution you'd like**
What you want to happen.

**Describe alternatives you've considered**
Other solutions you've thought about.

**Additional context**
Any other relevant information.
```

---

## Recognition

Contributors will be:
- Listed in AUTHORS.md (when created)
- Mentioned in release notes
- Given credit in commit history
- Invited to contributor discussions

---

## Questions?

- **General questions**: GitHub Discussions
- **Bug reports**: GitHub Issues
- **Security issues**: [security email - TBD]
- **Other**: [contact email - TBD]

---

Thank you for contributing to Open WhisperFlow! 🎉

Every contribution, no matter how small, makes a difference. We appreciate your time and effort in helping make this project better for everyone.

**Happy coding! 🚀**
