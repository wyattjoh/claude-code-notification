# CLAUDE.md

This file provides comprehensive guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a high-performance Rust CLI tool designed as a Claude Code hook for displaying cross-platform desktop notifications with advanced sound support. The tool integrates seamlessly with Claude Code's hooks system to provide immediate user feedback during development sessions.

## Build Commands

### Using Make (Recommended)

The project includes a comprehensive Makefile with optimized commands:

- `make build-release` - Compile optimized release binary with full optimizations
- `make install` - Build and install CLI globally as `claude-code-notification`
- `make test` - Run complete test suite with coverage
- `make fmt` - Format Rust code using rustfmt with project settings
- `make clippy` - Lint Rust code with clippy for code quality
- `make clean` - Clean all build artifacts and target directory
- `make help` - Show all available make targets with descriptions

### Using Cargo Directly

For direct cargo usage when Make is unavailable:

- `cargo build` - Compile debug binary with symbols
- `cargo build --release` - Compile optimized release binary
- `cargo run` - Run CLI from source with development settings
- `cargo install --path .` - Install CLI globally from current directory
- `cargo test` - Run test suite with default settings
- `cargo fmt` - Format Rust code with default configuration
- `cargo clippy` - Run clippy linter with default rules

## Usage as a Claude Code Hook

### Hook Configuration

This program integrates with Claude Code's notification system through the hooks configuration. Configure in your Claude Code settings file:

**Basic Configuration:**
```json
{
  "hooks": {
    "Notification": [
      {
        "type": "command",
        "command": "claude-code-notification"
      }
    ]
  }
}
```

**Advanced Configuration with Custom Sound:**
```json
{
  "hooks": {
    "Notification": [
      {
        "type": "command", 
        "command": "claude-code-notification --sound Submarine"
      }
    ]
  }
}
```

### JSON Input Schema

The hook receives structured JSON input via stdin:

```json
{
  "session_id": "string - Claude session identifier", 
  "transcript_path": "string - Path to session transcript file",
  "message": "string - Notification body text",
  "title": "string? - Optional notification title (defaults to 'Claude Code')"
}
```

## CLI Parameters and Sound System

### Sound Parameter Options

The `--sound` parameter supports intelligent path resolution:

**System Sounds** (recommended for consistency):
- Format: `--sound {SoundName}` (no path separators)
- Resolves to: `/System/Library/Sounds/{SoundName}.aiff`
- Available: Glass (default), Submarine, Frog, Purr, Basso, Blow, Bottle, Funk, Hero, Morse, Ping, Pop, Sosumi, Tink

**Custom Audio Files** (for specialized notifications):
- Format: `--sound {/path/to/file}` (contains path separators)
- Supports: `.wav`, `.aiff`, `.mp3`, `.m4a`, and other `afplay`-compatible formats
- Examples:
  - `--sound ./assets/notification.wav`
  - `--sound /Users/dev/sounds/alert.m4a`
  - `--sound ~/Music/custom-alert.aiff`

## Development Workflow

### Local Development

**Quick Development Cycle:**
```bash
# Run with immediate feedback
cargo run

# Run with custom sound for testing
echo '{"session_id":"dev","transcript_path":"/tmp/dev.md","message":"Development test","title":"Dev Test"}' | cargo run -- --sound Submarine
```

**Testing and Quality Assurance:**
```bash
# Run comprehensive tests
make test

# Check code formatting
make fmt

# Run linter for code quality
make clippy

# Full quality check pipeline
make test && make fmt && make clippy
```

### Manual Testing Scenarios

**Basic Functionality Testing:**
```bash
# Test default configuration
echo '{"session_id":"test","transcript_path":"/tmp/test.md","message":"Default notification test","title":"Basic Test"}' | cargo run

# Test system sound variants
echo '{"session_id":"test","transcript_path":"/tmp/test.md","message":"System sound test","title":"Sound Test"}' | cargo run -- --sound Glass
echo '{"session_id":"test","transcript_path":"/tmp/test.md","message":"Submarine sound test","title":"Sound Test"}' | cargo run -- --sound Submarine

# Test custom audio files
echo '{"session_id":"test","transcript_path":"/tmp/test.md","message":"Custom sound test","title":"Custom Test"}' | cargo run -- --sound ./366102__original_sound__confirmation-upward.wav
```

**Error Handling Testing:**
```bash
# Test invalid JSON handling
echo '{"invalid": json}' | cargo run 2>&1 | head -5

# Test missing sound file handling  
echo '{"session_id":"test","transcript_path":"/tmp/test.md","message":"Missing sound test","title":"Error Test"}' | cargo run -- --sound /nonexistent/file.wav
```

## Architecture and Implementation

### Project Structure

The codebase follows Rust best practices with clear separation of concerns:

- **`src/main.rs`** - CLI entry point with `clap` argument parsing and error handling
- **`src/lib.rs`** - Core notification logic, sound management, and parallel execution
- **`src/error.rs`** - Structured error types with `thiserror` for comprehensive error handling
- **`Cargo.toml`** - Dependency management with optimized release profile
- **`Makefile`** - Development workflow automation and build management

### Key Dependencies and Their Roles

**Core Functionality:**
- **`clap`** - Command-line argument parsing with derive macros for maintainability
- **`notify-rust`** - Cross-platform desktop notifications (Windows/macOS/Linux)
- **`serde`/`serde_json`** - JSON serialization/deserialization with error handling

**Error Management:**
- **`anyhow`** - Simplified error handling with context preservation
- **`thiserror`** - Structured error types with automatic trait implementations

**System Integration:**
- **`std::process`** - System command execution for `afplay` integration
- **`std::thread`** - Parallel execution of notifications and sound playback

### Performance Optimizations

**Release Build Configuration:**
```toml
[profile.release]
opt-level = 3         # Maximum optimization
lto = true           # Link-time optimization for smaller binaries
codegen-units = 1    # Single compilation unit for better optimization
panic = "abort"      # Smaller binaries by avoiding unwind handling
strip = true         # Remove debug symbols from release builds
```

**Parallel Execution Design:**
- Notifications and sound playback execute simultaneously using threading
- Non-blocking error handling ensures notification display even if sound fails
- Graceful degradation on systems without `afplay` support

### Testing Strategy

**Unit Test Coverage:**
- JSON parsing and validation with various input scenarios
- Sound path resolution logic for both system and custom sounds  
- Error handling for invalid inputs and missing files
- Cross-platform compatibility testing

**Integration Testing:**
- End-to-end notification display testing (when possible)
- Sound playback verification with different audio formats
- Claude Code hook integration validation

## Troubleshooting and Common Issues

### Development Issues

**Build Problems:**
- Ensure Rust toolchain is up-to-date: `rustup update`
- Clear build cache: `make clean` then rebuild
- Check dependency compatibility: `cargo tree` for conflicts

**Sound Issues:**
- Verify `afplay` availability: `which afplay`
- Test sound file directly: `afplay /System/Library/Sounds/Glass.aiff`
- Check file permissions for custom audio files

**Hook Integration Issues:**
- Validate JSON input format with online validators
- Test CLI independently before Claude Code integration
- Check Claude Code hook configuration syntax

### Performance Considerations

- Release builds are significantly faster than debug builds
- Custom sound files should be reasonably sized (< 1MB recommended)
- Parallel execution ensures UI responsiveness during sound playback
- Error logging helps diagnose issues without blocking functionality

This comprehensive development guide ensures efficient contribution and maintenance of the Claude Code notification system.
