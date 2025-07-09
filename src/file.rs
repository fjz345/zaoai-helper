use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};

#[derive(Debug, Clone)]
pub enum EntryKind {
    File(PathBuf),
    Directory(PathBuf),
    #[allow(dead_code)]
    Other(PathBuf), // symlink, device, etc.
}

pub fn list_dir_with_kind<P: AsRef<Path>>(
    path: P,
    cull_empty_folders: bool,
) -> Result<Vec<EntryKind>> {
    let entries = fs::read_dir(&path)
        .with_context(|| format!("Failed to read directory: {}", path.as_ref().display()))?;

    let mut results = Vec::new();

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        let file_type = entry.file_type()?;

        let kind = if file_type.is_file() {
            EntryKind::File(path)
        } else if file_type.is_dir() {
            if cull_empty_folders {
                // Check if the directory is empty
                let mut dir_iter = fs::read_dir(&path)
                    .with_context(|| format!("Failed to read directory: {}", path.display()))?;

                if dir_iter.next().is_none() {
                    // Directory is empty, skip adding it
                    continue;
                }
            }
            EntryKind::Directory(path)
        } else {
            EntryKind::Other(path)
        };

        results.push(kind);
    }

    Ok(results)
}
