use colored::*;

pub struct GitHubColors;

impl GitHubColors {
    pub const EMPTY: (u8, u8, u8) = (22, 27, 34);     // #161b22 - 0 commits
    pub const LEVEL_1: (u8, u8, u8) = (14, 68, 41);   // #0e4429 - 1 commit
    pub const LEVEL_2: (u8, u8, u8) = (1, 109, 50);   // #016d32 - 2 commits
    pub const LEVEL_3: (u8, u8, u8) = (38, 166, 65);  // #26a641 - 4 commits
    pub const LEVEL_4: (u8, u8, u8) = (57, 211, 83);  // #39d353 - 8 commits

    pub fn color_for_commits(commits: usize) -> (u8, u8, u8) {
        match commits {
            0 => Self::EMPTY,
            1 => Self::LEVEL_1,
            2 => Self::LEVEL_2,
            3 | 4 => Self::LEVEL_3,
            _ => Self::LEVEL_4,
        }
    }
}

/// Print a TrueColor ANSI preview of the GitHub contribution graph directly in the console.
pub fn render_graph_preview(commit_grid: &[Vec<usize>]) {
    let day_names = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
    let num_rows = commit_grid.len();
    let num_cols = if num_rows > 0 { commit_grid[0].len() } else { 0 };

    println!("\n{}", "GitHub Contribution Calendar Preview:".bold().cyan());
    println!("{}", "═".repeat(num_cols * 2 + 8).bright_black());

    // Print header with week markers (every 5 weeks)
    print!("     ");
    for w in 0..num_cols {
        if (w + 1) % 5 == 0 {
            print!("{:2}", (w + 1));
        } else if (w + 1) % 5 == 1 && w > 0 {
            print!("");
        } else {
            print!("  ");
        }
    }
    println!();

    let mut total_commits = 0usize;

    for (day_idx, row) in commit_grid.iter().enumerate() {
        let day_label = if day_idx < day_names.len() {
            day_names[day_idx]
        } else {
            "   "
        };

        print!("{} ", day_label.bright_black());

        for &commits in row {
            total_commits += commits;
            let (r, g, b) = GitHubColors::color_for_commits(commits);
            // Print a block character with TrueColor
            print!("\x1b[38;2;{};{};{}m■ \x1b[0m", r, g, b);
        }
        println!();
    }

    println!("{}", "═".repeat(num_cols * 2 + 8).bright_black());

    // Legend
    print!("Legend: {} ", "Less".bright_black());
    for &commits in &[0, 1, 2, 4, 8] {
        let (r, g, b) = GitHubColors::color_for_commits(commits);
        print!("\x1b[38;2;{};{};{}m■ \x1b[0m", r, g, b);
    }
    println!("{} (0, 1, 2, 4, 8 commits)", "More".bright_black());

    println!(
        "Total commits required: {}\n",
        total_commits.to_string().bold().green()
    );
}
