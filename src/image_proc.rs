use anyhow::{bail, Context, Result};
use image::GenericImageView;
use std::path::Path;

pub const MAX_WEEKS: usize = 49;
pub const DAYS_IN_WEEK: usize = 7;

/// Convert a grayscale pixel value (0-255) to a number of commits according to defined thresholds.
/// Darker pixels (near black) produce more commits to appear darker green on GitHub.
pub fn grayscale_to_commits(value: u8) -> usize {
    if value <= 51 {
        8 // Darkest green
    } else if value <= 102 {
        4 // Medium green
    } else if value <= 153 {
        2 // Light green
    } else if value <= 204 {
        1 // Very light green
    } else {
        0 // Empty / Dark background
    }
}

/// Load an image, convert to grayscale, and map to a 7x49 commit grid (7 days x 49 weeks).
/// If the image is smaller than 7x49, it will be safely placed in the grid with surrounding days empty.
pub fn image_to_commit_grid<P: AsRef<Path>>(image_path: P) -> Result<Vec<Vec<usize>>> {
    let path = image_path.as_ref();
    let img = image::open(path)
        .with_context(|| format!("Failed to open image file: {}", path.display()))?;

    let (width, height) = img.dimensions();

    if width > MAX_WEEKS as u32 || height > DAYS_IN_WEEK as u32 {
        bail!(
            "Image dimensions ({}x{}) exceed the maximum GitHub contribution grid limit of {}x{} pixels.",
            width,
            height,
            MAX_WEEKS,
            DAYS_IN_WEEK
        );
    }

    let gray = img.to_luma8();

    // Standardized 7 rows x 49 columns grid initialized to 0
    let mut commit_grid = vec![vec![0usize; MAX_WEEKS]; DAYS_IN_WEEK];

    for y in 0..height as usize {
        for x in 0..width as usize {
            let pixel = gray.get_pixel(x as u32, y as u32);
            let gray_value = pixel[0];
            commit_grid[y][x] = grayscale_to_commits(gray_value);
        }
    }

    Ok(commit_grid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grayscale_thresholds() {
        assert_eq!(grayscale_to_commits(0), 8);
        assert_eq!(grayscale_to_commits(51), 8);
        assert_eq!(grayscale_to_commits(52), 4);
        assert_eq!(grayscale_to_commits(102), 4);
        assert_eq!(grayscale_to_commits(103), 2);
        assert_eq!(grayscale_to_commits(153), 2);
        assert_eq!(grayscale_to_commits(154), 1);
        assert_eq!(grayscale_to_commits(204), 1);
        assert_eq!(grayscale_to_commits(205), 0);
        assert_eq!(grayscale_to_commits(255), 0);
    }
}
