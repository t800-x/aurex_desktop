use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard, OnceLock};

use rusqlite::{params, Connection, OptionalExtension};

use crate::error::{LibraryError, Result};
use crate::models::{Album, Artist, FullTrack, Playlist, Track};

// ---------------------------------------------------------------------------
// Singletons
// ---------------------------------------------------------------------------

pub fn library_service() -> &'static Mutex<LibraryService> {
    static INSTANCE: OnceLock<Mutex<LibraryService>> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        Mutex::new(LibraryService::open().expect("Failed to open database. PANICKING"))
    })
}

// ---------------------------------------------------------------------------
// Commands
// ---------------------------------------------------------------------------
#[tauri::command]
#[specta::specta]
pub async fn fulltrack_from_id(id: i32) -> Option<FullTrack> {
    if let Ok(library) = library_service().lock() {
        if let Ok(result) = library.get_full_track_by_id(id.into()) {
           return result; 
        }
    }

    None
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

const FULL_TRACK_SELECT: &str = "
    SELECT
        t.*,
        r.name  AS artist_name,
        a.title AS album_title,
        a.album_art
    FROM tracks t
    JOIN artists r ON t.artist_id = r.id
    JOIN albums  a ON t.album_id  = a.id
";

fn db_path() -> Result<PathBuf> {
    let dir = dirs::data_local_dir().ok_or(LibraryError::NoAppDir)?;
    Ok(dir.join("aurex").join("library.db"))
}

// ---------------------------------------------------------------------------
// LibraryService
// ---------------------------------------------------------------------------

pub struct LibraryService {
    conn: Mutex<Connection>,
}

impl LibraryService {
    /// Open (or create) the database at the platform-appropriate location.
    pub fn open() -> Result<Self> {
        let path = db_path()?;

        // Make sure the parent directory exists.
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let conn = Connection::open(&path)?;
        conn.execute_batch("PRAGMA foreign_keys = ON;")?;
        Self::run_migrations(&conn)?;

        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    /// Open an in-memory database — useful for tests.
    pub fn open_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        conn.execute_batch("PRAGMA foreign_keys = ON;")?;
        Self::run_migrations(&conn)?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    fn lock(&self) -> MutexGuard<Connection> {
        self.conn.lock().expect("DB mutex poisoned")
    }

    // -----------------------------------------------------------------------
    // Schema
    // -----------------------------------------------------------------------

    fn run_migrations(conn: &Connection) -> Result<()> {
        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS artists (
                id      INTEGER PRIMARY KEY AUTOINCREMENT,
                name    TEXT    NOT NULL UNIQUE,
                genre   TEXT
            );

            CREATE TABLE IF NOT EXISTS albums (
                id          INTEGER PRIMARY KEY AUTOINCREMENT,
                artist_id   INTEGER NOT NULL,
                title       TEXT    NOT NULL,
                year        INTEGER,
                genre       TEXT,
                album_art   TEXT,
                FOREIGN KEY (artist_id) REFERENCES artists (id) ON DELETE CASCADE,
                UNIQUE (artist_id, title)
            );

            CREATE TABLE IF NOT EXISTS tracks (
                id              INTEGER PRIMARY KEY AUTOINCREMENT,
                album_id        INTEGER NOT NULL,
                artist_id       INTEGER NOT NULL,
                file_path       TEXT    NOT NULL UNIQUE,
                title           TEXT,
                track_number    INTEGER,
                disc_number     INTEGER,
                bpm             INTEGER,
                duration        INTEGER,
                initial_key     TEXT,
                isrc            TEXT,
                lyrics          TEXT,
                composer        TEXT,
                FOREIGN KEY (album_id)  REFERENCES albums  (id) ON DELETE CASCADE,
                FOREIGN KEY (artist_id) REFERENCES artists (id) ON DELETE CASCADE
            );

            CREATE TABLE IF NOT EXISTS playlists (
                id          INTEGER PRIMARY KEY AUTOINCREMENT,
                name        TEXT    NOT NULL,
                cover_path  TEXT,
                created_at  INTEGER
            );

            CREATE TABLE IF NOT EXISTS playlist_tracks (
                playlist_id INTEGER NOT NULL,
                track_id    INTEGER NOT NULL,
                position    INTEGER NOT NULL,
                FOREIGN KEY (playlist_id) REFERENCES playlists (id) ON DELETE CASCADE,
                FOREIGN KEY (track_id)    REFERENCES tracks    (id) ON DELETE CASCADE
            );
        ",
        )?;
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Track ingestion
    // -----------------------------------------------------------------------

    pub fn add_track_with_metadata(
        &self,
        file_path: &str,
        track_title: Option<&str>,
        artist_name: Option<&str>,
        album_artist: Option<&str>,
        album_title: Option<&str>,
        album_art: Option<&str>,
        genre: Option<&str>,
        duration: Option<i64>,
        year: Option<i64>,
        track_number: Option<i64>,
        disc_number: Option<i64>,
        bpm: Option<i64>,
        initial_key: Option<&str>,
        isrc: Option<&str>,
        lyrics: Option<&str>,
        composer: Option<&str>,
    ) -> Result<()> {
        let conn = self.lock();

        let effective_album_artist = album_artist.or(artist_name).unwrap_or("Unknown Artist");

        let effective_album_title = album_title.unwrap_or("Unknown Album");

        // 1. Upsert the album artist.
        let album_artist_id = upsert_artist(&conn, effective_album_artist, genre)?;

        // 2. Upsert the track artist (may differ from album artist on features/compilations).
        let track_artist_id = match artist_name {
            Some(name) if name != effective_album_artist => upsert_artist(&conn, name, genre)?,
            _ => album_artist_id,
        };

        // 3. Upsert album.
        let album_id = upsert_album(
            &conn,
            album_artist_id,
            effective_album_title,
            year,
            genre,
            album_art,
        )?;

        // 4. Insert (or replace) track.
        conn.execute(
            "INSERT INTO tracks (
                album_id, artist_id, file_path, title,
                track_number, disc_number, bpm, duration,
                initial_key, isrc, lyrics, composer
             ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12)
             ON CONFLICT(file_path) DO UPDATE SET
                album_id     = excluded.album_id,
                artist_id    = excluded.artist_id,
                title        = excluded.title,
                track_number = excluded.track_number,
                disc_number  = excluded.disc_number,
                bpm          = excluded.bpm,
                duration     = excluded.duration,
                initial_key  = excluded.initial_key,
                isrc         = excluded.isrc,
                lyrics       = excluded.lyrics,
                composer     = excluded.composer",
            params![
                album_id,
                track_artist_id,
                file_path,
                track_title.unwrap_or("Unknown Title"),
                track_number.unwrap_or(0),
                disc_number.unwrap_or(1),
                bpm.unwrap_or(0),
                duration.unwrap_or(0),
                initial_key,
                isrc,
                lyrics,
                composer,
            ],
        )?;

        Ok(())
    }

    // -----------------------------------------------------------------------
    // Queries — Artists
    // -----------------------------------------------------------------------

    pub fn get_all_artists(&self) -> Result<Vec<Artist>> {
        let conn = self.lock();
        let mut stmt = conn.prepare("SELECT * FROM artists ORDER BY name ASC")?;
        let rows = stmt.query_map([], Artist::from_row)?;
        rows.map(|r| r.map_err(Into::into)).collect()
    }

    pub fn get_artist_by_id(&self, id: i64) -> Result<Option<Artist>> {
        let conn = self.lock();
        let result = conn
            .query_row(
                "SELECT * FROM artists WHERE id = ?1",
                params![id],
                Artist::from_row,
            )
            .optional()?;
        Ok(result)
    }

    // -----------------------------------------------------------------------
    // Queries — Albums
    // -----------------------------------------------------------------------

    pub fn get_all_albums(&self) -> Result<Vec<Album>> {
        let conn = self.lock();
        let mut stmt = conn.prepare("SELECT * FROM albums ORDER BY title ASC")?;
        let rows = stmt.query_map([], Album::from_row)?;
        rows.map(|r| r.map_err(Into::into)).collect()
    }

    pub fn get_album_by_id(&self, id: i64) -> Result<Option<Album>> {
        let conn = self.lock();
        let result = conn
            .query_row(
                "SELECT * FROM albums WHERE id = ?1",
                params![id],
                Album::from_row,
            )
            .optional()?;
        Ok(result)
    }

    pub fn get_albums_by_artist(&self, artist_id: i64) -> Result<Vec<Album>> {
        let conn = self.lock();
        let mut stmt =
            conn.prepare("SELECT * FROM albums WHERE artist_id = ?1 ORDER BY year DESC")?;
        let rows = stmt.query_map(params![artist_id], Album::from_row)?;
        rows.map(|r| r.map_err(Into::into)).collect()
    }

    pub fn get_album_id_by_path(&self, file_path: &str) -> Result<Option<i64>> {
        let conn = self.lock();
        let result = conn
            .query_row(
                "SELECT album_id FROM tracks WHERE file_path = ?1",
                params![file_path],
                |row| row.get(0),
            )
            .optional()?;
        Ok(result)
    }

    pub fn update_album_art(&self, album_id: i64, album_art_path: Option<&str>) -> Result<()> {
        let conn = self.lock();
        conn.execute(
            "UPDATE albums SET album_art = ?1 WHERE id = ?2",
            params![album_art_path, album_id],
        )?;
        Ok(())
    }

    pub fn delete_album(&self, album_id: i64) -> Result<()> {
        let conn = self.lock();
        conn.execute("DELETE FROM albums WHERE id = ?1", params![album_id])?;
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Queries — Tracks
    // -----------------------------------------------------------------------

    pub fn get_all_tracks(&self) -> Result<Vec<FullTrack>> {
        let conn = self.lock();
        let sql = format!("{FULL_TRACK_SELECT} ORDER BY t.id ASC");
        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt.query_map([], FullTrack::from_row)?;
        rows.map(|r| r.map_err(Into::into)).collect()
    }

    pub fn get_track_by_id(&self, id: i64) -> Result<Option<Track>> {
        let conn = self.lock();
        let result = conn
            .query_row(
                "SELECT * FROM tracks WHERE id = ?1",
                params![id],
                Track::from_row,
            )
            .optional()?;
        Ok(result)
    }

    pub fn get_full_track_by_id(&self, id: i64) -> Result<Option<FullTrack>> {
        let conn = self.lock();
        let sql = format!("{FULL_TRACK_SELECT} WHERE t.id = ?1");
        let result = conn
            .query_row(&sql, params![id], FullTrack::from_row)
            .optional()?;
        Ok(result)
    }

    pub fn get_tracks_by_album(&self, album_id: i64) -> Result<Vec<Track>> {
        let conn = self.lock();
        let mut stmt = conn.prepare(
            "SELECT * FROM tracks WHERE album_id = ?1 ORDER BY disc_number, track_number",
        )?;
        let rows = stmt.query_map(params![album_id], Track::from_row)?;
        rows.map(|r| r.map_err(Into::into)).collect()
    }

    pub fn is_track_in_library(&self, file_path: &str) -> Result<bool> {
        let conn = self.lock();
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM tracks WHERE file_path = ?1",
            params![file_path],
            |row| row.get(0),
        )?;
        Ok(count > 0)
    }

    pub fn delete_track(&self, track_id: i64) -> Result<()> {
        let conn = self.lock();
        conn.execute("DELETE FROM tracks WHERE id = ?1", params![track_id])?;
        Ok(())
    }

    /// Simple title / artist search. Case-insensitive, substring match.
    pub fn search_tracks(&self, query: &str) -> Result<Vec<FullTrack>> {
        let conn = self.lock();
        let pattern = format!("%{query}%");
        let sql = format!(
            "{FULL_TRACK_SELECT}
             WHERE t.title LIKE ?1 OR r.name LIKE ?1
             ORDER BY t.title ASC"
        );
        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt.query_map(params![pattern], FullTrack::from_row)?;
        rows.map(|r| r.map_err(Into::into)).collect()
    }

    // -----------------------------------------------------------------------
    // Playlists
    // -----------------------------------------------------------------------

    pub fn create_playlist(&self, name: &str, cover_path: Option<&str>) -> Result<i64> {
        let conn = self.lock();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis() as i64)
            .unwrap_or(0);
        conn.execute(
            "INSERT INTO playlists (name, cover_path, created_at) VALUES (?1, ?2, ?3)",
            params![name, cover_path, now],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn delete_playlist(&self, playlist_id: i64) -> Result<()> {
        let conn = self.lock();
        conn.execute("DELETE FROM playlists WHERE id = ?1", params![playlist_id])?;
        Ok(())
    }

    pub fn get_all_playlists(&self) -> Result<Vec<Playlist>> {
        let conn = self.lock();
        let mut stmt = conn.prepare("SELECT * FROM playlists ORDER BY name ASC")?;
        let rows = stmt.query_map([], Playlist::from_row)?;
        rows.map(|r| r.map_err(Into::into)).collect()
    }

    pub fn rename_playlist(&self, playlist_id: i64, new_name: &str) -> Result<()> {
        let conn = self.lock();
        conn.execute(
            "UPDATE playlists SET name = ?1 WHERE id = ?2",
            params![new_name, playlist_id],
        )?;
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Playlist track manipulation
    // -----------------------------------------------------------------------

    pub fn add_track_to_playlist(&self, playlist_id: i64, track_id: i64) -> Result<()> {
        let conn = self.lock();

        // Find the next available position.
        let next_pos: i64 = conn.query_row(
            "SELECT COALESCE(MAX(position), -1) + 1 FROM playlist_tracks WHERE playlist_id = ?1",
            params![playlist_id],
            |row| row.get(0),
        )?;

        conn.execute(
            "INSERT INTO playlist_tracks (playlist_id, track_id, position) VALUES (?1, ?2, ?3)",
            params![playlist_id, track_id, next_pos],
        )?;
        Ok(())
    }

    pub fn remove_track_from_playlist(
        &self,
        playlist_id: i64,
        track_id: i64,
        position: i64,
    ) -> Result<()> {
        let conn = self.lock();

        // Remove the specific occurrence at that position.
        conn.execute(
            "DELETE FROM playlist_tracks WHERE playlist_id = ?1 AND track_id = ?2 AND position = ?3",
            params![playlist_id, track_id, position],
        )?;

        // Close the gap.
        conn.execute(
            "UPDATE playlist_tracks SET position = position - 1
             WHERE playlist_id = ?1 AND position > ?2",
            params![playlist_id, position],
        )?;

        Ok(())
    }

    pub fn reorder_track_in_playlist(
        &self,
        playlist_id: i64,
        old_index: i64,
        new_index: i64,
    ) -> Result<()> {
        if old_index == new_index {
            return Ok(());
        }
        let conn = self.lock();

        // Park the moving track out of range so shift operations don't collide.
        conn.execute(
            "UPDATE playlist_tracks SET position = -1
             WHERE playlist_id = ?1 AND position = ?2",
            params![playlist_id, old_index],
        )?;

        if old_index < new_index {
            conn.execute(
                "UPDATE playlist_tracks SET position = position - 1
                 WHERE playlist_id = ?1 AND position > ?2 AND position <= ?3",
                params![playlist_id, old_index, new_index],
            )?;
        } else {
            conn.execute(
                "UPDATE playlist_tracks SET position = position + 1
                 WHERE playlist_id = ?1 AND position >= ?2 AND position < ?3",
                params![playlist_id, new_index, old_index],
            )?;
        }

        // Drop the parked track into its final spot.
        conn.execute(
            "UPDATE playlist_tracks SET position = ?1
             WHERE playlist_id = ?2 AND position = -1",
            params![new_index, playlist_id],
        )?;

        Ok(())
    }

    pub fn get_tracks_in_playlist(&self, playlist_id: i64) -> Result<Vec<FullTrack>> {
        let conn = self.lock();
        let sql = "
            SELECT t.*, pt.position, r.name AS artist_name, a.title AS album_title, a.album_art
            FROM tracks t
            JOIN playlist_tracks pt ON t.id = pt.track_id
            JOIN artists r ON t.artist_id = r.id
            JOIN albums  a ON t.album_id  = a.id
            WHERE pt.playlist_id = ?1
            ORDER BY pt.position ASC
        ";
        let mut stmt = conn.prepare(sql)?;
        let rows = stmt.query_map(params![playlist_id], FullTrack::from_row)?;
        rows.map(|r| r.map_err(Into::into)).collect()
    }
}

// ---------------------------------------------------------------------------
// Private helpers (don't need &self, just a borrow of the locked connection)
// ---------------------------------------------------------------------------

fn upsert_artist(conn: &Connection, name: &str, genre: Option<&str>) -> Result<i64> {
    conn.execute(
        "INSERT INTO artists (name, genre) VALUES (?1, ?2)
         ON CONFLICT(name) DO NOTHING",
        params![name, genre],
    )?;
    let id: i64 = conn.query_row(
        "SELECT id FROM artists WHERE name = ?1",
        params![name],
        |row| row.get(0),
    )?;
    Ok(id)
}

fn upsert_album(
    conn: &Connection,
    artist_id: i64,
    title: &str,
    year: Option<i64>,
    genre: Option<&str>,
    album_art: Option<&str>,
) -> Result<i64> {
    conn.execute(
        "INSERT INTO albums (artist_id, title, year, genre, album_art)
         VALUES (?1, ?2, ?3, ?4, ?5)
         ON CONFLICT(artist_id, title) DO UPDATE SET
             album_art = COALESCE(albums.album_art, excluded.album_art)",
        params![artist_id, title, year.unwrap_or(0), genre, album_art],
    )?;
    let id: i64 = conn.query_row(
        "SELECT id FROM albums WHERE artist_id = ?1 AND title = ?2",
        params![artist_id, title],
        |row| row.get(0),
    )?;
    Ok(id)
}
