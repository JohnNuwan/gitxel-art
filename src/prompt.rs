use anyhow::{anyhow, Result};
use chrono::{Datelike, Local};
use inquire::validator::Validation;
use inquire::{Confirm, Select, Text};
use std::fs;
use std::path::{Path, PathBuf};

pub struct InteractiveConfig {
    pub image_path: PathBuf,
    pub repo_path: PathBuf,
    pub dummy_file: String,
    pub year: i32,
    pub preview_only: bool,
    pub push: bool,
    pub dry_run: bool,
}

pub fn run_interactive_wizard() -> Result<InteractiveConfig> {
    // 1. Image selection
    let default_space_invader = PathBuf::from("./pixel_art/Space_invader.jpg");
    let mut image_options: Vec<String> = Vec::new();

    // Check pixel_art folder for available presets
    if let Ok(entries) = fs::read_dir("./pixel_art") {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                let ext_lower = ext.to_lowercase();
                if ext_lower == "png" || ext_lower == "jpg" || ext_lower == "jpeg" {
                    image_options.push(path.to_string_lossy().to_string());
                }
            }
        }
    }

    image_options.sort();
    image_options.push("Custom path...".to_string());

    let image_path = if !image_options.is_empty() {
        let choice = Select::new("Select a pixel art image:", image_options)
            .with_help_message("Pick an existing template or specify your own image file")
            .prompt()
            .map_err(|e| anyhow!("Prompt error: {}", e))?;

        if choice == "Custom path..." {
            let custom_path = Text::new("Enter image path:")
                .with_default("./pixel_art/Space_invader.jpg")
                .prompt()
                .map_err(|e| anyhow!("Prompt error: {}", e))?;
            PathBuf::from(custom_path.trim())
        } else {
            PathBuf::from(choice)
        }
    } else {
        default_space_invader
    };

    // 2. Repository path
    let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let repo_path_str = Text::new("Enter your local Git repository path:")
        .with_default(&current_dir.to_string_lossy())
        .with_help_message("Target repository where commits will be created")
        .with_validator(|input: &str| {
            let path = Path::new(input.trim());
            if !path.exists() {
                Ok(Validation::Invalid("Specified path does not exist".into()))
            } else if !path.join(".git").exists() {
                Ok(Validation::Invalid("Directory is not a Git repository (no .git folder found)".into()))
            } else {
                Ok(Validation::Valid)
            }
        })
        .prompt()
        .map_err(|e| anyhow!("Prompt error: {}", e))?;

    let repo_path = PathBuf::from(repo_path_str.trim());

    // 3. Dummy file name
    let dummy_file = Text::new("Enter dummy file name:")
        .with_default("dummy.txt")
        .with_validator(|input: &str| {
            let trimmed = input.trim();
            if trimmed.is_empty() {
                Ok(Validation::Invalid("File name cannot be empty".into()))
            } else {
                Ok(Validation::Valid)
            }
        })
        .prompt()
        .map_err(|e| anyhow!("Prompt error: {}", e))?;

    // 4. Target year
    let default_year = Local::now().year() - 1; // Default to previous year to fill a full completed year
    let year_str = Text::new("Enter the target year for pixel art:")
        .with_default(&default_year.to_string())
        .with_validator(|input: &str| {
            match input.trim().parse::<i32>() {
                Ok(y) if y < 1974 => Ok(Validation::Invalid("Git commits cannot predate 1974".into())),
                Ok(y) if y > 2100 => Ok(Validation::Invalid("Year seems too far in the future".into())),
                Ok(_) => Ok(Validation::Valid),
                Err(_) => Ok(Validation::Invalid("Please enter a valid year (e.g. 2024)".into())),
            }
        })
        .prompt()
        .map_err(|e| anyhow!("Prompt error: {}", e))?;

    let year: i32 = year_str.trim().parse()?;

    // 5. Actions
    let action_choices = vec![
        "Preview in terminal only",
        "Generate commits locally",
        "Generate commits and push to GitHub",
        "Dry run (simulation only)",
    ];

    let action = Select::new("What would you like to do?", action_choices)
        .prompt()
        .map_err(|e| anyhow!("Prompt error: {}", e))?;

    let (preview_only, push, dry_run) = match action {
        "Preview in terminal only" => (true, false, false),
        "Generate commits locally" => (false, false, false),
        "Generate commits and push to GitHub" => {
            let confirmed = Confirm::new("Are you sure you want to automatically push to origin/main?")
                .with_default(false)
                .prompt()
                .map_err(|e| anyhow!("Prompt error: {}", e))?;
            (false, confirmed, false)
        }
        "Dry run (simulation only)" => (false, false, true),
        _ => (true, false, false),
    };

    Ok(InteractiveConfig {
        image_path,
        repo_path,
        dummy_file: dummy_file.trim().to_string(),
        year,
        preview_only,
        push,
        dry_run,
    })
}
