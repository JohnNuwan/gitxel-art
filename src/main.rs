mod cli;
mod git_bot;
mod image_proc;
mod preview;
mod prompt;

use anyhow::{Context, Result};
use clap::Parser;
use cli::Cli;
use colored::*;
use git_bot::{ensure_git_repo, generate_commits, push_commits, CommitConfig};
use image_proc::image_to_commit_grid;
use preview::render_graph_preview;
use prompt::run_interactive_wizard;
use std::path::{Path, PathBuf};

fn print_banner() {
    let logo = r#"
   ____ _ _             _         _         _   
  / ___(_) |_ _  _____ | |       / \   _ __| |_ 
 | |  _| | __\ \/ / _ \| |_____ / _ \ | '__| __|
 | |_| | | |_ >  <  __/| |_____/ ___ \| |  | |_ 
  \____|_|\__/_/\_\___||_|    /_/   \_\_|   \__|
                 ##          ##
                   ##      ##        
                 ##############
               ####  ######  ####
             ######################
             ##  ##############  ##     
             ##  ##          ##  ##
                   ####  ####
    "#;

    println!("{}", logo.bright_purple());
    println!(
        "{} {}",
        "Gitxel-Art (Rust Edition)".bold().cyan(),
        "v0.2.0".bright_black()
    );
    println!("{}", "═".repeat(50).bright_black());
    println!(
        "{} {}",
        "DISCLAIMER:".bold().yellow(),
        "Intended solely for educational and artistic purposes.".bright_black()
    );
    println!(
        "{}\n",
        "Ensure your target repository is private to protect your GitHub activity."
            .bright_black()
    );
}

fn main() -> Result<()> {
    print_banner();

    let cli = Cli::parse();

    let (image_path, repo_path, dummy_file, year, preview_only, push, dry_run) =
        if cli.is_non_interactive() {
            let image = cli
                .image
                .unwrap_or_else(|| PathBuf::from("./pixel_art/Space_invader.jpg"));
            let repo = cli
                .repo
                .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
            let year = cli.year.unwrap_or(2024);
            (
                image,
                repo,
                cli.dummy_file,
                year,
                cli.preview,
                cli.push,
                cli.dry_run,
            )
        } else {
            let config = run_interactive_wizard()?;
            (
                config.image_path,
                config.repo_path,
                config.dummy_file,
                config.year,
                config.preview_only,
                config.push,
                config.dry_run,
            )
        };

    println!(
        "{} Loading image from: {}",
        "==>".bold().green(),
        image_path.display().to_string().cyan()
    );

    let mut commit_grid = image_to_commit_grid(&image_path)
        .with_context(|| format!("Failed to process image: {}", image_path.display()))?;

    if cli.multiplier > 1 {
        println!(
            "{} Applying commit multiplier: {}x",
            "==>".bold().green(),
            cli.multiplier.to_string().cyan()
        );
        for row in &mut commit_grid {
            for commits in row {
                *commits *= cli.multiplier;
            }
        }
    }

    // Render TrueColor terminal preview
    render_graph_preview(&commit_grid);

    if preview_only {
        println!("{}", "Preview mode active: No commits generated.".bold().yellow());
        return Ok(());
    }

    if !dry_run {
        ensure_git_repo(&repo_path, cli.init)?;
    }

    println!(
        "{} Target repository: {}",
        "==>".bold().green(),
        repo_path.display().to_string().cyan()
    );
    println!(
        "{} Target year: {}",
        "==>".bold().green(),
        year.to_string().cyan()
    );
    println!(
        "{} Dummy file: {}",
        "==>".bold().green(),
        dummy_file.cyan()
    );

    let config = CommitConfig {
        repo_path: Path::new(&repo_path),
        dummy_file: &dummy_file,
        year,
        dry_run,
    };

    println!("\n{}", "Generating commits...".bold());
    let commits_count = generate_commits(&commit_grid, &config)?;

    let has_remote_origin = std::process::Command::new("git")
        .arg("-C")
        .arg(&repo_path)
        .arg("remote")
        .arg("get-url")
        .arg("origin")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    if commits_count > 0 && push && !dry_run {
        if has_remote_origin {
            push_commits(&repo_path)?;
        } else {
            println!(
                "\n{} Cannot push: no remote 'origin' is configured for this repository.\n\
                 First add your remote using:\n  git -C \"{}\" remote add origin <URL>\n\
                 Then push using:\n  git -C \"{}\" push -u origin main",
                "WARNING:".bold().yellow(),
                repo_path.display(),
                repo_path.display()
            );
        }
    } else if commits_count > 0 && !dry_run {
        if has_remote_origin {
            println!(
                "\n{} Commits created locally. You can push them using:\n  {}",
                "INFO:".bold().cyan(),
                format!("git -C \"{}\" push origin main", repo_path.display()).yellow()
            );
        } else {
            println!(
                "\n{} Commits created locally in: {}\n\
                 To display them on your GitHub profile, link a GitHub repository:\n  \
                 1. Create a repository on GitHub (private recommended)\n  \
                 2. Link it:  {}\n  \
                 3. Push it:  {}",
                "SUCCESS:".bold().green(),
                repo_path.display().to_string().cyan(),
                format!("git -C \"{}\" remote add origin https://github.com/<username>/<repo>.git", repo_path.display()).yellow(),
                format!("git -C \"{}\" push -u origin main", repo_path.display()).yellow()
            );
        }
    }

    println!("\n{}", "Done! ✨".bold().green());
    Ok(())
}
