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

    let commit_grid = image_to_commit_grid(&image_path)
        .with_context(|| format!("Failed to process image: {}", image_path.display()))?;

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

    if commits_count > 0 && push && !dry_run {
        push_commits(&repo_path)?;
    } else if commits_count > 0 && !dry_run {
        println!(
            "\n{} Commits created locally. You can push them using: {}",
            "INFO:".bold().cyan(),
            format!("git -C \"{}\" push origin main", repo_path.display()).yellow()
        );
    }

    println!("\n{}", "Done! ✨".bold().green());
    Ok(())
}
