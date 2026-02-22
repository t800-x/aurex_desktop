use std::fs;
use serde::{Deserialize, Serialize};
use specta::Type;

use crate::models::FullTrack;

fn swap_extension(path: &str, new_ext: &str) -> String {
    match path.rfind('.') {
        Some(i) => format!("{}.{}", &path[..i], new_ext.trim_start_matches('.')),
        None => format!("{}.{}", path, new_ext.trim_start_matches('.')),
    }
}

#[tauri::command]
#[specta::specta]
pub async fn get_line_lyrics(track: FullTrack) -> Vec<LineLyrics> {
    LineLyrics::from_ttml(swap_extension(&track.track.file_path, "ttml").as_str())
}

#[derive(Clone, Serialize, Deserialize, Debug, Type)]
pub struct LineLyrics {
    start_time: f64,
    end_time: Option<f64>,
    line: String,
    writers: String,
}

impl LineLyrics {
    pub fn from_ttml(filepath: &str) -> Vec<LineLyrics> {
        let content = fs::read_to_string(filepath).expect("Failed to read TTML file");

        if content.contains("itunes:timing=\"Word\"") {
            panic!("This is a word-level TTML file, not line-level. Aborting.");
        }

        let writers = Self::extract_writers(&content);

        let mut lyrics = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        for line in lines {
            let line = line.trim();
            if line.starts_with("<p ") {
                if let Some(lyric) = Self::parse_p_tag(line, &writers) {
                    lyrics.push(lyric.clone());
                    dbg!(lyric);
                }
            }
        }

        lyrics
    }

    fn extract_writers(content: &str) -> String {
        let mut writers = Vec::new();
        let mut remaining = content;

        while let Some(start) = remaining.find("<songwriter>") {
            let start = start + "<songwriter>".len();
            remaining = &remaining[start..];
            if let Some(end) = remaining.find("</songwriter>") {
                writers.push(remaining[..end].trim().to_string());
                remaining = &remaining[end..];
            }
        }

        writers.join(", ")
    }

    fn parse_time(time_str: &str) -> f64 {
        let time_str = time_str.trim_matches('"');
        if time_str.contains(':') {
            let parts: Vec<&str> = time_str.splitn(2, ':').collect();
            let minutes: f64 = parts[0].parse().unwrap_or(0.0);
            let seconds: f64 = parts[1].parse().unwrap_or(0.0);
            minutes * 60.0 + seconds
        } else {
            time_str.parse().unwrap_or(0.0)
        }
    }

    fn parse_p_tag(tag_line: &str, writers: &str) -> Option<LineLyrics> {
        let begin = Self::extract_attr(tag_line, "begin")?;
        let end = Self::extract_attr(tag_line, "end");

        let text_start = tag_line.find('>')? + 1;
        let text_end = tag_line.rfind("</p>")?;
        let text = &tag_line[text_start..text_end];

        Some(LineLyrics {
            start_time: Self::parse_time(&begin),
            end_time: end.map(|e| Self::parse_time(&e)),
            line: text.trim().to_string(),
            writers: writers.to_string(),
        })
    }

    fn extract_attr(tag: &str, attr: &str) -> Option<String> {
        let search = format!("{}=\"", attr);
        let start = tag.find(&search)? + search.len();
        let end = tag[start..].find('"')? + start;
        Some(tag[start..end].to_string())
    }
}