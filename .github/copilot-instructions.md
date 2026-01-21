# Copilot Instructions for Foo Fight

## Project Overview
Foo Fight is a CLI/TUI speed typing trainer for commands written in Rust. It helps users memorize and quickly type commands from different tools (Git, Jujutsu, PowerShell, etc.) through an interactive terminal interface.

## Tech Stack
- **Language**: Rust (2021 edition)
- **TUI Framework**: ratatui 0.26
- **Terminal Backend**: crossterm 0.27
- **Serialization**: serde + toml
- **Error Handling**: anyhow
- **Random**: rand (for shuffling questions)

## Architecture

### File Structure
```
src/
├── main.rs         # Entry point, event loop
├── app.rs          # Application state machine
├── dict.rs         # Dictionary parsing and validation
├── engine.rs       # Training logic and statistics
└── ui.rs           # TUI rendering with ratatui

dictionaries/       # TOML format dictionaries
├── git.toml
├── jujutsu.toml
└── powershell_pester.toml
```

## Coding Standards

### Rust Conventions
- Follow Rust 2021 edition idioms
- Use `cargo fmt` for formatting (enforced in CI)
- Run `clippy` for linting (enforced in CI)
- Prefer idiomatic error handling with `anyhow::Result`
- Use descriptive variable names

### Error Handling
- Use `anyhow::Result<T>` for functions that can fail
- Provide context with `.context()` when propagating errors
- Handle errors gracefully in the TUI to avoid crashes

### Code Organization
- Keep state management in `app.rs`
- Business logic in `engine.rs`
- UI rendering separate in `ui.rs`
- Data structures and parsing in `dict.rs`

## Development Workflow

### Building and Running
```bash
cargo build          # Development build
cargo build --release # Production build
cargo run            # Run the application
```

### Testing
```bash
cargo test           # Run all tests
cargo test --lib     # Run library tests only
```

### Linting
```bash
cargo fmt            # Format code
cargo clippy         # Run linter
```

## Dictionary Format

Dictionaries are stored in TOML format in the `dictionaries/` directory:

```toml
name = "Dictionary Name"
version = 1
language = "fr"  # or "en"

[[items]]
id = "unique_id"
prompt = "Question or description"
answer = "expected command"
aliases = ["alt1", "alt2"]  # Optional alternative answers
tags = ["tag1", "tag2"]     # Optional for future filtering
difficulty = 1              # Optional: 1-3
```

### Dictionary Guidelines
- Each item must have a unique `id`
- `prompt` describes what the command does
- `answer` is the exact command expected
- `aliases` are alternative valid answers
- Keep dictionaries focused on specific tools/topics

## UI/UX Patterns

### TUI Design with ratatui
- Use `ratatui::widgets` for consistent UI components
- Handle terminal events with crossterm
- Implement responsive layouts that work in different terminal sizes
- Provide clear visual feedback for user actions

### User Input
- Handle keyboard input through crossterm events
- Support common shortcuts (Esc, Ctrl+C for exit)
- Validate input before processing
- Show clear error messages

## Testing Guidelines
- Write unit tests for business logic in `engine.rs`
- Test dictionary parsing edge cases
- Mock user input for integration tests where needed
- Keep tests fast and focused

## CI/CD
- All pushes trigger CI: format check, clippy, tests
- Releases are automatic on push to `main`
- Multi-platform builds: Linux, Windows, macOS (x86_64 and aarch64)

## Dependencies
- Minimize external dependencies
- Update dependencies carefully, testing thoroughly
- Prefer well-maintained crates from the Rust ecosystem

## Common Patterns
- Use `match` for state transitions in the app
- Leverage Rust's type system for correctness
- Prefer iterators over explicit loops
- Use `derive` macros for common traits (Debug, Clone, etc.)

## French Language Support
- The project documentation and prompts are in French
- Keep UI strings in French for consistency
- Comments can be in English or French
