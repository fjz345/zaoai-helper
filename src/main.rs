use std::{env, fmt::Display, path::Path};

use anyhow::{Error, Result};

use zaoai_types::chapters::Chapters;
use zaoai_types::file::{
    EntryKind, clear_folder_contents, list_dir, relative_after, relative_before,
};
use zaoai_types::utils::{ListDirSplit, list_dir_with_kind_has_chapters_split};

use crate::mkv::{collect_list_dir_split, collect_series_with_chapters, path_exists};

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

    collect_list_dir_split(path, out_path).unwrap();

    // let read_list_dir_split = ListDirSplit::from_file_json(out_path).unwrap();
    // dbg!(read_list_dir_split);

    Ok(())
}
