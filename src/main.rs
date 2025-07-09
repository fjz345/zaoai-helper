use std::env;

use crate::{
    chapters::read_chapters_from_mkv,
    file::list_dir_with_kind,
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

    let result2 = list_dir_with_kind(r#"test\test_Source"#, true);
    let r = result2.expect("failed");

    let result2 = list_dir_with_kind_has_chapters_split(&r, true);
    let (with_chapters, without_chapters) = result2.expect("failed");

    println!(
        "With chapters: {:?}",
        with_chapters
            .iter()
            .map(|f| match f {
                file::EntryKind::File(path_buf)
                | file::EntryKind::Directory(path_buf)
                | file::EntryKind::Other(path_buf) => {
                    path_buf.file_stem().unwrap().display()
                }
            })
            .collect::<Vec<_>>()
    );
    println!(
        "Without chapters: {:?}",
        without_chapters
            .iter()
            .map(|f| match f {
                file::EntryKind::File(path_buf)
                | file::EntryKind::Directory(path_buf)
                | file::EntryKind::Other(path_buf) => {
                    path_buf.file_stem().unwrap().display()
                }
            })
            .collect::<Vec<_>>()
    );

    let mut file_stack = with_chapters;
    while let Some(item) = file_stack.pop() {
        match &item {
            file::EntryKind::File(_path_buf) => {
                let b = process_mkv_file(&item);
                b.expect("ASD");
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
