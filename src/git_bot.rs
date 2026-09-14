use anyhow::{anyhow, Context, Result};
use chrono::{Datelike, Duration, NaiveDate, Weekday};
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::process::Command;

/// Calculate the first Sunday of the given year.
pub fn get_first_sunday_of_year(year: i32) -> Result<NaiveDate> {
    let mut date = NaiveDate::from_ymd_opt(year, 1, 1)
        .ok_or_else(|| anyhow!("Invalid year: {}", year))?;

    while date.weekday() != Weekday::Sun {
        date = date
            .succ_opt()
            .ok_or_else(|| anyhow!("Calendar calculation overflow while searching for first Sunday"))?;
    }

    Ok(date)
}

/// Verify that the target path is a valid Git repository.
pub fn validate_git_repo<P: AsRef<Path>>(repo_path: P) -> Result<()> {
    let path = repo_path.as_ref();
    if !path.exists() {
        return Err(anyhow!("Specified path does not exist: {}", path.display()));
    }
    if !path.join(".git").exists() {
        return Err(anyhow!(
            "Specified path is not a Git repository (missing .git folder): {}",
            path.display()
        ));
    }
    Ok(())
}

/// Parameters required to execute commit generation.
pub struct CommitConfig<'a> {
    pub repo_path: &'a Path,
    pub dummy_file: &'a str,
    pub year: i32,
    pub dry_run: bool,
}

/// Generate backdated Git commits based on the 7x49 commit grid.
pub fn generate_commits(commit_grid: &[Vec<usize>], config: &CommitConfig) -> Result<usize> {
    let sanitized_dummy = config.dummy_file.trim();
    if sanitized_dummy.is_empty() {
        return Err(anyhow!("Dummy file name cannot be empty."));
    }

    if !config.dry_run {
        validate_git_repo(config.repo_path)?;
    }

    let start_date = get_first_sunday_of_year(config.year)?;
    let dummy_path = config.repo_path.join(sanitized_dummy);

    let total_commits: usize = commit_grid.iter().map(|row| row.iter().sum::<usize>()).sum();

    if total_commits == 0 {
        println!("No commits to generate (image is completely blank).");
        return Ok(0);
    }

    let pb = ProgressBar::new(total_commits as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} commits ({eta}) {msg}")
            .expect("Invalid progress bar template")
            .progress_chars("#>-"),
    );

    let num_rows = commit_grid.len();
    let num_cols = if num_rows > 0 { commit_grid[0].len() } else { 0 };

    let mut commit_counter = 0usize;

    for week in 0..num_cols {
        for day in 0..num_rows {
            let num_commits = commit_grid[day][week];
            if num_commits == 0 {
                continue;
            }

            let commit_date = start_date
                .checked_add_signed(Duration::weeks(week as i64))
                .and_then(|d| d.checked_add_signed(Duration::days(day as i64)))
                .ok_or_else(|| anyhow!("Date calculation overflow for week {}, day {}", week, day))?;

            let date_str = commit_date.format("%Y-%m-%dT12:00:00").to_string();

            for _ in 0..num_commits {
                commit_counter += 1;
                if config.dry_run {
                    pb.set_message(format!("[DRY RUN] Would commit #{} for {}", commit_counter, date_str));
                } else {
                    // Append line to dummy file in isolated scope to ensure file is closed before git commands
                    {
                        let mut file = OpenOptions::new()
                            .create(true)
                            .append(true)
                            .open(&dummy_path)
                            .with_context(|| format!("Failed to open dummy file at {}", dummy_path.display()))?;

                        writeln!(file, "Commit #{} for {}", commit_counter, date_str)?;
                        file.flush()?;
                    }

                    // git add
                    let add_output = Command::new("git")
                        .arg("-C")
                        .arg(config.repo_path)
                        .arg("add")
                        .arg(sanitized_dummy)
                        .output()
                        .context("Failed to run 'git add'")?;

                    if !add_output.status.success() {
                        let stderr = String::from_utf8_lossy(&add_output.stderr);
                        return Err(anyhow!("'git add' failed:\n{}", stderr));
                    }

                    // git commit --allow-empty --date (with -q for quiet progress bar)
                    let commit_output = Command::new("git")
                        .arg("-C")
                        .arg(config.repo_path)
                        .arg("commit")
                        .arg("-q")
                        .arg("--allow-empty")
                        .arg("-m")
                        .arg("Automated commit")
                        .arg("--date")
                        .arg(&date_str)
                        .output()
                        .context("Failed to run 'git commit'")?;

                    if !commit_output.status.success() {
                        let stderr = String::from_utf8_lossy(&commit_output.stderr);
                        return Err(anyhow!("'git commit' failed:\n{}", stderr));
                    }
                }

                pb.inc(1);
            }
        }
    }

    if config.dry_run {
        pb.finish_with_message("[DRY RUN] Simulation complete.");
    } else {
        pb.finish_with_message("All commits successfully generated!");
    }

    Ok(total_commits)
}

/// Push generated commits to remote origin/main.
pub fn push_commits<P: AsRef<Path>>(repo_path: P) -> Result<()> {
    let path = repo_path.as_ref();
    println!("Pushing commits to origin/main...");

    let output = Command::new("git")
        .arg("-C")
        .arg(path)
        .arg("push")
        .arg("origin")
        .arg("main")
        .output()
        .context("Failed to execute 'git push'")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("'git push origin main' failed:\n{}", stderr));
    }

    println!("Successfully pushed commits to GitHub!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_sunday() {
        // 2024: Jan 1 was a Monday, first Sunday was Jan 7
        let sunday_2024 = get_first_sunday_of_year(2024).unwrap();
        assert_eq!(sunday_2024, NaiveDate::from_ymd_opt(2024, 1, 7).unwrap());
        assert_eq!(sunday_2024.weekday(), Weekday::Sun);

        // 2023: Jan 1 was a Sunday!
        let sunday_2023 = get_first_sunday_of_year(2023).unwrap();
        assert_eq!(sunday_2023, NaiveDate::from_ymd_opt(2023, 1, 1).unwrap());
        assert_eq!(sunday_2023.weekday(), Weekday::Sun);
    }
}
