use std::fs;
use serde::{Deserialize, Serialize};
use specta::Type;

use crate::models::FullTrack;

// ==================== Data Structures ====================

#[derive(Clone, Serialize, Deserialize, Debug, Type, Default)]
pub struct Lyrics {
    pub writers: String,
    pub unsynced: Option<String>,
    pub line_lyrics: Option<Vec<LineLyrics>>,
    pub syllable_lyrics: Option<Vec<SyllableLine>>,
    pub lyricstype: LyricsType
}

#[derive(Clone, Serialize, Deserialize, Debug, Type)]
pub enum LyricsType {
    Line,
    Syllable,
    Unsynced
}

impl Default for LyricsType {
    fn default() -> Self {
        Self::Line
    }
}

/// One line of line-synced lyrics (line-level TTML or LRC)
#[derive(Clone, Serialize, Deserialize, Debug, Type)]
pub struct LineLyrics {
    pub start_time: f64,
    pub end_time: Option<f64>,
    pub real_end_time: Option<f64>,
    pub line: String,
    pub speaker: i32,
}

/// One line in word-timed lyrics (word-level TTML)
#[derive(Clone, Serialize, Deserialize, Debug, Type)]
pub struct SyllableLine {
    pub start_time: f64,
    pub end_time: f64,
    pub real_end_time: f64,
    pub speaker: i32,
    pub is_background: bool,
    pub words: Vec<SyllableWord>,
}

/// A single word/syllable within a SyllableLine
#[derive(Clone, Serialize, Deserialize, Debug, Type)]
pub struct SyllableWord {
    pub start_time: f64,
    pub end_time: f64,
    pub text: String,
}

// ==================== Command ====================

#[tauri::command]
#[specta::specta]
pub async fn get_lyrics(track: FullTrack) -> Lyrics {
    let path = &track.track.file_path;
    let ttml_path = swap_extension(path, "ttml");
    let lrc_path = swap_extension(path, "lrc");

    // Priority 1 & 2: TTML (word-level beats line-level)
    if let Ok(content) = fs::read_to_string(&ttml_path) {
        let writers = extract_writers_ttml(&content);
        return if content.contains(r#"itunes:timing="Word""#) {
            let lines = parse_word_ttml(&content);
            Lyrics {
                writers,
                syllable_lyrics: (!lines.is_empty()).then_some(lines),
                lyricstype: LyricsType::Syllable,
                ..Default::default()
            }
        } else {
            let lines = parse_line_ttml(&content);
            Lyrics {
                writers,
                line_lyrics: (!lines.is_empty()).then_some(lines),
                lyricstype: LyricsType::Line,
                ..Default::default()
            }
        };
    }

    // Priority 3: LRC file
    if let Ok(content) = fs::read_to_string(&lrc_path) {
        let (lines, writers) = parse_lrc(&content);
        if !lines.is_empty() {
            return Lyrics {
                writers,
                line_lyrics: Some(lines),
                lyricstype: LyricsType::Line,
                ..Default::default()
            };
        }
    }

    // Priority 4: embedded lyrics tag on the track
    if let Some(embedded) = &track.track.lyrics {
        let trimmed = embedded.trim();
        if !trimmed.is_empty() {
            if lrc_looks_synced(trimmed) {
                let (lines, writers) = parse_lrc(trimmed);
                if !lines.is_empty() {
                    return Lyrics {
                        writers,
                        line_lyrics: Some(lines),
                        lyricstype: LyricsType::Line,
                        ..Default::default()
                    };
                }
            }
            return Lyrics {
                unsynced: Some(trimmed.to_string()),
                lyricstype: LyricsType::Unsynced,
                ..Default::default()
            };
        }
    }

    Lyrics::default()
}

// ==================== TTML ====================

const TTM_NS: &str = "http://www.w3.org/ns/ttml#metadata";

fn extract_writers_ttml(content: &str) -> String {
    let Ok(doc) = roxmltree::Document::parse(content) else {
        return String::new();
    };
    doc.descendants()
        .filter(|n| n.tag_name().name() == "songwriter")
        .map(|n| collect_text_children(&n).trim().to_string())
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(", ")
}

fn parse_word_ttml(content: &str) -> Vec<SyllableLine> {
    let Ok(doc) = roxmltree::Document::parse(content) else {
        eprintln!("failed to parse word-level TTML");
        return Vec::new();
    };

    let mut lines = Vec::new();

    for node in doc.descendants() {
        if node.tag_name().name() != "p" {
            continue;
        }

        let start_time = node.attribute("begin").map(parse_time).unwrap_or(0.0);
        let end_time = node.attribute("end").map(parse_time).unwrap_or(0.0);
        let speaker = node
            .attribute((TTM_NS, "agent"))
            .map(agent_to_speaker)
            .unwrap_or(0);

        let mut main_words: Vec<SyllableWord> = Vec::new();
        let mut bg_words: Vec<SyllableWord> = Vec::new();

        for child in node.children() {
            if child.tag_name().name() != "span" {
                continue;
            }
            if child.attribute((TTM_NS, "role")) == Some("x-bg") {
                // background vocals: collect the nested spans
                for bg_span in child.children() {
                    if bg_span.tag_name().name() == "span" {
                        if let Some(word) = node_to_word(&bg_span) {
                            bg_words.push(word);
                        }
                    }
                }
            } else {
                if let Some(word) = node_to_word(&child) {
                    main_words.push(word);
                }
            }
        }

        if !main_words.is_empty() {
            lines.push(SyllableLine {
                start_time,
                end_time,
                real_end_time: end_time,
                speaker,
                is_background: false,
                words: main_words,
            });
        }

        if !bg_words.is_empty() {
            // use actual word timings for bg lines instead of the parent p bounds
            let bg_start = bg_words.first().map(|w| w.start_time).unwrap_or(start_time);
            let bg_end = bg_words.last().map(|w| w.end_time).unwrap_or(end_time);
            lines.push(SyllableLine {
                start_time: bg_start,
                end_time: bg_end,
                real_end_time: bg_end,
                speaker,
                is_background: true,
                words: bg_words,
            });
        }
    }

    lines
}

fn parse_line_ttml(content: &str) -> Vec<LineLyrics> {
    let Ok(doc) = roxmltree::Document::parse(content) else {
        eprintln!("failed to parse line-level TTML");
        return Vec::new();
    };

    let mut lines = Vec::new();

    for node in doc.descendants() {
        if node.tag_name().name() != "p" {
            continue;
        }
        let Some(begin) = node.attribute("begin") else {
            continue;
        };

        let start_time = parse_time(begin);
        let end_time = node.attribute("end").map(parse_time);
        let speaker = node
            .attribute((TTM_NS, "agent"))
            .map(agent_to_speaker)
            .unwrap_or(0);
        let line = collect_element_text(&node);

        if line.is_empty() {
            continue;
        }

        lines.push(LineLyrics { start_time, end_time, real_end_time: end_time, line, speaker });
    }

    lines
}

fn node_to_word(node: &roxmltree::Node) -> Option<SyllableWord> {
    let start_time = node.attribute("begin").map(parse_time)?;
    let end_time = node.attribute("end").map(parse_time)?;
    let text = collect_text_children(node).to_string();
    (!text.is_empty()).then_some(SyllableWord { start_time, end_time, text })
}

/// Collect only direct text node children (handles roxmltree's text() being None on elements)
fn collect_text_children(node: &roxmltree::Node) -> String {
    node.children()
        .filter(|n| n.is_text())
        .filter_map(|n| n.text())
        .collect()
}

/// Recursively collect all text content under an element (for line-level p tags)
fn collect_element_text(node: &roxmltree::Node) -> String {
    let mut buf = String::new();
    for child in node.children() {
        if child.is_text() {
            if let Some(t) = child.text() {
                buf.push_str(t);
            }
        } else if child.is_element() {
            buf.push_str(&collect_element_text(&child));
        }
    }
    buf.trim().to_string()
}

fn agent_to_speaker(agent: &str) -> i32 {
    match agent {
        "v1" => 0,
        "v2" => 1,
        "v1000" => 2, // group/ensemble
        _ => 0,
    }
}

/// Handles plain seconds ("11.497"), MM:SS.mmm ("1:16.185"), HH:MM:SS.mmm
fn parse_time(s: &str) -> f64 {
    let s = s.trim_matches('"');
    let parts: Vec<&str> = s.splitn(3, ':').collect();
    match parts.len() {
        3 => {
            let h: f64 = parts[0].parse().unwrap_or(0.0);
            let m: f64 = parts[1].parse().unwrap_or(0.0);
            let sec: f64 = parts[2].parse().unwrap_or(0.0);
            h * 3600.0 + m * 60.0 + sec
        }
        2 => {
            let m: f64 = parts[0].parse().unwrap_or(0.0);
            let sec: f64 = parts[1].parse().unwrap_or(0.0);
            m * 60.0 + sec
        }
        _ => s.parse().unwrap_or(0.0),
    }
}

// ==================== LRC ====================

fn lrc_looks_synced(content: &str) -> bool {
    content.lines().any(|line| {
        let line = line.trim();
        if !line.starts_with('[') {
            return false;
        }
        let Some(close) = line.find(']') else {
            return false;
        };
        let inner = &line[1..close];
        inner.chars().next().map_or(false, |c| c.is_ascii_digit())
    })
}

fn parse_lrc(content: &str) -> (Vec<LineLyrics>, String) {
    let mut offset_ms: f64 = 0.0;
    let mut writers = String::new();
    let mut raw: Vec<(f64, String)> = Vec::new();

    for line in content.lines() {
        let line = line.trim();

        if let Some((key, val)) = lrc_meta(line) {
            match key.to_lowercase().as_str() {
                "offset" => offset_ms = val.trim().parse().unwrap_or(0.0),
                "au" => writers = val.trim().to_string(),
                _ => {}
            }
            continue;
        }

        let timestamps = lrc_timestamps(line);
        if timestamps.is_empty() {
            continue;
        }

        let text = strip_word_timestamps(lrc_strip_timestamps(line))
            .trim()
            .to_string();

        for ts in timestamps {
            raw.push((ts, text.clone()));
        }
    }

    if raw.is_empty() {
        return (Vec::new(), writers);
    }

    raw.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

    let offset_sec = offset_ms / 1000.0;
    let lyrics = raw
        .iter()
        .enumerate()
        .map(|(i, (start, text))| {
            let start_time = (start + offset_sec).max(0.0);
            let end_time = raw.get(i + 1).map(|(ns, _)| (ns + offset_sec).max(0.0));
            LineLyrics { start_time, end_time, real_end_time: end_time, line: text.clone(), speaker: 0 }
        })
        .collect();

    (lyrics, writers)
}

fn lrc_meta(line: &str) -> Option<(String, String)> {
    if !line.starts_with('[') || line.matches('[').count() > 1 {
        return None;
    }
    let inner = line.strip_prefix('[')?.strip_suffix(']')?;
    let colon = inner.find(':')?;
    let key = &inner[..colon];
    if key.chars().next()?.is_ascii_digit() {
        return None;
    }
    Some((key.to_string(), inner[colon + 1..].to_string()))
}

fn lrc_timestamps(line: &str) -> Vec<f64> {
    let mut times = Vec::new();
    let mut remaining = line;
    while remaining.starts_with('[') {
        let Some(close) = remaining.find(']') else { break };
        let inner = &remaining[1..close];
        match parse_lrc_timestamp(inner) {
            Some(t) => {
                times.push(t);
                remaining = &remaining[close + 1..];
            }
            None => break,
        }
    }
    times
}

fn lrc_strip_timestamps(line: &str) -> &str {
    let mut remaining = line;
    while remaining.starts_with('[') {
        let Some(close) = remaining.find(']') else { break };
        if parse_lrc_timestamp(&remaining[1..close]).is_some() {
            remaining = &remaining[close + 1..];
        } else {
            break;
        }
    }
    remaining
}

fn parse_lrc_timestamp(inner: &str) -> Option<f64> {
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

// ==================== Utility ====================

fn swap_extension(path: &str, new_ext: &str) -> String {
    match path.rfind('.') {
        Some(i) => format!("{}.{}", &path[..i], new_ext.trim_start_matches('.')),
        None => format!("{}.{}", path, new_ext.trim_start_matches('.')),
    }
}