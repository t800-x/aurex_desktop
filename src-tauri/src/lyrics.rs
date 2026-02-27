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
    let path = &track.track.file_path;
    let ttml = swap_extension(path, "ttml");
    let lrc = swap_extension(path, "lrc");

    if std::path::Path::new(&ttml).exists() {
        LineLyrics::from_ttml(&ttml)
    } else {
        LineLyrics::from_lrc(&lrc)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, Type)]
pub struct LineLyrics {
    start_time: f64,
    end_time: Option<f64>,
    line: String,
    writers: String,
}

impl LineLyrics {

    //TTML
    pub fn from_ttml(filepath: &str) -> Vec<LineLyrics> {
        if let Ok(content) = fs::read_to_string(filepath) {

            if content.contains("itunes:timing=\"Word\"") {
                println!("This is a word-level TTML file, not line-level. Aborting.");
                return Vec::new();
            }

            let writers = Self::extract_writers(&content);

            let mut lyrics = Vec::new();
            let lines: Vec<&str> = content.lines().collect();

            for line in lines {
                let line = line.trim();
                if line.starts_with("<p ") {
                    if let Some(lyric) = Self::parse_p_tag(line, &writers) {
                        lyrics.push(lyric);
                    }
                }
            }

            return lyrics;
        }

        Vec::new()
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

    //LRC
    pub fn from_lrc(filepath: &str) -> Vec<LineLyrics> {
        let content = match fs::read_to_string(filepath) {
            Ok(c) => c,
            Err(_) => return Vec::new(),
        };

        let mut offset_ms: f64 = 0.0;
        let mut writers = String::new();
        // collected as (start_time, line_text) before we assign end times
        let mut raw: Vec<(f64, String)> = Vec::new();

        for line in content.lines() {
            let line = line.trim();

            // metadata tags: [tag:value]
            if let Some(meta) = Self::lrc_meta(line) {
                match meta.0.to_lowercase().as_str() {
                    "offset" => {
                        // offset is in milliseconds, positive = shift forward
                        offset_ms = meta.1.trim().parse::<f64>().unwrap_or(0.0);
                    }
                    "au" | "ar" => {
                        // au = author/lyricist, ar = artist. use au as writers if present
                        if meta.0.to_lowercase() == "au" {
                            writers = meta.1.trim().to_string();
                        }
                    }
                    _ => {}
                }
                continue;
            }

            // lyric lines can have multiple time tags: [mm:ss.xx][mm:ss.xx]actual text
            let timestamps = Self::lrc_timestamps(line);
            if timestamps.is_empty() {
                continue;
            }

            // everything after the last timestamp tag is the lyric text
            let text = Self::strip_word_timestamps(Self::lrc_strip_timestamps(line)).trim().to_string();

            // skip pure instrumental/blank lines but keep empty string lines
            // (some lrc files use empty lines to mark gaps, still valid)
            for ts in timestamps {
                raw.push((ts, text.clone()));
            }
        }

        if raw.is_empty() {
            return Vec::new();
        }

        // sort by start time since multiple timestamps per line can be out of order
        raw.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

        // apply offset (convert ms to seconds)
        let offset_sec = offset_ms / 1000.0;

        let mut lyrics: Vec<LineLyrics> = Vec::new();
        for (i, (start, text)) in raw.iter().enumerate() {
            let start_time = (start + offset_sec).max(0.0);
            let end_time = raw
                .get(i + 1)
                .map(|(next_start, _)| (next_start + offset_sec).max(0.0));

            lyrics.push(LineLyrics {
                start_time,
                end_time,
                line: text.clone(),
                writers: writers.clone(),
            });
        }

        lyrics
    }

    /// Parses a metadata line like [tag:value], returns None if it looks like a timestamp tag
    fn lrc_meta(line: &str) -> Option<(String, String)> {
        if !line.starts_with('[') {
            return None;
        }
        let inner = line.strip_prefix('[')?.strip_suffix(']')?;
        // timestamp tags are [digits:...], metadata tags have alpha keys
        let colon = inner.find(':')?;
        let key = &inner[..colon];
        // if the key is purely digits it's a timestamp, not metadata
        if key.chars().next()?.is_ascii_digit() {
            return None;
        }
        // also reject if the line has multiple brackets (lyric line with timestamps)
        if line.matches('[').count() > 1 {
            return None;
        }
        Some((key.to_string(), inner[colon + 1..].to_string()))
    }

    /// Extracts all timestamps from a line, returns them in seconds
    fn lrc_timestamps(line: &str) -> Vec<f64> {
        let mut times = Vec::new();
        let mut remaining = line;
        while remaining.starts_with('[') {
            let close = match remaining.find(']') {
                Some(i) => i,
                None => break,
            };
            let inner = &remaining[1..close];
            if let Some(t) = Self::parse_lrc_timestamp(inner) {
                times.push(t);
            } else {
                break; // not a timestamp, stop
            }
            remaining = &remaining[close + 1..];
        }
        times
    }

    /// Strips all leading timestamp tags, returning the lyric text
    fn lrc_strip_timestamps(line: &str) -> &str {
        let mut remaining = line;
        while remaining.starts_with('[') {
            let close = match remaining.find(']') {
                Some(i) => i,
                None => break,
            };
            let inner = &remaining[1..close];
            if Self::parse_lrc_timestamp(inner).is_some() {
                remaining = &remaining[close + 1..];
            } else {
                break;
            }
        }
        remaining
    }

    /// Parses [mm:ss.xx] or [mm:ss.xxx] or [mm:ss] inner content to seconds
    fn parse_lrc_timestamp(inner: &str) -> Option<f64> {
        // must start with a digit
        if !inner.chars().next()?.is_ascii_digit() {
            return None;
        }
        let colon = inner.find(':')?;
        let minutes: f64 = inner[..colon].parse().ok()?;
        let seconds: f64 = inner[colon + 1..].parse().ok()?;
        Some(minutes * 60.0 + seconds)
    }

    fn strip_word_timestamps(text: &str) -> String {
        let mut result = String::new();
        let mut remaining = text;
        while let Some(open) = remaining.find('<') {
            result.push_str(&remaining[..open]);
            remaining = &remaining[open + 1..];
            if let Some(close) = remaining.find('>') {
                let inner = &remaining[..close];
                // only strip if it looks like a timestamp (digit-first), keep actual < > chars
                if inner.chars().next().map_or(false, |c| c.is_ascii_digit()) {
                    remaining = &remaining[close + 1..];
                } else {
                    result.push('<');
                }
            } else {
                result.push('<');
            }
        }
        result.push_str(remaining);
        result
    }
}