# Claude Code Notifier

A macOS desktop notification hook for Claude Code that displays native
notifications when certain events occur during Claude Code sessions.

## Features

- Native macOS desktop notifications using `osascript`
- Customizable system sounds
- Simple JSON-based interface for Claude Code hooks
- Lightweight Deno-based implementation

## Installation

### Using JSR (Recommended)

```bash
deno run --allow-run jsr:@wyattjoh/claude-code-notification
```

### From Source

Clone this repository and run directly:

```bash
git clone https://github.com/wyattjoh/claude-code-notifier.git
cd claude-code-notifier
deno run --allow-run main.ts
```

## Usage

Configure this tool as a hook in your Claude Code settings:

```json
{
  "hooks": {
    "Notification": [
      {
        "type": "command",
        "command": "/opt/homebrew/bin/deno run --allow-run jsr:@wyattjoh/claude-code-notification"
      }
    ]
  }
}
```

> Note: You'll need to specify the absolute path to your locally installed Deno
> binary. Replace `/opt/homebrew/bin/deno` with the result of `which deno`.

### Custom Sound

Specify a custom system sound with the `--sound` parameter:

```json
{
  "hooks": {
    "Notification": [
      {
        "type": "command",
        "command": "/opt/homebrew/bin/deno run --allow-run jsr:@wyattjoh/claude-code-notification --sound Submarine"
      }
    ]
  }
}
```

> Note: You'll need to specify the absolute path to your locally installed Deno
> binary. Replace `/opt/homebrew/bin/deno` with the result of `which deno`.

Available system sounds can be found in `/System/Library/Sounds/`. Common
options include:

- Glass (default)
- Submarine
- Frog
- Purr
- Basso
- Blow
- Bottle
- Funk
- Hero
- Morse
- Ping
- Pop
- Sosumi
- Tink

## Development

### Prerequisites

- [Deno](https://deno.land/) runtime
- macOS (for notification support)

### Running Tests

```bash
deno test
```

### Manual Testing

Test the notifier with sample JSON input:

```bash
# Default sound
echo '{"session_id":"test","transcript_path":"/tmp/test.md","message":"Test message","title":"Test"}' | deno run --allow-run main.ts

# Custom sound
echo '{"session_id":"test","transcript_path":"/tmp/test.md","message":"Test message","title":"Test"}' | deno run --allow-run main.ts --sound Submarine
```

## How It Works

The notifier:

1. Receives JSON input via stdin from Claude Code
2. Parses the notification data (title, message, session info)
3. Uses macOS `osascript` to display a native notification
4. Plays the specified system sound
