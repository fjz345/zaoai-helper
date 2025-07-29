use std::fs::File;
use std::io::Read;
use std::time::Duration;
use std::{env, fmt::Display, path::Path};

mod soloud;

use anyhow::{Error, Result};
use serde_json::map::Entry;
use std::io::Write;
use zaoai_types::sound::S_SPECTOGRAM_NUM_BINS;
use zaoai_types::spectrogram::{
    SPECTOGRAM_HEIGHT, SPECTOGRAM_WIDTH, generate_spectogram, save_spectrogram,
};

use zaoai_types::ai_labels::{
    ZAOAI_LABEL_VERSION, ZaoaiLabel, collect_zaoai_labels, generate_zaoai_label_spectrograms,
};
use zaoai_types::chapters::{Chapters, VideoMetadata};
use zaoai_types::file::{
    EntryKind, clear_folder_contents, list_dir, relative_after, relative_before,
};
use zaoai_types::utils::{ListDirSplit, list_dir_with_kind_has_chapters_split};

use zaoai_types::mkv::{collect_list_dir_split, path_exists};

/*
Want to do:
* Read all files in folder -R
* Move all files to new folder, split read files in to two folders, no chapters, chapters, also make sure to output a .txt with list of animes that had chapters
* Futher analyze chapters folder, has opening or not
*/

fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    unsafe { env::set_var("RUST_BACKTRACE", "1") };

    unsafe { env::set_var("RUST_LOG", "debug") }; // or "info" or "debug"
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    #[allow(unused)]
    let args: Vec<String> = env::args().collect();
    // // args[0] is the program name
    // if args.len() < 2 {
    //     eprintln!("Usage: {} <path>", args[0]);
    // }

    let media_path =
        std::env::var("ZAOAI_MEDIA_PATH").unwrap_or_else(|_| "test/test_Source".to_string());
    let output_path = std::env::var("OUTPUT_PATH").unwrap_or_else(|_| "output".to_string());
    let mut zaoai_labels_out_path = output_path.clone();
    zaoai_labels_out_path.push_str("\\zaoai_labels");

    // println!("Collecting list_dir_split...");
    // let mut list_dir_split_out_path = output_path.clone();
    // list_dir_split_out_path.push_str("\\list_dir_split.json");
    // path_exists(&media_path);
    // collect_list_dir_split(media_path, list_dir_split_out_path).unwrap();
    // println!("Finished collecting list_dir_split!");

    // println!("Collecting zaoai_labels...");
    // let read_list_dir_split =
    //     ListDirSplit::from_file_json("output\\list_dir_split_001.json").unwrap();
    // collect_zaoai_labels(&read_list_dir_split, zaoai_labels_out_path)?;
    // println!("Finished collecting zaoai_labels!");

    println!("Generating Spectrograms for zaoai_labels...");
    let spectogram_width: usize = std::env::var("SPECTROGRAM_WIDTH")
        .ok()
        .and_then(|val| val.parse::<usize>().ok())
        .unwrap_or(SPECTOGRAM_WIDTH);
    let spectogram_height: usize = std::env::var("SPECTROGRAM_HEIGHT")
        .ok()
        .and_then(|val| val.parse::<usize>().ok())
        .unwrap_or(SPECTOGRAM_HEIGHT);
    let spectrogram_file_extension =
        std::env::var("SPECTROGRAM_EXTENSION").unwrap_or_else(|_| "spectrogram".to_string());

    let list_dir = list_dir(zaoai_labels_out_path, true)?;
    generate_zaoai_label_spectrograms(
        &list_dir,
        &spectrogram_file_extension,
        [spectogram_width, spectogram_height],
    )?;

    println!("Finished Generating Spectrograms!");

    Ok(())
}
