use std::{env, fmt::Display, path::Path};

use crate::{
    chapters::{Chapters, read_chapters_from_mkv},
    file::{EntryKind, clear_folder_contents, list_dir_with_kind, relative_after, relative_before},
    utils::{list_dir_with_kind_has_chapters_split, process_mkv_file},
};

mod chapters;
mod file;
mod temp;
mod utils;

/*
Want to do:
* Read all files on Series/
* split read files in to two folders, no chapters, chapters, also make sure to output a .txt with list of animes that had chapters
* Futher analyze chapters folder, has opening or not
*/

fn main() {
    #[allow(unused)]
    let args: Vec<String> = env::args().collect();

    // // args[0] is the program name
    // if args.len() < 2 {
    //     eprintln!("Usage: {} <path>", args[0]);
    //     std::process::exit(1);
    // }

    // let input_file = &args[1];
    // println!("Input path: {}", input_file);

    // let test_file = "test/test1.mkv";
    // let _result = read_chapters_from_mkv(test_file);

    let list_of_entries = list_dir_with_kind(r#"test\test_Source"#, true).expect("failed");

    let (with_chapters, without_chapters) =
        list_dir_with_kind_has_chapters_split(&list_of_entries, true).expect("failed");

    let entry_kind_vec_format = |vec: &Vec<EntryKind>| -> Vec<String> {
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
    println!("With chapters: {:?}", entry_kind_vec_format(&with_chapters));
    println!(
        "Without chapters: {:?}",
        entry_kind_vec_format(&without_chapters)
    );

    let output_dir = Path::new("output");
    clear_folder_contents(output_dir).expect("Could not clear");

    let mut file_stack = with_chapters;
    while let Some(item) = file_stack.pop() {
        match &item {
            file::EntryKind::File(_path_buf) => {
                let b = process_mkv_file(&item, &Path::new("test\\test_Source"), output_dir);
                match b {
                    Ok(o) => {}
                    Err(e) => println!("{e}"),
                }
            }
            file::EntryKind::Directory(path_buf) => match list_dir_with_kind(path_buf, true) {
                Ok(res) => {
                    for r in res {
                        file_stack.push(r);
                    }
                }
                Err(e) => println!("{:?}", e),
            },
            file::EntryKind::Other(_path_buf) => todo!(),
        }
    }
}
