use std::path::PathBuf;

use walkdir::WalkDir;

/// Get the directories in the given path.
pub(crate) fn walk_path(
    path: PathBuf,
    min_depth: Option<usize>,
    max_depth: Option<usize>,
    skip_files: bool,
) -> color_eyre::Result<Vec<String>> {
    // Canonicalize the path to get absolute path.
    Ok(WalkDir::new(std::fs::canonicalize(path)?)
        .max_depth(max_depth.unwrap_or(1))
        .min_depth(min_depth.unwrap_or(1))
        .into_iter()
        .filter_entry(|e| {
            e.metadata()
                .map_or(false, |meta| if skip_files { meta.is_dir() } else { true })
        })
        .flatten()
        .map(|e| e.path().display().to_string())
        .collect::<Vec<String>>())
}
