# Claude Code Notifier

A high-performance Rust CLI tool for displaying cross-platform desktop notifications as a Claude Code hook, with advanced sound support and parallel execution.

## Installation

```shell
brew install wyattjoh/stable/claude-code-notification
```

## Features

- **Cross-Platform Notifications** - Native desktop notifications on Windows, macOS, and Linux
- **Stop Hook Support** - Get notified when Claude Code finishes a task
- **Notification Hook Support** - Get notified for permission requests and other events
- **Terminal Auto-Activation** - Automatically bring the correct terminal to the front (NEW!)
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

### Settings File Location

Hooks can be configured at different scopes:

| Location | Scope | Shareable |
| --- | --- | --- |
| `~/.claude/settings.json` | All your projects | No, local to your machine |
| `.claude/settings.json` | Single project | Yes, can be committed to the repo |
| `.claude/settings.local.json` | Single project | No, gitignored |
| Managed policy settings | Organization-wide | Yes, admin-controlled |
| Plugin `hooks/hooks.json` | When plugin is enabled | Yes, bundled with the plugin |
| Skill or agent frontmatter | While component is active | Yes, defined in the component |

### Configuration Examples

**Basic Integration:**

Configure in your Claude Code settings file:

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

**With Custom Sound:**

```json
{
  "hooks": {
    "Notification": [
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

**With Matcher (Specific Notification Types Only):**

To only trigger notifications for specific types:

```json
{
  "hooks": {
    "Notification": [
      {
        "matcher": "permission_prompt",
        "hooks": [
          {
            "type": "command",
            "command": "claude-code-notification --sound Glass"
          }
        ]
      }
    ]
  }
}
```

**Stop Hook (Task Completion Notification):**

Get notified when Claude Code finishes responding:

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

**Both Notification and Stop Hooks:**

Configure both hooks to get notified for all events:

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
  "cwd": "string",
  "permission_mode": "string",
  "hook_event_name": "Notification",
  "message": "string",
  "title": "string (optional)",
  "notification_type": "string (optional)"
}
```

**Fields:**
- `session_id` - Claude session identifier
- `transcript_path` - Path to session transcript file
- `cwd` - Current working directory of the project
- `permission_mode` - Current permission mode (e.g., "default")
- `hook_event_name` - Name of the hook event (always "Notification" for this hook)
- `message` - Notification body text
- `title` - Notification title (defaults to "Claude Code")
- `notification_type` - Type of notification event (see below)

**Notification Types:**

The `notification_type` field can have the following values, which can be used in hook matchers to filter when the hook should execute:

- `permission_prompt` - Request for user permission to use a tool
- `idle_prompt` - Prompt shown when Claude is idle
- `auth_success` - Successful authentication event
- `elicitation_dialog` - User elicitation/dialog event

**Matcher Reference:**

For more advanced filtering, you can use regex patterns to match specific notification types:

| Matcher | Description |
| --- | --- |
| `permission_prompt` | Request for user permission to use a tool |
| `idle_prompt` | Prompt shown when Claude is idle |
| `auth_success` | Successful authentication event |
| `elicitation_dialog` | User elicitation/dialog event |

The `matcher` field supports regex patterns, so you can use `permission_prompt|idle_prompt` to match multiple notification types at once.

To omit the matcher and run for all notification types, simply leave it out of the configuration.

## Manual Testing

Test the notifier with sample JSON input:

```bash
# Test Notification hook
echo '{"session_id":"test","transcript_path":"/tmp/test.md","cwd":"/tmp","permission_mode":"default","hook_event_name":"Notification","message":"Claude needs your permission","title":"Permission needed","notification_type":"permission_prompt"}' | claude-code-notification

# Test Stop hook (task completion)
echo '{"session_id":"test","transcript_path":"/tmp/test.md","cwd":"/Users/test/project","permission_mode":"default","hook_event_name":"Stop","message":"","stop_hook_active":false}' | claude-code-notification

# With custom sound
echo '{"session_id":"test","transcript_path":"/tmp/test.md","cwd":"/Users/test/project","permission_mode":"default","hook_event_name":"Stop","message":"","stop_hook_active":false}' | claude-code-notification --sound Submarine

# Test with custom sound file
echo '{"session_id":"test","transcript_path":"/tmp/test.md","cwd":"/tmp","permission_mode":"default","hook_event_name":"Notification","message":"Test notification"}' | claude-code-notification --sound ./custom-sound.wav

# Test terminal auto-activation (macOS)
echo '{"session_id":"test","transcript_path":"/tmp/test.md","cwd":"/tmp","permission_mode":"default","hook_event_name":"Notification","message":"Terminal activation test"}' | claude-code-notification --activate-terminal

# Test with both sound and terminal activation
echo '{"session_id":"test","transcript_path":"/tmp/test.md","cwd":"/tmp","permission_mode":"default","hook_event_name":"Stop","message":""}' | claude-code-notification --sound Submarine --activate-terminal
```

**NEW: Terminal Auto-Activation**

The `--activate-terminal` flag automatically brings your terminal to the front when a notification is displayed. This is especially useful when running multiple Claude Code sessions:

```bash
# Enable terminal activation in hooks
claude-code-notification --activate-terminal
```

**How It Works:**
1. The tool detects your terminal application (Terminal.app, iTerm2, Warp, WezTerm, etc.)
2. After displaying the notification, it activates the terminal using AppleScript
3. The terminal becomes the frontmost window, ready for your input

**Supported Terminals:** Terminal.app, iTerm2, Warp, WezTerm, VSCode Integrated, JetBrains IDE terminals

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

The CLI uses `osascript` for reliable macOS notifications, `notify-rust` for cross-platform compatibility, and `afplay` for macOS sound playback, with comprehensive error handling and parallel execution for optimal user experience.

## Architecture

The notification system consists of:

- **CLI Entry Point** (`src/main.rs`) - Argument parsing with `clap`
- **Core Library** (`src/lib.rs`) - Notification logic and sound playback
- **Error Handling** (`src/error.rs`) - Structured error types with `thiserror`
- **macOS Notifications** - `osascript` for reliable notification display (primary), `notify-rust` for cross-platform compatibility (fallback)
- **Sound Playback** - `afplay` for macOS audio playback
- **Parallel Execution** - Threading for simultaneous notification display and sound playback

### Notification Display (macOS)

On macOS, the tool uses **osascript** as the primary notification method for maximum reliability:

```bash
osascript -e 'display notification "message" with title "title" sound name "Glass"'
```

This ensures notifications are consistently displayed in the macOS Notification Center. The `notify-rust` library is also called as a fallback for cross-platform compatibility, but osascript provides the most reliable experience on macOS.

## License

MIT License - see [LICENSE](LICENSE) file for details.
