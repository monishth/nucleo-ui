use std::path::PathBuf;

use walkdir::WalkDir;

/// Get the directories in the given path.
pub fn get_directories(
    path: PathBuf,
    min_depth: Option<usize>,
    max_depth: Option<usize>,
) -> color_eyre::Result<Vec<String>> {
    // Canonicalize the path to get absolute path.
    Ok(WalkDir::new(std::fs::canonicalize(path)?)
        .max_depth(max_depth.unwrap_or(1))
        .min_depth(min_depth.unwrap_or(1))
        .into_iter()
        .filter_entry(|e| e.metadata().map_or(false, |meta| meta.is_dir()))
        .flatten()
        .map(|e| e.path().display().to_string())
        .collect::<Vec<String>>())
}
