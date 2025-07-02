# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with
code in this repository.

## Project Overview

This is a Claude Code hook designed to display macOS desktop notifications. It
can be used with Claude Code's hooks feature to notify users when certain events
occur during Claude Code sessions.

## Code Formatting

Code should be formatted using `deno fmt` after writing.

## Usage as a Claude Code Hook

This program is designed to be used with the "Notification" hook event in Claude
Code. When configured, it will display macOS notifications when Claude Code
sends notification events.

Example configuration in Claude Code settings:

```json
{
  "hooks": {
    "Notification": [
      {
        "type": "command",
        "command": "deno run --allow-run jsr:@wyattjoh/claude-code-notification"
      }
    ]
  }
}
```

With a custom sound:

```json
{
  "hooks": {
    "Notification": [
      {
        "type": "command",
        "command": "deno run --allow-run jsr:@wyattjoh/claude-code-notification --sound Submarine"
      }
    ]
  }
}
```

The hook receives JSON input via stdin with:

- `session_id`: The Claude session identifier
- `transcript_path`: Path to the session transcript
- `message`: Notification message to display
- `title`: Notification title

## CLI Parameters

- `--sound <name>`: Specify the system sound to play (default: "Glass")
  - Available sounds can be found in `/System/Library/Sounds/`
  - Common sounds: Glass, Submarine, Frog, Purr, Basso, Blow, Bottle, Funk, Hero, Morse, Ping, Pop, Sosumi, Tink

## Development Commands

### Running tests

```bash
deno test
```

### Running in development mode

```bash
deno task dev
```

### Testing the notifier manually

```bash
# Default Glass sound
echo '{"session_id":"test","transcript_path":"/tmp/test.md","message":"Test message","title":"Test"}' | deno run main.ts

# Custom sound
echo '{"session_id":"test","transcript_path":"/tmp/test.md","message":"Test message","title":"Test"}' | deno run main.ts --sound Submarine
```

## Implementation Details

- Uses macOS `osascript` command to display native notifications
- Plays the "Glass" sound with notifications
- Handles special characters in notification text
- Tests mock the `Deno.Command` API to avoid triggering actual notifications
  during test runs
