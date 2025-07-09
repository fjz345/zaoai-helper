use std::{env, fs};

use crate::{
    chapters::{extract_chapters, parse_chapter_xml, read_chapters_from_mkv},
    file::list_dir_with_kind,
};

mod chapters;
mod file;
mod utils;

/*
Want to do:
* Read all files on Series/
* split read files in to two folders, no chapters, chapters, also make sure to output a .txt with list of animes that had chapters
* Futher analyze chapters folder, has opening or not
*/

fn main() {
    let args: Vec<String> = env::args().collect();

    // // args[0] is the program name
    // if args.len() < 2 {
    //     eprintln!("Usage: {} <path>", args[0]);
    //     std::process::exit(1);
    // }

    // let input_file = &args[1];
    // println!("Input path: {}", input_file);

    let test_file = "test/test1.mkv";
    let result = read_chapters_from_mkv(test_file);

    let result2 = list_dir_with_kind("test", true);
    let r = result2.expect("failed");
    println!("{:?}", r);
}
