use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "gitxel-art",
    author = "Tanguy Pauwels, Aza",
    version = "0.2.0",
    about = "Create pixel art in your GitHub contribution calendar using automated backdated commits"
)]
pub struct Cli {
    /// Path to the pixel art image (PNG/JPEG, max 49x7)
    #[arg(short, long, value_name = "FILE")]
    pub image: Option<PathBuf>,

    /// Target local git repository path
    #[arg(short, long, value_name = "DIR")]
    pub repo: Option<PathBuf>,

    /// Target year for the contributions (>= 1974)
    #[arg(short, long, value_name = "YEAR")]
    pub year: Option<i32>,

    /// Name of the dummy file to modify in the repo
    #[arg(short, long, default_value = "dummy.txt")]
    pub dummy_file: String,

    /// Preview only in the terminal (does not generate commits)
    #[arg(short, long)]
    pub preview: bool,

    /// Automatically push commits to origin/main after generation
    #[arg(long)]
    pub push: bool,

    /// Dry run: simulate commit generation without modifying any files or git
    #[arg(long)]
    pub dry_run: bool,

    /// Run in interactive mode
    #[arg(long)]
    pub interactive: bool,
}

impl Cli {
    /// Returns true if enough arguments were provided to run non-interactively without prompt
    pub fn is_non_interactive(&self) -> bool {
        !self.interactive && (self.image.is_some() || self.repo.is_some() || self.year.is_some() || self.preview)
    }
}
