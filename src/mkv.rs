use std::{
    fs::File,
    path::{Path, PathBuf},
    time::Duration,
};

use anyhow::Error;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::io::Write;
use zaoai_types::{
    chapters::{ChapterAtom, VideoMetadata},
    file::{self, clear_folder_contents, list_dir},
    utils::list_dir_with_kind_has_chapters_split,
};
use {
    zaoai_types::chapters::{Chapters, extract_chapters},
    zaoai_types::file::{EntryKind, get_top_level_dir},
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MkvMetadata {
    pub path: PathBuf,
    #[serde(with = "humantime_serde")]
    pub duration: Duration,
    pub chapters: Vec<ChapterAtom>,
}

impl Into<VideoMetadata> for MkvMetadata {
    fn into(self) -> VideoMetadata {
        VideoMetadata {
            container_format: Some("mkv".to_owned()),
            duration: self.duration,
            chapters: self.chapters,

            ..Default::default()
        }
    }
}

// ffprobe -select_streams v -show_frames -show_entries frame=pkt_pts_time -of csv input.mkv

pub fn process_mkv_file(entry: &EntryKind) -> Result<MkvMetadata> {
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

    // Read chapters
    let chapters = extract_chapters(path)?.unwrap_or_default();

    let metadata = MkvMetadata {
        path: path.clone(),
        chapters: chapters.into(),
        duration: Duration::new(1337, 0),
    };

    Ok(metadata)
}

pub(crate) fn collect_series_with_chapters(
    path: impl AsRef<Path>,
    out_path: impl AsRef<Path>,
) -> Result<()> {
    let list_of_entries = list_dir(&path, true).expect("");
    let list_dir_split = list_dir_with_kind_has_chapters_split(&list_of_entries, true).expect("");

    let entry_kind_vec_string = |vec: &Vec<EntryKind>| -> Vec<String> {
        vec.iter()
            .map(|f| match f {
                EntryKind::File(path_buf)
                | EntryKind::Directory(path_buf)
                | EntryKind::Other(path_buf) => {
                    path_buf.file_stem().unwrap().to_string_lossy().to_string()
                }
            })
            .collect::<Vec<_>>()
    };
    println!(
        "With chapters: {:?}",
        entry_kind_vec_string(&list_dir_split.with_chapters)
    );
    println!(
        "Without chapters: {:?}",
        entry_kind_vec_string(&list_dir_split.without_chapters)
    );

    println!("Clearing output path...");
    clear_folder_contents(out_path.as_ref()).expect("Could not clear output path");

    // Process each EntryKind::File
    for item in list_dir_split.with_chapters {
        match &item {
            file::EntryKind::File(path_buf) => {
                let b = process_mkv_file(&item);
                match b {
                    Ok(mkv_metadata) => {
                        let video_metadata: VideoMetadata = mkv_metadata.into();
                        let base_dir = path.as_ref();
                        let output_dir = out_path.as_ref();

                        let top_level_dir =
                            get_top_level_dir(path_buf, base_dir)?.ok_or_else(|| {
                                anyhow::anyhow!("File is directly in base_dir without subdirectory")
                            })?;

                        let output_path = output_dir
                            .join(&top_level_dir)
                            .join(path_buf.file_name().unwrap())
                            .with_extension("chapters.txt");

                        if let Some(parent) = output_path.parent() {
                            std::fs::create_dir_all(parent)?;
                        }

                        if output_path.exists() {
                            eprintln!(
                                "Warning: Output file already exists and will be overwritten: {}",
                                output_path.display()
                            );
                        }

                        let mut file = File::create(&output_path)?;
                        let json = serde_json::to_string_pretty(&video_metadata)?;
                        writeln!(file, "{}", json)?;
                        println!("Wrote: {}", output_path.display());
                    }
                    Err(e) => println!("{e}"),
                }
            }
            file::EntryKind::Directory(_path_buf) => {
                println!("Directory!, should not be triggered");
            }
            file::EntryKind::Other(_path_buf) => todo!(),
        }
    }

    Ok(())
}

pub(crate) fn collect_list_dir_split(
    path: impl AsRef<Path>,
    out_path: impl AsRef<Path>,
) -> Result<()> {
    let list_of_entries = list_dir(&path, true).expect("");
    let list_dir_split = list_dir_with_kind_has_chapters_split(&list_of_entries, true).expect("");

    let entry_kind_vec_string = |vec: &Vec<EntryKind>| -> Vec<String> {
        vec.iter()
            .map(|f| match f {
                EntryKind::File(path_buf)
                | EntryKind::Directory(path_buf)
                | EntryKind::Other(path_buf) => {
                    path_buf.file_stem().unwrap().to_string_lossy().to_string()
                }
            })
            .collect::<Vec<_>>()
    };
    println!(
        "With chapters: {:?}",
        entry_kind_vec_string(&list_dir_split.with_chapters)
    );
    println!(
        "Without chapters: {:?}",
        entry_kind_vec_string(&list_dir_split.without_chapters)
    );

    let mut out_file = std::fs::File::create(&out_path)?;

    if !path_exists(&out_path) || !out_path.as_ref().is_file() {
        anyhow::bail!("Not valid output file path");
    }
    out_file.write_all(&serde_json::to_vec_pretty(&list_dir_split)?)?;

    Ok(())
}

pub(crate) fn path_exists(path: impl AsRef<Path>) -> bool {
    let exists = std::path::Path::new(path.as_ref()).exists();
    if exists {
        println!("✅ Path exists: {}", path.as_ref().display());
    } else {
        println!("❌ Path does NOT exist: {}", path.as_ref().display());
    }

    exists
}
