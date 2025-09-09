pub mod error;

use anyhow::Result;
use notify_rust::Notification;
use serde::{Deserialize, Serialize};
use std::io::Read;
use std::process::Command;
use std::thread;

pub use error::{NotificationError, NotificationResult};

#[derive(Debug, Deserialize, Serialize)]
pub struct NotificationInput {
    pub session_id: String,
    pub transcript_path: String,
    pub message: String,
    pub title: Option<String>,
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

pub fn main<R: Read>(mut stdin: R, sound: Sound) -> Result<()> {
    // Read all input from stdin
    let mut buffer = String::new();
    stdin.read_to_string(&mut buffer)?;

    // Parse the JSON input
    let input: NotificationInput = serde_json::from_str(&buffer)?;

    // Create and send the notification
    send_notification(&input, &sound)?;

    Ok(())
}

fn send_notification(input: &NotificationInput, sound: &Sound) -> Result<()> {
    let title = input.title.as_deref().unwrap_or("Claude Code");

    // Clone the sound for the thread
    let sound_clone = sound.clone();

    // Spawn a thread to play the sound in parallel
    let sound_handle = thread::spawn(move || {
        if let Err(e) = play_sound(&sound_clone) {
            eprintln!("Warning: Failed to play sound: {}", e);
        }
    });

    // Show the notification (this happens in parallel with sound)
    let notification_result = Notification::new()
        .summary(title)
        .body(&input.message)
        .show();

    // Wait for the sound thread to complete
    if let Err(e) = sound_handle.join() {
        eprintln!("Warning: Sound thread panicked: {:?}", e);
    }

    // Return the notification result
    notification_result?;
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
        let result = main(cursor, Sound::Glass);

        assert!(result.is_err());
    }

    #[test]
    fn test_empty_input() {
        let empty_input = "";
        let cursor = Cursor::new(empty_input);
        let result = main(cursor, Sound::Glass);

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
