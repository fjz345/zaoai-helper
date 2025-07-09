use std::fs;

use crate::chapters::{extract_chapters, parse_chapter_xml, read_chapters_from_mkv};

mod chapters;

fn main() {
    let xml_file = "chapters.xml";

    let mkv_file = "test/test1.mkv";
    extract_chapters(mkv_file, xml_file).expect("ADS");

    let xml_content = fs::read_to_string(xml_file).expect("FAUILED");

    let parsed = parse_chapter_xml(&xml_content).expect("FAISD");

    let result = read_chapters_from_mkv(mkv_file);
    result.expect("failed");
}
