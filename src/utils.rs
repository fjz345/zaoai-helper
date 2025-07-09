use crate::{
    chapters::read_chapters_from_mkv,
    file::{EntryKind, list_dir_with_kind},
    temp::copy_to_temp,
};
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

pub(crate) fn get_third_party_binary(name: &str) -> PathBuf {
    // CARGO_MANIFEST_DIR will be the zaohelper/ path even when used from zaoai
    let base = Path::new(env!("CARGO_MANIFEST_DIR"));
    base.join("third_party/bin").join(name)
}

pub(crate) fn list_dir_with_kind_has_chapters_split(
    list: &[EntryKind],
    cull_empty_folders: bool,
) -> Result<(Vec<EntryKind>, Vec<EntryKind>)> {
    let mut with_chapters = Vec::new();
    let mut without_chapters = Vec::new();

    for item in list {
        match item {
            EntryKind::File(path_buf) => {
                if has_chapters(path_buf)? {
                    with_chapters.push(item.clone());
                } else {
                    without_chapters.push(item.clone());
                }
            }
            EntryKind::Directory(path_buf) => {
                // Recursively check if all files inside have chapters
                if all_files_have_chapters(path_buf, cull_empty_folders)? {
                    with_chapters.push(item.clone());
                } else {
                    without_chapters.push(item.clone());
                }
            }
            EntryKind::Other(_) => {
                // Skip or handle Other if needed
            }
        }
    }

    println!(
        "With Chapters: {}\nWithout Chapters: {}",
        with_chapters.len(),
        without_chapters.len()
    );

    Ok((with_chapters, without_chapters))
}

fn all_files_have_chapters(dir_path: &Path, cull_empty_folders: bool) -> Result<bool> {
    let entries = list_dir_with_kind(dir_path, cull_empty_folders)?;

    for entry in entries {
        match entry {
            EntryKind::File(ref path) => {
                if !has_chapters(path)? {
                    return Ok(false);
                }
            }
            EntryKind::Directory(ref path) => {
                if !all_files_have_chapters(path, cull_empty_folders)? {
                    return Ok(false);
                }
            }
            EntryKind::Other(_) => {
                // skip or treat as no chapters (your choice)
            }
        }
    }

    Ok(true)
}

fn has_chapters(path: &Path) -> Result<bool> {
    if let Some(ext) = path.extension() {
        if ext == "mkv" {
            // Copy only this file to temp
            let (temp_dir, temp_file_path) = copy_to_temp(path)?;
            let mkv_file_str = temp_file_path
                .to_str()
                .ok_or_else(|| anyhow::anyhow!("Invalid temp path string"))?;

            // Read chapters from local copy
            let chapters = match read_chapters_from_mkv(mkv_file_str) {
                Ok(chaps) => chaps,
                Err(_) => return Ok(false),
            };

            return Ok(!chapters.iter().next().is_none());
        }
    }
    Ok(false)
}
