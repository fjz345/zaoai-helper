use crate::chapters::read_chapters_from_mkv;

mod chapters;

fn main() {
    let result = read_chapters_from_mkv("test/test0.mkv");
    result.expect("failed");
}
