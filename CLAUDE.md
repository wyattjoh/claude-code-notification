# CLAUDE.md

This file provides comprehensive guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a high-performance Rust CLI tool designed as a Claude Code hook for displaying cross-platform desktop notifications with advanced sound support. The tool integrates seamlessly with Claude Code's hooks system to provide immediate user feedback during development sessions.

**Supported Hook Events:**
- **Notification Hook** - Displays notifications for permission prompts, idle events, and other system notifications
- **Stop Hook** - Notifies when Claude Code finishes responding to a prompt with task completion status

## Build Commands

### Using Make (Recommended)

The project includes a comprehensive Makefile with optimized commands:

- `make build-release` - Compile optimized release binary with full optimizations
- `make install` - Build and install CLI globally as `claude-code-notification`
- `make test` - Run complete test suite with coverage
- `cargo test <test_name>` - Run a specific test (e.g., `cargo test test_validate_sound_path`)
- `make fmt` - Format Rust code using rustfmt with project settings
- `make clippy` - Lint Rust code with clippy for code quality
- `make clean` - Clean all build artifacts and target directory
- `make help` - Show all available make targets with descriptions

### Using Cargo Directly

For direct cargo usage when Make is unavailable:

- `cargo build` - Compile debug binary with symbols
- `cargo build --release` - Compile optimized release binary
- `cargo run` - Run CLI from source with development settings
- `cargo run -- setup` - Run interactive setup wizard to configure Claude Code settings
- `cargo install --path .` - Install CLI globally from current directory
- `cargo test` - Run test suite with default settings
- `cargo fmt` - Format Rust code with default configuration
- `cargo clippy` - Run clippy linter with default rules

## Usage as a Claude Code Hook

### Hook Configuration

This program integrates with Claude Code's hooks system. Configure in your Claude Code settings file:

**Basic Notification Hook:**
```json
{
  "hooks": {
    "Notification": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "claude-code-notification"
          }
        ]
      }
    ]
  }
}
```

**Stop Hook (Task Completion):**
```json
{
  "hooks": {
    "Stop": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "claude-code-notification --sound Submarine"
          }
        ]
      }
    ]
  }
}
```

**Both Hooks Configured:**
```json
{
  "hooks": {
    "Notification": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "claude-code-notification --sound Glass"
          }
        ]
      }
    ],
    "Stop": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "claude-code-notification --sound Submarine"
          }
        ]
      }
    ]
  }
}
```

**With Matcher (Specific Notification Types):**

To only trigger for specific notification types:
```json
{
  "hooks": {
    "Notification": [
      {
        "matcher": "permission_prompt",
        "hooks": [
          {
            "type": "command",
            "command": "claude-code-notification"
          }
        ]
      }
    ]
  }
}
```

### JSON Input Schema

The hook receives structured JSON input via stdin. The format varies by hook event type:

**Common Fields (all events):**
```json
{
  "session_id": "string - Claude session identifier",
  "transcript_path": "string - Path to session transcript file",
  "cwd": "string - Current working directory",
  "permission_mode": "string - Permission mode (default, plan, etc.)",
  "hook_event_name": "string - Event name: 'Notification' or 'Stop'"
}
```

**Notification Hook Input:**
```json
{
  "session_id": "abc123",
  "transcript_path": "/path/to/transcript.jsonl",
  "cwd": "/Users/dev/project",
  "permission_mode": "default",
  "hook_event_name": "Notification",
  "message": "Claude needs your permission to use Bash",
  "title": "Permission needed (optional)",
  "notification_type": "permission_prompt"
}
```

**Stop Hook Input:**
```json
{
  "session_id": "abc123",
  "transcript_path": "/path/to/transcript.jsonl",
  "cwd": "/Users/dev/project",
  "permission_mode": "default",
  "hook_event_name": "Stop",
  "stop_hook_active": false,
  "reason": "optional reason for stopping"
}
```

**Field Descriptions:**

| Field | Type | Description |
|-------|------|-------------|
| `session_id` | string | Claude session identifier |
| `transcript_path` | string | Path to session transcript file |
| `cwd` | string | Current working directory |
| `permission_mode` | string | Current permission mode |
| `hook_event_name` | string | Event type: "Notification" or "Stop" |
| `message` | string | Notification body text (Notification hook) |
| `title` | string? | Optional notification title |
| `notification_type` | string? | Notification type (Notification hook only) |
| `stop_hook_active` | boolean? | Whether stop hook is already active (Stop hook) |
| `reason` | string? | Stop reason if provided (Stop hook) |

## Interactive Setup

The CLI includes an interactive setup wizard that simplifies Claude Code integration:

```bash
claude-code-notification setup
# Or during development:
cargo run -- setup
```

**Setup Wizard Features:**
- Choose which hooks to configure (Notification, Stop, or both)
- Automatically detects available system sounds by scanning `/System/Library/Sounds`
- Interactive sound selection with live validation
- Custom sound file path input with existence checking
- Auto-creates `~/.claude/settings.json` with proper hook configuration
- Validates generated JSON against Claude Code's official schema
- Preserves existing settings when updating hooks

**Setup Options:**
1. **Notification only** - Configure notifications for permission prompts and system events
2. **Stop only** - Configure task completion notifications
3. **Both** - Full notification coverage (recommended)

**Under the Hood:**
The setup command (`src/setup.rs`) uses the `inquire` crate for interactive prompts and generates settings that match the Claude Code schema hosted at schemastore.org. It performs schema validation tests to ensure compatibility.

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

### Terminal Activation Parameter

**NEW:** `--activate-terminal` - Automatically activate the terminal application when notification is displayed

This feature is especially useful when running multiple Claude Code sessions in different terminals. When enabled, the notification will automatically bring the corresponding terminal to the foreground, saving you from manually searching for the correct window.

**How It Works:**
- The tool automatically detects your terminal application (Terminal.app, iTerm2, Warp, WezTerm, etc.)
- After displaying the notification, it uses AppleScript to activate the terminal
- A 300ms delay ensures the notification is visible before switching focus
- The terminal becomes the frontmost window, ready for your input

**Supported Terminals:**
- **Terminal.app** (com.apple.Terminal) - Default macOS terminal
- **iTerm2** (com.googlecode.iTerm2) - Popular enhanced terminal
- **Warp** (dev.warp.Warp-Stable) - Modern Rust-based terminal
- **WezTerm** (org.wezfurlong.wezterm) - Cross-platform GPU terminal
- **VSCode Integrated** (com.microsoft.VSCode) - Visual Studio Code terminal
- **JetBrains IDE** (com.jetbrains.intellij) - IDE integrated terminals

**Usage Examples:**
```bash
# Enable terminal activation
claude-code-notification --activate-terminal

# Combine with custom sound
claude-code-notification --sound Submarine --activate-terminal
```

**Hook Configuration with Terminal Activation:**
```json
{
  "hooks": {
    "Notification": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "claude-code-notification --activate-terminal"
          }
        ]
      }
    ]
  }
}
```

**Detection Method:**
The tool detects your terminal by checking the following environment variables:
- `TERM_PROGRAM` - Set by most macOS terminals (e.g., "iTerm.app", "Apple_Terminal")
- `TERM` - Contains terminal type (e.g., "wezterm" for WezTerm)
- `VSCODE_PID` - Indicates VSCode integrated terminal
- `IDE_PRODUCT`/`JETBRAINS_IDE` - Indicates JetBrains IDE terminal

**Note:** Terminal activation is currently macOS-only and requires AppleScript support. On other platforms, the feature is silently ignored.

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

**Notification Hook Testing:**
```bash
# Test permission prompt notification
echo '{"session_id":"test","transcript_path":"/tmp/test.md","cwd":"/tmp","permission_mode":"default","hook_event_name":"Notification","message":"Claude needs your permission to use Bash","title":"Permission needed","notification_type":"permission_prompt"}' | claude-code-notification

# Test with different notification types
echo '{"session_id":"test","transcript_path":"/tmp/test.md","cwd":"/tmp","permission_mode":"default","hook_event_name":"Notification","message":"Claude is waiting for input","title":"Idle","notification_type":"idle_prompt"}' | claude-code-notification

# Test system sound variants
echo '{"session_id":"test","transcript_path":"/tmp/test.md","cwd":"/tmp","permission_mode":"default","hook_event_name":"Notification","message":"Test","title":"Test"}' | claude-code-notification --sound Glass
echo '{"session_id":"test","transcript_path":"/tmp/test.md","cwd":"/tmp","permission_mode":"default","hook_event_name":"Notification","message":"Test","title":"Test"}' | claude-code-notification --sound Submarine
```

**Stop Hook Testing:**
```bash
# Test task completion notification (no custom message)
echo '{"session_id":"test","transcript_path":"/tmp/test.md","cwd":"/Users/test/project","permission_mode":"default","hook_event_name":"Stop","message":"","stop_hook_active":false}' | claude-code-notification

# Test with Submarine sound
echo '{"session_id":"test","transcript_path":"/tmp/test.md","cwd":"/Users/test/project","permission_mode":"default","hook_event_name":"Stop","message":"","stop_hook_active":false}' | claude-code-notification --sound Submarine
```

**Custom Audio File Testing:**
```bash
# Test custom sound file
echo '{"session_id":"test","transcript_path":"/tmp/test.md","cwd":"/tmp","permission_mode":"default","hook_event_name":"Notification","message":"Custom sound test"}' | claude-code-notification --sound ./custom-sound.wav
```

**Error Handling Testing:**
```bash
# Test invalid JSON handling
echo '{"invalid": json}' | claude-code-notification 2>&1 | head -5

# Test missing sound file handling
echo '{"session_id":"test","transcript_path":"/tmp/test.md","cwd":"/tmp","permission_mode":"default","hook_event_name":"Notification","message":"Test"}' | claude-code-notification --sound /nonexistent/file.wav
```

## Architecture and Implementation

### Project Structure

The codebase follows Rust best practices with clear separation of concerns:

- **`src/main.rs`** - CLI entry point with `clap` argument parsing and command routing
- **`src/lib.rs`** - Core notification logic, sound management, and parallel execution
  - `NotificationInput` - Struct for parsing hook JSON input (all fields optional with `#[serde(default)]`)
  - `Sound` - Enum for system sounds and custom audio files
  - `generate_notification_content()` - Creates notification content based on hook event type
  - `send_notification()` - Orchestrates notification display and sound playback in parallel
  - `send_notification_osascript()` - Displays macOS notifications using native osascript command
  - `play_sound()` - Plays audio files using afplay
- **`src/error.rs`** - Structured error types with `thiserror` for comprehensive error handling
- **`src/setup.rs`** - Interactive setup wizard that auto-configures Claude Code settings
- **`Cargo.toml`** - Dependency management with optimized release profile
- **`Makefile`** - Development workflow automation and build management

### Notification Content Generation

The `generate_notification_content()` function intelligently creates notification content based on the hook event type:

**Stop Hook Content:**
- Title: "Claude Code" (or custom title from input)
- Body: "✅ Task completed!" followed by the current working directory
- If a custom message is provided, it takes precedence
- If a `reason` field is provided, shows "Claude stopped: {reason}"

**Notification Hook Content:**
- Title: "Claude Code" (or custom title from input)
- Body: If `notification_type` is present, formats as "{type} - {message}"
- Otherwise displays the message directly

**Legacy Format Support:**
- For inputs without `hook_event_name`, treats as legacy format
- Displays title and message as-is for backward compatibility

### Key Dependencies and Their Roles

**Core Functionality:**
- **`clap`** - Command-line argument parsing with derive macros for maintainability
- **`notify-rust`** - Cross-platform desktop notifications (Windows/macOS/Linux) - used as fallback
- **`serde`/`serde_json`** - JSON serialization/deserialization with error handling
- **`inquire`** - Interactive command-line prompts for the setup wizard

**Error Management:**
- **`anyhow`** - Simplified error handling with context preservation
- **`thiserror`** - Structured error types with automatic trait implementations

**System Integration:**
- **`std::process`** - System command execution for `osascript` and `afplay` integration
- **`std::thread`** - Parallel execution of notifications and sound playback

### Notification Display Mechanism

**macOS (Primary Method - osascript):**
The tool uses macOS's native `osascript` command for reliable notification display:

```rust
fn send_notification_osascript(title: &str, body: &str) -> Result<()> {
    let clean_title = title.replace('\\', "\\\\").replace('"', r#"\""#);
    let clean_body = body.replace('\\', "\\\\").replace('"', r#"\"#).replace('\n', " ");

    let script = format!(
        r#"display notification "{}" with title "{}" sound name "Glass""#,
        clean_body, clean_title
    );

    Command::new("osascript").arg("-e").arg(&script).output()?;
    Ok(())
}
```

**Why osascript?**
- Ensures notifications appear in macOS Notification Center
- More reliable than `notify-rust` on macOS
- Native integration with macOS notification system
- Supports sound playback through the same command

**Cross-Platform Fallback (notify-rust):**
After calling osascript, the tool also attempts `notify-rust` for compatibility:

```rust
// Also try notify-rust for cross-platform compatibility
let _ = Notification::new()
    .summary(&title)
    .body(&body)
    .show();
```

This provides a fallback for non-macOS systems and ensures maximum compatibility.

**Parallel Execution with Sound:**
Notifications and sound playback execute simultaneously using threading:

```rust
let sound_handle = thread::spawn(move || {
    if let Err(e) = play_sound(&sound_clone) {
        eprintln!("Warning: Failed to play sound: {}", e);
    }
});

send_notification_osascript(&title, &body)?;

// Wait for sound thread to complete
sound_handle.join()
```

This ensures the notification displays immediately while the sound plays in parallel.

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

**Core Library Tests** (`src/lib.rs`):
- `test_parse_valid_input` - Tests basic JSON parsing
- `test_parse_stop_hook_input` - Tests Stop hook JSON parsing
- `test_parse_notification_hook_input` - Tests Notification hook JSON parsing
- `test_generate_notification_content_stop` - Tests Stop hook notification content generation
- `test_generate_notification_content_notification` - Tests Notification hook notification content generation
- `test_parse_missing_title` - Tests JSON without optional title field
- `test_parse_invalid_json` - Tests invalid JSON error handling
- `test_empty_input` - Tests empty input error handling
- `test_sound_from_name` - Tests sound name parsing
- `test_sound_as_str` - Tests sound string conversion
- `test_sound_default` - Tests default sound selection
- `test_notification_input_with_special_characters` - Tests special character handling
- `test_sound_path_resolution` - Tests system vs custom sound path resolution
- `test_sound_path_edge_cases` - Tests edge cases in sound paths

**Setup Module Tests** (`src/setup.rs`):
- `test_generated_settings_match_schema` - Validates generated JSON against Claude Code's official schema from schemastore.org
- `test_settings_creation_and_validation` - Tests settings file creation and structure verification
- `test_validate_sound_path_system_sound` - Validates system sound path checking
- `test_validate_sound_path_custom_file` - Validates custom file path validation
- `test_get_available_system_sounds` - Tests system sound detection

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
