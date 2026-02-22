use rusqlite::Row;
use serde::{Deserialize, Serialize};
use specta::{specta, Type};


// ---------------------------------------------------------------------------
// Artist
// ---------------------------------------------------------------------------

#[derive(Clone, Serialize, Deserialize, Debug, Type)]
pub struct Artist {
    pub id: Option<i64>,
    pub name: String,
    pub genre: Option<String>,
}

impl Artist {
    pub fn from_row(row: &Row) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get("id")?,
            name: row.get("name")?,
            genre: row.get("genre")?,
        })
    }
}

// ---------------------------------------------------------------------------
// Album
// ---------------------------------------------------------------------------

#[derive(Clone, Serialize, Deserialize, Debug, Type)]
pub struct Album {
    pub id: Option<i64>,
    pub artist_id: i64,
    pub title: String,
    pub year: i64,
    pub genre: Option<String>,
    pub album_art: Option<String>,
}

impl Album {
    pub fn from_row(row: &Row) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get("id")?,
            artist_id: row.get("artist_id")?,
            title: row
                .get::<_, Option<String>>("title")?
                .unwrap_or_else(|| "Unknown Album".into()),
            year: row.get::<_, Option<i64>>("year")?.unwrap_or(0),
            genre: row.get("genre")?,
            album_art: row.get("album_art")?,
        })
    }
}

// ---------------------------------------------------------------------------
// Track
// ---------------------------------------------------------------------------

#[derive(Clone, Serialize, Deserialize, Debug, Type)]
pub struct Track {
    pub id: Option<i64>,
    pub album_id: i64,
    pub artist_id: i64,
    pub file_path: String,
    pub title: String,
    pub track_number: i64,
    pub disc_number: i64,
    pub bpm: i64,
    pub duration: i64,
    pub initial_key: Option<String>,
    pub isrc: Option<String>,
    pub lyrics: Option<String>,
    pub composer: Option<String>,
}

impl Track {
    pub fn from_row(row: &Row) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get("id")?,
            album_id: row.get("album_id")?,
            artist_id: row.get("artist_id")?,
            file_path: row.get("file_path")?,
            title: row
                .get::<_, Option<String>>("title")?
                .unwrap_or_else(|| "Unknown Title".into()),
            track_number: row.get::<_, Option<i64>>("track_number")?.unwrap_or(0),
            disc_number: row.get::<_, Option<i64>>("disc_number")?.unwrap_or(1),
            bpm: row.get::<_, Option<i64>>("bpm")?.unwrap_or(0),
            duration: row.get::<_, Option<i64>>("duration")?.unwrap_or(0),
            initial_key: row.get("initial_key")?,
            isrc: row.get("isrc")?,
            lyrics: row.get("lyrics")?,
            composer: row.get("composer")?,
        })
    }

    /// Format duration (milliseconds) into a human-readable string.
    pub fn formatted_duration(&self) -> String {
        if self.duration <= 0 {
            return "0:00".into();
        }
        let total_secs = self.duration / 1000;
        let hours = total_secs / 3600;
        let minutes = (total_secs % 3600) / 60;
        let seconds = total_secs % 60;

        if hours > 0 {
            format!("{hours}:{minutes:02}:{seconds:02}")
        } else {
            format!("{minutes}:{seconds:02}")
        }
    }
}

// ---------------------------------------------------------------------------
// FullTrack  (Track + joined artist/album data)
// ---------------------------------------------------------------------------

#[derive(Clone, Serialize, Deserialize, Debug, Type)]
pub struct FullTrack {
    pub track: Track,
    pub artist_name: String,
    pub album_title: String,
    pub album_art: Option<String>,
    pub playlist_position: Option<i64>,
}

impl FullTrack {
    pub fn from_row(row: &Row) -> rusqlite::Result<Self> {
        Ok(Self {
            track: Track::from_row(row)?,
            artist_name: row
                .get::<_, Option<String>>("artist_name")?
                .unwrap_or_else(|| "Unknown Artist".into()),
            album_title: row
                .get::<_, Option<String>>("album_title")?
                .unwrap_or_else(|| "Unknown Album".into()),
            album_art: row.get("album_art")?,
            playlist_position: row.get("position").ok(), // only present in playlist queries
        })
    }
}

// ---------------------------------------------------------------------------
// Playlist
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct Playlist {
    pub id: Option<i64>,
    pub name: String,
    pub cover_path: Option<String>,
    pub created_at: i64, // Unix ms timestamp
}

impl Playlist {
    pub fn from_row(row: &Row) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get("id")?,
            name: row.get("name")?,
            cover_path: row.get("cover_path")?,
            created_at: row.get("created_at")?,
        })
    }
}
