use std::fs::File;
use std::io::Read;
use std::time::Duration;
use std::{env, fmt::Display, path::Path};

use anyhow::{Error, Result};
use std::io::Write;

use zaoai_types::ai_labels::{ZAOAI_LABEL_VERSION, ZaoaiLabel};
use zaoai_types::chapters::{Chapters, VideoMetadata};
use zaoai_types::file::{
    EntryKind, clear_folder_contents, get_top_level_dir, list_dir, relative_after, relative_before,
};
use zaoai_types::utils::{ListDirSplit, list_dir_with_kind_has_chapters_split};

use crate::mkv::{
    collect_list_dir_split, collect_series_with_chapters, path_exists, process_mkv_file,
};

mod mkv;

/*
Want to do:
* Read all files in folder -R
* Move all files to new folder, split read files in to two folders, no chapters, chapters, also make sure to output a .txt with list of animes that had chapters
* Futher analyze chapters folder, has opening or not
*/

fn main() -> Result<()> {
    unsafe { env::set_var("RUST_BACKTRACE", "1") };

    #[allow(unused)]
    let args: Vec<String> = env::args().collect();

    // // args[0] is the program name
    // if args.len() < 2 {
    //     eprintln!("Usage: {} <path>", args[0]);
    //     std::process::exit(1);
    // }

    let path = "test\\test_Source";
    let out_path = "output\\list_dir_split.json";
    path_exists(path);

    // collect_list_dir_split(path, out_path).unwrap();

    let out_path = "output\\zaoai_labels";
    let read_list_dir_split =
        ListDirSplit::from_file_json("output\\list_dir_split_001.json").unwrap();
    dbg!(&read_list_dir_split);

    for entry_with_chapters in &read_list_dir_split.with_chapters {
        let path_buf = entry_with_chapters.as_ref();
        let zaoai_label = if path_buf.is_file() {
            if path_buf.is_file() {
                let b = process_mkv_file(&entry_with_chapters);
                match b {
                    Ok(mkv_metadata) => {
                        let (op_start, op_end) = mkv_metadata.extract_opening_times();

                        if op_start.is_some() && op_end.is_some() {
                            let opening_start_time = op_start.unwrap();
                            let opening_end_time = op_end.unwrap();

                            let video_metadata: VideoMetadata = mkv_metadata.into();
                            let total_secs = video_metadata.duration.as_secs_f64();

                            let ai_label = ZaoaiLabel {
                                path: path_buf.clone(),
                                metadata: video_metadata,
                                version: ZAOAI_LABEL_VERSION,
                                opening_start_time: Some(opening_start_time),
                                opening_end_time: Some(opening_end_time),
                                opening_start_frame: None,
                                opening_end_frame: None,
                                // Not sure if it should be div_eclid or div_ceil
                                opening_start_normalized: Some(
                                    opening_start_time.as_secs_f64() / total_secs,
                                ),
                                opening_end_normalized: Some(
                                    opening_end_time.as_secs_f64() / total_secs,
                                ),
                                ..Default::default()
                            };
                            Some(ai_label)
                        } else {
                            None
                        }
                    }
                    Err(e) => {
                        println!("{e}");
                        None
                    }
                }
            } else {
                None
            }
        } else {
            None
        };

        if let Some(label) = zaoai_label {
            let base_dir = path.as_ref();

            let top_level_dir = get_top_level_dir(&path_buf, base_dir)?.ok_or_else(|| {
                anyhow::anyhow!("File is directly in base_dir without subdirectory")
            })?;

            let output_dir = Path::new(out_path);
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
            let json = serde_json::to_string_pretty(&label)?;

            writeln!(file, "{}", json)?;
            println!("Wrote: {}", output_path.display());
        }
    }

    Ok(())
}
