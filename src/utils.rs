use crate::{
    chapters::{Chapters, read_chapters_from_mkv},
    file::{EntryKind, get_top_level_dir, list_dir_with_kind},
    temp::copy_to_temp,
};
use anyhow::{Error, Result};
use std::{
    fs::{self, File},
    path::{Path, PathBuf},
};

use std::io::Write;

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
                // ignore .txt files
                if path.extension().unwrap_or_default() == "txt" {
                    continue;
                }
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
    println!("{:?}", path);
    if let Some(ext) = path.extension() {
        if ext == "mkv" {
            // Copy only this file to temp
            let (_temp_dir, temp_file_path) = copy_to_temp(path)?;
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

pub struct MkvMetadata {
    pub path: PathBuf,
    pub chapters: Chapters,
    pub duration: f64,
}

pub fn process_mkv_file(
    entry: &EntryKind,
    base_dir: &Path,
    output_dir: &Path,
) -> Result<MkvMetadata> {
    // Only process files
    let path = match entry {
        EntryKind::File(p) => p,
        _ => return Err(anyhow::anyhow!("Only processes files")),
    };

    // Check if it's an .mkv file
    if path.extension().and_then(|s| s.to_str()).unwrap_or("") != "mkv" {
        let string = format!("Only .mkv supported for now, {}", path.display());
        return Err(anyhow::anyhow!(string));
    }

    println!("Processing: {}", path.display());

    // Read chapters
    let chapters = read_chapters_from_mkv(path.to_str().unwrap())?;

    let metadata = MkvMetadata {
        path: path.clone(),
        chapters,
        duration: 20.0,
    };

    // Get the top-level directory under base_dir
    let top_level_dir = get_top_level_dir(path, base_dir)?
        .ok_or_else(|| anyhow::anyhow!("File is directly in base_dir without subdirectory"))?;

    // Build output path
    let output_path = output_dir
        .join(&top_level_dir)
        .join(path.file_name().unwrap())
        .with_extension("chapters.txt");

    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Check if output file already exists and warn
    if output_path.exists() {
        eprintln!(
            "Warning: Output file already exists and will be overwritten: {}",
            output_path.display()
        );
    }

    let mut file = File::create(&output_path)?;

    for chapter in &metadata.chapters {
        writeln!(
            file,
            "Start: {:<12} End: {:<12} Title: {}",
            chapter.start_time,
            chapter
                .end_time
                .clone()
                .unwrap_or_else(|| "???".to_string()),
            chapter.display.title
        )?;
    }

    println!("Wrote: {}", output_path.display());

    Ok(metadata)
}
