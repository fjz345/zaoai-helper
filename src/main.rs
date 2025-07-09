use std::fs;

use crate::chapters::{extract_chapters, parse_chapter_xml, read_chapters_from_mkv};

mod chapters;

fn main() {
    let mkv_file = "test/test1.mkv";
    let result = read_chapters_from_mkv(mkv_file);
    result.expect("failed");
}
