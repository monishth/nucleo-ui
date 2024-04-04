use walkdir::WalkDir;

pub fn get_directories() -> Vec<String> {
    WalkDir::new("/home/monish/projects")
        .max_depth(1)
        .min_depth(1)
        .into_iter()
        .filter_entry(|e| e.metadata().map_or(false, |meta| meta.is_dir()))
        .flatten()
        .map(|e| e.path().display().to_string())
        .collect::<Vec<String>>()
}
