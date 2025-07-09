use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};

#[derive(Debug)]
pub enum EntryKind {
    File(PathBuf),
    Directory(PathBuf),
    Other(PathBuf), // symlink, device, etc.
}

pub fn list_dir_with_kind<P: AsRef<Path>>(path: P) -> Result<Vec<EntryKind>> {
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
            EntryKind::Directory(path)
        } else {
            EntryKind::Other(path)
        };

        results.push(kind);
    }

    Ok(results)
}
