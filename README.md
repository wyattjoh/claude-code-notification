# Claude Code Notifier

A high-performance Rust CLI tool for displaying cross-platform desktop notifications as a Claude Code hook, with advanced sound support and parallel execution.

## Installation

```shell
cargo install --git https://github.com/wyattjoh/claude-code-notifier
```

## Features

- **Cross-Platform Notifications** - Native desktop notifications on Windows, macOS, and Linux
- **Advanced Sound Support** - System sounds and custom audio files with intelligent path resolution
- **Parallel Execution** - Notification display and sound playback execute simultaneously
- **Claude Code Integration** - Seamless hook integration with JSON-based interface
- **Error Resilience** - Graceful handling of sound failures without blocking notifications
- **High Performance** - Compiled Rust binary with optimized release builds

## Requirements

- [Rust](https://rustup.rs/) (for building from source)
- macOS with `afplay` (for sound support)
- Claude Code (for hook integration)

## Usage

The tool integrates with Claude Code as a notification hook, receiving JSON input via stdin and displaying native notifications with optional sound playback.

**Basic Integration:**

Configure in your Claude Code settings:

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

**With Custom Sound:**

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

## Configuration

### Sound Options

The `--sound` parameter supports two modes:

**System Sounds** (no `/` in name):
- Resolves to `/System/Library/Sounds/{name}.aiff`
- Available: Glass (default), Submarine, Frog, Purr, Basso, Blow, Bottle, Funk, Hero, Morse, Ping, Pop, Sosumi, Tink

**Custom Paths** (contains `/`):
- Used directly as file path to `afplay`
- Supports: `.wav`, `.aiff`, `.mp3`, `.m4a`, and other formats supported by `afplay`
- Examples:
  - `--sound /path/to/custom/sound.wav`
  - `--sound ./sounds/notification.mp3`
  - `--sound ~/Music/alert.m4a`

### JSON Input Schema

The tool expects JSON input via stdin with the following structure:

```json
{
  "session_id": "string",
  "transcript_path": "string", 
  "message": "string",
  "title": "string (optional)"
}
```

**Fields:**
- `session_id` - Claude session identifier
- `transcript_path` - Path to session transcript file
- `message` - Notification body text
- `title` - Notification title (defaults to "Claude Code")

## Manual Testing

Test the notifier with sample JSON input:

```bash
# Default Glass sound
echo '{"session_id":"test","transcript_path":"/tmp/test.md","message":"Test message","title":"Test"}' | claude-code-notification

# System sound
echo '{"session_id":"test","transcript_path":"/tmp/test.md","message":"Test message","title":"Test"}' | claude-code-notification --sound Submarine

# Custom sound file
echo '{"session_id":"test","transcript_path":"/tmp/test.md","message":"Test message","title":"Test"}' | claude-code-notification --sound ./custom-sound.wav
```

## Development

To contribute or modify the CLI:

```bash
# Clone the repository
git clone https://github.com/wyattjoh/claude-code-notifier.git
cd claude-code-notifier

# Run in development
cargo run

# Build release binary
make build-release

# Run tests
make test

# Format code
make fmt

# Lint code  
make clippy

# Install globally
make install
```

The CLI uses `notify-rust` for cross-platform notifications and `afplay` for macOS sound playback, with comprehensive error handling and parallel execution for optimal user experience.

## Architecture

The notification system consists of:

- **CLI Entry Point** (`src/main.rs`) - Argument parsing with `clap`
- **Core Library** (`src/lib.rs`) - Notification logic and sound playback
- **Error Handling** (`src/error.rs`) - Structured error types with `thiserror`
- **Cross-Platform Support** - `notify-rust` for notifications, `afplay` for sounds
- **Parallel Execution** - Threading for simultaneous notification display and sound playback

## License

MIT License - see [LICENSE](LICENSE) file for details.
