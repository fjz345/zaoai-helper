use serde::{Deserialize, Serialize};
use serde_xml_rs::de::from_str;
use std::{fs, path::Path, process::Command};
use std::{fs::File, io::Write};

pub fn extract_chapters(mkv_file: &str, out_xml: &str) -> anyhow::Result<()> {
    let tool_path = Path::new("third_party/bin/mkvextract.exe");

    let status = Command::new(tool_path)
        .arg(mkv_file)
        .arg("chapters")
        .arg(out_xml)
        .status()?;

    if !status.success() {
        anyhow::bail!("Failed to extract chapters from {mkv_file}");
    }

    Ok(())
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Chapters {
    #[serde(rename = "EditionEntry")]
    edition_entry: EditionEntry,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EditionEntry {
    #[serde(rename = "ChapterAtom", default)]
    chapters: Vec<ChapterAtom>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChapterAtom {
    #[serde(rename = "ChapterTimeStart")]
    start_time: String,

    #[serde(rename = "ChapterTimeEnd")]
    end_time: Option<String>,

    #[serde(rename = "ChapterDisplay")]
    display: ChapterDisplay,
}

#[derive(Debug, Deserialize, Serialize)]
struct ChapterDisplay {
    #[serde(rename = "ChapterString")]
    title: String,
}

pub fn parse_chapter_xml(xml: &str) -> anyhow::Result<Chapters> {
    let chapters: Chapters = from_str(xml)?;
    Ok(chapters)
}

pub fn read_chapters_from_mkv(mkv_file: &str) -> anyhow::Result<()> {
    let xml_file = "chapters.xml";

    extract_chapters(mkv_file, xml_file)?;

    let xml_content = fs::read_to_string(xml_file)?;
    let parsed = parse_chapter_xml(&xml_content)?;

    for chapter in parsed.edition_entry.chapters {
        println!(
            "Start: {}, End: {:?}, Title: {}",
            chapter.start_time, chapter.end_time, chapter.display.title
        );
    }

    Ok(())
}

fn chapters_to_xml(chapters: &Chapters) -> anyhow::Result<String> {
    let inner = serde_xml_rs::to_string(chapters)?;
    Ok(format!(r#"<?xml version="1.0"?>\n{}"#, inner))
}

pub fn add_chapter_to_mkv(mkv_file: &str, timestamp: &str, title: &str) -> anyhow::Result<()> {
    let xml_file = "chapters.xml";

    // Step 1: Extract current chapters
    extract_chapters(mkv_file, xml_file)?;

    // Step 2: Read and parse chapters
    let xml_content = std::fs::read_to_string(xml_file)?;
    let mut chapters = parse_chapter_xml(&xml_content)?;

    // Step 3: Create and append new chapter
    let new_chapter = ChapterAtom {
        start_time: timestamp.to_string(),
        end_time: None,
        display: ChapterDisplay {
            title: title.to_string(),
        },
    };
    chapters.edition_entry.chapters.push(new_chapter);

    // Step 4: Serialize back to XML
    let xml_output = chapters_to_xml(&chapters)?;

    // Step 5: Write XML back to file
    let mut file = File::create(xml_file)?;
    file.write_all(xml_output.as_bytes())?;

    // Step 6: Apply changes via mkvpropedit
    let status = Command::new("third_party/bin/mkvpropedit.exe")
        .arg(mkv_file)
        .arg("--chapters")
        .arg(xml_file)
        .status()?;

    if !status.success() {
        anyhow::bail!("Failed to apply chapters with mkvpropedit");
    }

    Ok(())
}
