pub mod error;

use anyhow::Result;
use notify_rust::Notification;
use serde::{Deserialize, Serialize};
use std::env;
use std::io::Read;
use std::process::Command;
use std::thread;

pub use error::{NotificationError, NotificationResult};

/// Terminal application types supported for auto-activation
#[derive(Debug, Clone, PartialEq)]
pub enum TerminalApp {
    Terminal,
    ITerm,
    Warp,
    WezTerm,
    VSCodeIntegrated,
    JetBrainsIDE,
    Unknown(String),
}

impl TerminalApp {
    /// Get the bundle identifier for macOS app activation
    pub fn bundle_id(&self) -> &str {
        match self {
            TerminalApp::Terminal => "com.apple.Terminal",
            TerminalApp::ITerm => "com.googlecode.iTerm2",
            TerminalApp::Warp => "dev.warp.Warp-Stable",
            TerminalApp::WezTerm => "org.wezfurlong.wezterm",
            TerminalApp::VSCodeIntegrated => "com.microsoft.VSCode",
            TerminalApp::JetBrainsIDE => "com.jetbrains.intellij", // Generic JetBrains
            TerminalApp::Unknown(_) => "",
        }
    }

    /// Detect the terminal application from environment variables
    pub fn detect() -> Self {
        // Check TERM_PROGRAM (set by most macOS terminals)
        if let Ok(term_program) = env::var("TERM_PROGRAM") {
            return match term_program.as_str() {
                "Apple_Terminal" => TerminalApp::Terminal,
                "iTerm.app" => TerminalApp::ITerm,
                "Warp.app" => TerminalApp::Warp,
                "vscode" => TerminalApp::VSCodeIntegrated,
                _ => TerminalApp::Unknown(term_program),
            };
        }

        // Check for WezTerm
        if let Ok(term) = env::var("TERM") {
            if term.contains("wezterm") {
                return TerminalApp::WezTerm;
            }
        }

        // Check for JetBrains IDE terminal
        if env::var("IDE_PRODUCT").is_ok() || env::var("JETBRAINS_IDE").is_ok() {
            return TerminalApp::JetBrainsIDE;
        }

        // Check VSCode integrated terminal
        if env::var("VSCODE_PID").is_ok() || env::var("TERM_PROGRAM").is_ok() && env::var("TERM_PROGRAM").unwrap() == "vscode" {
            return TerminalApp::VSCodeIntegrated;
        }

        // Default to Terminal.app on macOS
        #[cfg(target_os = "macos")]
        return TerminalApp::Terminal;

        #[cfg(not(target_os = "macos"))]
        return TerminalApp::Unknown("unknown".to_string());
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NotificationInput {
    #[serde(default)]
    pub session_id: String,
    #[serde(default)]
    pub transcript_path: String,
    #[serde(default)]
    pub cwd: String,
    #[serde(default)]
    pub permission_mode: String,
    #[serde(default)]
    pub hook_event_name: String,
    #[serde(default)]
    pub message: String,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub notification_type: Option<String>,
    #[serde(default)]
    pub stop_hook_active: Option<bool>,
    #[serde(default)]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub enum Sound {
    #[default]
    Glass,
    Submarine,
    Frog,
    Purr,
    Basso,
    Blow,
    Bottle,
    Funk,
    Hero,
    Morse,
    Ping,
    Pop,
    Sosumi,
    Tink,
    Custom(String),
}

impl Sound {
    pub fn from_name(name: &str) -> Self {
        match name {
            "Glass" => Sound::Glass,
            "Submarine" => Sound::Submarine,
            "Frog" => Sound::Frog,
            "Purr" => Sound::Purr,
            "Basso" => Sound::Basso,
            "Blow" => Sound::Blow,
            "Bottle" => Sound::Bottle,
            "Funk" => Sound::Funk,
            "Hero" => Sound::Hero,
            "Morse" => Sound::Morse,
            "Ping" => Sound::Ping,
            "Pop" => Sound::Pop,
            "Sosumi" => Sound::Sosumi,
            "Tink" => Sound::Tink,
            custom => Sound::Custom(custom.to_string()),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Sound::Glass => "Glass",
            Sound::Submarine => "Submarine",
            Sound::Frog => "Frog",
            Sound::Purr => "Purr",
            Sound::Basso => "Basso",
            Sound::Blow => "Blow",
            Sound::Bottle => "Bottle",
            Sound::Funk => "Funk",
            Sound::Hero => "Hero",
            Sound::Morse => "Morse",
            Sound::Ping => "Ping",
            Sound::Pop => "Pop",
            Sound::Sosumi => "Sosumi",
            Sound::Tink => "Tink",
            Sound::Custom(name) => name,
        }
    }

    pub fn get_afplay_path(&self) -> String {
        let sound_name = self.as_str();

        // If the sound name contains a slash, treat it as a custom path
        if sound_name.contains('/') {
            sound_name.to_string()
        } else {
            // System sound - add path prefix and .aiff extension
            format!("/System/Library/Sounds/{}.aiff", sound_name)
        }
    }
}

pub fn main<R: Read>(mut stdin: R, sound: Sound, activate_terminal: bool) -> Result<()> {
    // Read all input from stdin
    let mut buffer = String::new();
    stdin.read_to_string(&mut buffer)?;

    // Parse the JSON input
    let input: NotificationInput = serde_json::from_str(&buffer)?;

    // Detect the terminal application if activation is requested
    let terminal = if activate_terminal {
        Some(TerminalApp::detect())
    } else {
        None
    };

    // Create and send the notification
    send_notification(&input, &sound, terminal)?;

    Ok(())
}

/// Generate notification title and body based on hook event type
fn generate_notification_content(input: &NotificationInput) -> (String, String) {
    match input.hook_event_name.as_str() {
        "Stop" => {
            // Claude Code has finished responding
            let title = input.title.as_deref()
                .unwrap_or("Claude Code");

            let body = if input.message.is_empty() {
                // If no custom message provided, create a more informative one
                let cwd_display = if !input.cwd.is_empty() {
                    format!("\n📁 {}", input.cwd)
                } else {
                    String::new()
                };
                format!("✅ Task completed!{}", cwd_display)
            } else if let Some(reason) = &input.reason {
                format!("Claude stopped: {}", reason)
            } else {
                input.message.clone()
            };

            (title.to_string(), body)
        }
        "Notification" => {
            // Regular notification event
            let title = input.title.as_deref()
                .unwrap_or("Claude Code");

            let body = if let Some(notification_type) = &input.notification_type {
                format!("{} - {}", notification_type, input.message)
            } else {
                input.message.clone()
            };

            (title.to_string(), body)
        }
        "" | "unknown" => {
            // Legacy format or unknown event type
            let title = input.title.as_deref()
                .unwrap_or("Claude Code");
            (title.to_string(), input.message.clone())
        }
        _ => {
            // Other event types
            let title = input.title.as_deref()
                .unwrap_or("Claude Code");
            let body = format!("[{}] {}",
                input.hook_event_name, input.message);
            (title.to_string(), body)
        }
    }
}

fn send_notification(
    input: &NotificationInput,
    sound: &Sound,
    terminal: Option<TerminalApp>,
) -> Result<()> {
    // Generate notification content based on event type
    let (title, body) = generate_notification_content(input);

    // Clone the sound for the thread
    let sound_clone = sound.clone();

    // Clone data for osascript
    let title_clone = title.clone();
    let body_clone = body.clone();

    // Spawn a thread to play the sound in parallel
    let sound_handle = thread::spawn(move || {
        if let Err(e) = play_sound(&sound_clone) {
            eprintln!("Warning: Failed to play sound: {}", e);
        }
    });

    // Send notification using osascript (more reliable on macOS)
    send_notification_osascript(&title_clone, &body_clone)?;

    // Also try notify-rust for cross-platform compatibility
    let _ = Notification::new()
        .summary(&title)
        .body(&body)
        .show();

    // Wait for the sound thread to complete
    if let Err(e) = sound_handle.join() {
        eprintln!("Warning: Sound thread panicked: {:?}", e);
    }

    // Activate the terminal application if requested
    if let Some(term) = terminal {
        // Add a small delay to ensure notification is visible first
        std::thread::sleep(std::time::Duration::from_millis(300));
        activate_terminal(&term)?;
    }

    Ok(())
}

/// Send notification using macOS osascript (more reliable than notify-rust)
fn send_notification_osascript(title: &str, body: &str) -> Result<()> {
    // Sanitize the body and title for AppleScript
    // Replace backslashes and quotes with escaped versions
    let clean_title = title
        .replace('\\', "\\\\")
        .replace('"', r#"\""#);

    let clean_body = body
        .replace('\\', "\\\\")
        .replace('"', r#"\"#)
        .replace('\n', " ");  // Replace newlines with spaces for single-line display

    let script = format!(
        r#"display notification "{}" with title "{}" sound name "Glass""#,
        clean_body,
        clean_title
    );

    let output = Command::new("osascript")
        .arg("-e")
        .arg(&script)
        .output();

    match output {
        Ok(result) => {
            if !result.status.success() {
                let stderr = String::from_utf8_lossy(&result.stderr);
                eprintln!(
                    "Warning: osascript notification failed: {}",
                    stderr.trim()
                );
            }
        }
        Err(e) => {
            eprintln!(
                "Warning: Failed to execute osascript for notification: {}",
                e
            );
        }
    }

    Ok(())
}

fn play_sound(sound: &Sound) -> Result<()> {
    let sound_path = sound.get_afplay_path();

    // Execute afplay command to play the sound
    let output = Command::new("afplay").arg(&sound_path).output();

    match output {
        Ok(result) => {
            if !result.status.success() {
                // Log a warning but don't fail the whole notification
                eprintln!(
                    "Warning: Failed to play sound '{}'. afplay exit code: {:?}",
                    sound_path,
                    result.status.code()
                );
            }
        }
        Err(e) => {
            // Log a warning but don't fail the whole notification
            eprintln!(
                "Warning: Failed to execute afplay for sound '{}': {}",
                sound_path, e
            );
        }
    }

    Ok(())
}

/// Activate the terminal application using AppleScript
fn activate_terminal(terminal: &TerminalApp) -> Result<()> {
    let bundle_id = terminal.bundle_id();

    if bundle_id.is_empty() {
        eprintln!(
            "Warning: Cannot activate unknown terminal application: {:?}",
            terminal
        );
        return Ok(());
    }

    let script = format!(
        r#"tell application "{}" to activate"#,
        bundle_id
    );

    let output = Command::new("osascript")
        .arg("-e")
        .arg(&script)
        .output();

    match output {
        Ok(result) => {
            if !result.status.success() {
                let stderr = String::from_utf8_lossy(&result.stderr);
                eprintln!(
                    "Warning: Failed to activate terminal '{}': {}",
                    bundle_id,
                    stderr.trim()
                );
            } else {
                eprintln!("✅ Activated terminal: {}", bundle_id);
            }
        }
        Err(e) => {
            eprintln!(
                "Warning: Failed to execute osascript to activate terminal: {}",
                e
            );
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_parse_valid_input() {
        let input_data = r#"{
            "session_id": "test-session-123",
            "transcript_path": "/path/to/transcript.md",
            "message": "Test notification message",
            "title": "Test Title"
        }"#;

        // Test that we can parse the JSON correctly
        let input: Result<NotificationInput, _> = serde_json::from_str(input_data);
        assert!(input.is_ok());

        let input = input.unwrap();
        assert_eq!(input.session_id, "test-session-123");
        assert_eq!(input.message, "Test notification message");
        assert_eq!(input.title, Some("Test Title".to_string()));
    }

    #[test]
    fn test_parse_stop_hook_input() {
        let input_data = r#"{
            "session_id": "test-session-456",
            "transcript_path": "/path/to/transcript.md",
            "cwd": "/Users/test/project",
            "permission_mode": "default",
            "hook_event_name": "Stop",
            "stop_hook_active": false
        }"#;

        // Test that we can parse Stop hook input
        let input: Result<NotificationInput, _> = serde_json::from_str(input_data);
        assert!(input.is_ok());

        let input = input.unwrap();
        assert_eq!(input.session_id, "test-session-456");
        assert_eq!(input.hook_event_name, "Stop");
        assert_eq!(input.cwd, "/Users/test/project");
        assert_eq!(input.stop_hook_active, Some(false));
    }

    #[test]
    fn test_parse_notification_hook_input() {
        let input_data = r#"{
            "session_id": "test-session-789",
            "transcript_path": "/path/to/transcript.md",
            "cwd": "/Users/test/project",
            "permission_mode": "default",
            "hook_event_name": "Notification",
            "message": "Permission needed",
            "title": "Alert",
            "notification_type": "permission_prompt"
        }"#;

        // Test that we can parse Notification hook input
        let input: Result<NotificationInput, _> = serde_json::from_str(input_data);
        assert!(input.is_ok());

        let input = input.unwrap();
        assert_eq!(input.hook_event_name, "Notification");
        assert_eq!(input.message, "Permission needed");
        assert_eq!(input.notification_type, Some("permission_prompt".to_string()));
    }

    #[test]
    fn test_generate_notification_content_stop() {
        let input = NotificationInput {
            session_id: "test".to_string(),
            transcript_path: "/path/to/transcript.md".to_string(),
            cwd: "/Users/test/project".to_string(),
            permission_mode: "default".to_string(),
            hook_event_name: "Stop".to_string(),
            message: String::new(),
            title: None,
            notification_type: None,
            stop_hook_active: Some(false),
            reason: None,
        };

        let (title, body) = generate_notification_content(&input);
        assert_eq!(title, "Claude Code");
        assert!(body.contains("✅ Task completed!"));
        assert!(body.contains("/Users/test/project"));
    }

    #[test]
    fn test_generate_notification_content_notification() {
        let input = NotificationInput {
            session_id: "test".to_string(),
            transcript_path: "/path/to/transcript.md".to_string(),
            cwd: String::new(),
            permission_mode: "default".to_string(),
            hook_event_name: "Notification".to_string(),
            message: "Test message".to_string(),
            title: Some("Custom Title".to_string()),
            notification_type: Some("permission_prompt".to_string()),
            stop_hook_active: None,
            reason: None,
        };

        let (title, body) = generate_notification_content(&input);
        assert_eq!(title, "Custom Title");
        assert_eq!(body, "permission_prompt - Test message");
    }

    #[test]
    fn test_parse_missing_title() {
        let input_data = r#"{
            "session_id": "test-session-456",
            "transcript_path": "/path/to/transcript.md",
            "message": "Message without title"
        }"#;

        // Test that we can parse the JSON correctly
        let input: Result<NotificationInput, _> = serde_json::from_str(input_data);
        assert!(input.is_ok());

        let input = input.unwrap();
        assert_eq!(input.session_id, "test-session-456");
        assert_eq!(input.message, "Message without title");
        assert_eq!(input.title, None);
    }

    #[test]
    fn test_parse_invalid_json() {
        let invalid_json = "{ invalid json }";
        let cursor = Cursor::new(invalid_json);
        let result = main(cursor, Sound::Glass, false);

        assert!(result.is_err());
    }

    #[test]
    fn test_empty_input() {
        let empty_input = "";
        let cursor = Cursor::new(empty_input);
        let result = main(cursor, Sound::Glass, false);

        assert!(result.is_err());
    }

    #[test]
    fn test_sound_from_name() {
        assert!(matches!(Sound::from_name("Glass"), Sound::Glass));
        assert!(matches!(Sound::from_name("Submarine"), Sound::Submarine));
        assert!(matches!(Sound::from_name("CustomSound"), Sound::Custom(_)));
    }

    #[test]
    fn test_sound_as_str() {
        assert_eq!(Sound::Glass.as_str(), "Glass");
        assert_eq!(Sound::Submarine.as_str(), "Submarine");
        assert_eq!(Sound::Custom("Test".to_string()).as_str(), "Test");
    }

    #[test]
    fn test_sound_default() {
        assert!(matches!(Sound::default(), Sound::Glass));
    }

    #[test]
    fn test_notification_input_with_special_characters() {
        let input_data = r#"{
            "session_id": "test-session-789",
            "transcript_path": "/path/to/transcript.md",
            "message": "Message with \"quotes\" and special chars",
            "title": "Title with \"quotes\""
        }"#;

        // Test that we can parse the JSON with special characters correctly
        let input: Result<NotificationInput, _> = serde_json::from_str(input_data);
        assert!(input.is_ok());

        let input = input.unwrap();
        assert_eq!(input.message, "Message with \"quotes\" and special chars");
        assert_eq!(input.title, Some("Title with \"quotes\"".to_string()));
    }

    #[test]
    fn test_sound_path_resolution() {
        // Test system sound path resolution
        assert_eq!(
            Sound::Glass.get_afplay_path(),
            "/System/Library/Sounds/Glass.aiff"
        );
        assert_eq!(
            Sound::Submarine.get_afplay_path(),
            "/System/Library/Sounds/Submarine.aiff"
        );

        // Test custom path pass-through
        let custom_sound = Sound::Custom("/custom/path/sound.wav".to_string());
        assert_eq!(custom_sound.get_afplay_path(), "/custom/path/sound.wav");

        // Test relative path pass-through
        let relative_sound = Sound::Custom("./sounds/custom.aiff".to_string());
        assert_eq!(relative_sound.get_afplay_path(), "./sounds/custom.aiff");
    }

    #[test]
    fn test_sound_path_edge_cases() {
        // Test sound name that happens to contain a slash but isn't meant as a path
        let edge_case = Sound::Custom("weird/name".to_string());
        assert_eq!(edge_case.get_afplay_path(), "weird/name");

        // Test empty custom sound (edge case) - gets treated as system sound
        let empty_custom = Sound::Custom("".to_string());
        assert_eq!(
            empty_custom.get_afplay_path(),
            "/System/Library/Sounds/.aiff"
        );
    }
}
