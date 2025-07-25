use std::fs::File;
use std::io::Read;
use std::time::Duration;
use std::{env, fmt::Display, path::Path};

use anyhow::{Error, Result};
use std::io::Write;

use zaoai_types::ai_labels::{ZAOAI_LABEL_VERSION, ZaoaiLabel, collect_zaoai_labels};
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

    #[allow(unused)]
    let args: Vec<String> = env::args().collect();
    // // args[0] is the program name
    // if args.len() < 2 {
    //     eprintln!("Usage: {} <path>", args[0]);
    // }

    let media_path =
        std::env::var("ZAOAI_MEDIA_PATH").unwrap_or_else(|_| "test/test_Source".to_string());
    let list_dir_split_out_path = "output\\list_dir_split.json";
    path_exists(&media_path);

    println!("Collecting list_dir_split...");
    collect_list_dir_split(media_path, list_dir_split_out_path).unwrap();
    println!("Finished collecting list_dir_split!");

    println!("Collecting zaoai_labels...");
    let zaoai_labels_out_path = "output\\zaoai_labels";
    // let out_path = "output\\zaoai_labels";
    let read_list_dir_split =
        ListDirSplit::from_file_json("output\\list_dir_split_001.json").unwrap();
    collect_zaoai_labels(&read_list_dir_split, zaoai_labels_out_path)?;
    println!("Finished collecting zaoai_labels!");

    Ok(())
}
