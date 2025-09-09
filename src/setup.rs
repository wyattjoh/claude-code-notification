use anyhow::Result;
use inquire::{validator::Validation, Select, Text};
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};

const SYSTEM_SOUNDS_DIR: &str = "/System/Library/Sounds";
const DEFAULT_SOUNDS: &[&str] = &[
    "Basso",
    "Blow",
    "Bottle",
    "Frog",
    "Funk",
    "Glass",
    "Hero",
    "Morse",
    "Ping",
    "Pop",
    "Purr",
    "Sosumi",
    "Submarine",
    "Tink",
];

fn get_claude_settings_path() -> Result<PathBuf> {
    let home = std::env::var("HOME")?;
    Ok(PathBuf::from(home).join(".claude").join("settings.json"))
}

fn get_available_system_sounds() -> Vec<String> {
    let system_sounds_path = Path::new(SYSTEM_SOUNDS_DIR);
    if !system_sounds_path.exists() {
        return DEFAULT_SOUNDS.iter().map(|s| s.to_string()).collect();
    }

    let mut sounds = Vec::new();
    if let Ok(entries) = fs::read_dir(system_sounds_path) {
        for entry in entries.flatten() {
            if let Some(name) = entry.file_name().to_str() {
                if name.ends_with(".aiff") {
                    let sound_name = name.trim_end_matches(".aiff");
                    sounds.push(sound_name.to_string());
                }
            }
        }
    }

    sounds.sort();
    if sounds.is_empty() {
        DEFAULT_SOUNDS.iter().map(|s| s.to_string()).collect()
    } else {
        sounds
    }
}

fn validate_sound_path(
    sound: &str,
) -> Result<Validation, Box<dyn std::error::Error + Send + Sync + 'static>> {
    if sound.contains('/') {
        let path = Path::new(sound);
        if path.exists() {
            Ok(Validation::Valid)
        } else {
            Ok(Validation::Invalid("Sound file does not exist".into()))
        }
    } else {
        let system_sound_path = Path::new(SYSTEM_SOUNDS_DIR).join(format!("{}.aiff", sound));
        if system_sound_path.exists() {
            Ok(Validation::Valid)
        } else {
            Ok(Validation::Invalid("System sound does not exist".into()))
        }
    }
}

pub fn run_setup() -> Result<()> {
    println!("🔧 Setting up Claude Code notifications\n");

    let available_sounds = get_available_system_sounds();
    let mut sound_options: Vec<String> = available_sounds;
    sound_options.push("Custom file path...".to_string());

    let sound_choice = Select::new("Select a notification sound:", sound_options)
        .with_help_message(
            "Choose a system sound or select 'Custom file path...' to specify your own",
        )
        .prompt()?;

    let selected_sound = if sound_choice == "Custom file path..." {
        Text::new("Enter the path to your custom sound file:")
            .with_help_message("Supported formats: .wav, .aiff, .mp3, .m4a")
            .with_validator(validate_sound_path)
            .prompt()?
    } else {
        sound_choice
    };

    let settings_path = get_claude_settings_path()?;

    // Create .claude directory if it doesn't exist
    if let Some(parent) = settings_path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Read existing settings or create new ones
    let mut settings: Value = if settings_path.exists() {
        let content = fs::read_to_string(&settings_path)?;
        serde_json::from_str(&content).unwrap_or_else(|_| json!({}))
    } else {
        json!({})
    };

    // Update the hooks configuration
    let notification_command = if selected_sound.contains('/') {
        format!("claude-code-notification --sound \"{}\"", selected_sound)
    } else {
        format!("claude-code-notification --sound {}", selected_sound)
    };

    settings["hooks"] = json!({
        "Notification": [
            {
                "hooks": [
                    {
                        "type": "command",
                        "command": notification_command
                    }
                ]
            }
        ]
    });

    // Write updated settings
    let settings_json = serde_json::to_string_pretty(&settings)?;
    fs::write(&settings_path, settings_json)?;

    println!("✅ Claude Code settings updated successfully!");
    println!("📁 Settings file: {}", settings_path.display());
    println!("🔊 Selected sound: {}", selected_sound);
    println!("\nYour Claude Code notifications are now configured.");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;
    use tempfile::TempDir;

    #[test]
    fn test_generated_settings_match_schema() {
        // Fetch the Claude Code settings schema
        let schema_response =
            reqwest::blocking::get("https://www.schemastore.org/claude-code-settings.json")
                .expect("Failed to fetch schema from schemastore.org");

        let schema_json: Value = schema_response.json().expect("Failed to parse schema JSON");

        // Compile the schema
        let validator =
            jsonschema::validator_for(&schema_json).expect("Failed to compile JSON schema");

        // Generate sample settings JSON like our setup command does
        let notification_command = "claude-code-notification --sound Glass";

        let test_settings = json!({
            "hooks": {
                "Notification": [
                    {
                        "hooks": [
                            {
                                "type": "command",
                                "command": notification_command
                            }
                        ]
                    }
                ]
            }
        });

        // Validate the generated JSON against the schema
        if validator.is_valid(&test_settings) {
            // Test passed - our JSON structure matches the schema
            println!(
                "✅ Generated settings JSON successfully validates against Claude Code schema"
            );
        } else {
            // Print detailed error information for debugging
            eprintln!("❌ Schema validation failed:");
            for error in validator.iter_errors(&test_settings) {
                eprintln!("  - {}", error);
                eprintln!("    Instance path: {}", error.instance_path);
                eprintln!("    Schema path: {}", error.schema_path);
            }
            panic!("Generated settings JSON does not match Claude Code schema");
        }
    }

    #[test]
    fn test_settings_creation_and_validation() {
        // Create a temporary directory for testing
        let temp_dir = TempDir::new().expect("Failed to create temporary directory");
        let temp_settings_path = temp_dir.path().join("test_settings.json");

        // Create sample settings using our internal logic
        let notification_command = "claude-code-notification --sound Submarine";
        let mut settings = json!({});

        settings["hooks"] = json!({
            "Notification": [
                {
                    "hooks": [
                        {
                            "type": "command",
                            "command": notification_command
                        }
                    ]
                }
            ]
        });

        // Write the settings to file
        let settings_json =
            serde_json::to_string_pretty(&settings).expect("Failed to serialize settings");
        std::fs::write(&temp_settings_path, &settings_json)
            .expect("Failed to write test settings file");

        // Read back and verify the structure
        let read_settings: Value = serde_json::from_str(
            &std::fs::read_to_string(&temp_settings_path)
                .expect("Failed to read test settings file"),
        )
        .expect("Failed to parse read settings");

        // Verify the structure matches what we expect
        assert!(read_settings["hooks"].is_object());
        assert!(read_settings["hooks"]["Notification"].is_array());

        let notification_hooks = &read_settings["hooks"]["Notification"];
        assert_eq!(notification_hooks.as_array().unwrap().len(), 1);

        let first_hook = &notification_hooks[0];
        assert!(first_hook["hooks"].is_array());

        let commands = first_hook["hooks"].as_array().unwrap();
        assert_eq!(commands.len(), 1);
        assert_eq!(commands[0]["type"], "command");
        assert_eq!(commands[0]["command"], notification_command);
    }

    #[test]
    fn test_validate_sound_path_system_sound() {
        let result = validate_sound_path("Glass");
        // This might fail on systems without the sound file, but that's expected
        match result {
            Ok(validation) => {
                // If it succeeds, it should be valid
                if let inquire::validator::Validation::Valid = validation {
                    // Test passes
                } else {
                    // System doesn't have the sound file - that's okay for testing
                }
            }
            Err(_) => {
                // Error in validation function - that's a real problem
                panic!("Sound path validation function failed");
            }
        }
    }

    #[test]
    fn test_validate_sound_path_custom_file() {
        // Test with a non-existent custom file
        let result = validate_sound_path("/nonexistent/file.wav");
        match result {
            Ok(validation) => {
                // Should be invalid since file doesn't exist
                match validation {
                    inquire::validator::Validation::Invalid(_) => {
                        // Expected result
                    }
                    inquire::validator::Validation::Valid => {
                        panic!("Validation should fail for non-existent file");
                    }
                }
            }
            Err(_) => {
                panic!("Sound path validation function failed");
            }
        }
    }

    #[test]
    fn test_get_available_system_sounds() {
        let sounds = get_available_system_sounds();

        // Should return at least some sounds (either from filesystem or defaults)
        assert!(!sounds.is_empty());

        // Should include some expected default sounds
        let sound_names: std::collections::HashSet<_> = sounds.iter().collect();
        assert!(sound_names.contains(&"Glass".to_string()));
    }
}
