use crate::constants;
use crate::constants::cover_cache;
use crate::library_service::{library_service, LibraryService};
use crate::models::FileMetadata;
use lofty::picture::PictureType;
use lofty::prelude::*;
use lofty::probe::Probe;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Emitter};
use walkdir::WalkDir;

#[tauri::command]
#[specta::specta]
pub async fn index(app_handle: AppHandle) {
    index_tracks();
    delete_non_existant();
    index_playlists();

    _ = app_handle.emit("indexing-done", ());
}

pub fn delete_non_existant() {
    if let Ok(library) = library_service().lock() {
        if let Ok(tracks) = library.get_all_tracks() {
            for track in tracks {
                if !Path::new(&track.track.file_path).exists() {
                    _ = library.delete_track(track.track.id.unwrap());
                }
            }
        }
    }
}

pub fn index_tracks() {
    println!("Begin indexing tracks");

    let parsed: Vec<FileMetadata> = get_all_audio_files()
        .into_par_iter()
        .filter_map(parse_and_write_cover)
        .collect();

    let service = library_service();
    let guard = match service.lock() {
        Ok(g) => g,
        Err(e) => {
            eprintln!("Mutex poisoned: {:?}", e);
            return;
        }
    };

    for meta in parsed {
        index_file_to_db(&guard, meta);
    }

    println!("Done indexing tracks");
}

fn parse_and_write_cover(file: PathBuf) -> Option<FileMetadata> {
    let probe = Probe::open(file.clone()).ok()?;
    let tagged_file = probe.read().ok()?;

    let tag = match tagged_file.primary_tag() {
        Some(t) => t,
        None => tagged_file.first_tag()?,
    };

    let props = tagged_file.properties();
    let parse_tag = |key: ItemKey| tag.get_string(key).and_then(|s| s.parse::<i64>().ok());

    let year = tag
        .get_string(ItemKey::RecordingDate)
        .or_else(|| tag.get_string(ItemKey::Year))
        .and_then(|d| d.chars().take(4).collect::<String>().parse::<i64>().ok());

    // write cover to a temp path keyed by file hash/path, album ID not known yet
    // use a sanitised version of "artist - album" as filename for now,
    // then update_album_art in the serial phase with the real album ID
    let cover_path = tag
        .pictures()
        .iter()
        .find(|p| p.pic_type() == PictureType::CoverFront)
        .or_else(|| tag.pictures().first())
        .and_then(|pic| {
            let ext = pic
                .mime_type()
                .and_then(|m| m.as_str().split('/').last().map(str::to_owned))
                .unwrap_or_else(|| "jpg".to_owned());

            // temp filename: hash the file path so it's unique and deterministic
            let hash = {
                use std::collections::hash_map::DefaultHasher;
                use std::hash::{Hash, Hasher};
                let mut h = DefaultHasher::new();
                file.hash(&mut h);
                h.finish()
            };

            let out = cover_cache().join(format!("{}.{}", hash, ext));
            if out.exists() {
                return Some(out); // already written, skip
            }

            match File::create(&out) {
                Ok(mut f) => {
                    if let Err(e) = f.write_all(pic.data()) {
                        eprintln!("Failed to write cover: {}", e);
                        None
                    } else {
                        Some(out)
                    }
                }
                Err(e) => {
                    eprintln!("{}", e);
                    None
                }
            }
        });

    Some(FileMetadata {
        path: file,
        title: tag.title().map(|s| s.to_string()),
        artist: tag.artist().map(|s| s.to_string()),
        album_artist: tag.get_string(ItemKey::AlbumArtist).map(str::to_owned),
        album: tag.album().map(|s| s.to_string()),
        genre: tag.genre().map(|s| s.to_string()),
        duration: Some(props.duration().as_secs() as i64),
        year,
        track_num: parse_tag(ItemKey::TrackNumber),
        disc_num: parse_tag(ItemKey::DiscNumber),
        bpm: parse_tag(ItemKey::Bpm),
        initial_key: tag.get_string(ItemKey::InitialKey).map(str::to_owned),
        isrc: tag.get_string(ItemKey::Isrc).map(str::to_owned),
        lyrics: tag.get_string(ItemKey::Lyrics).map(str::to_owned),
        composer: tag.get_string(ItemKey::Composer).map(str::to_owned),
        cover_path,
    })
}

fn index_file_to_db(guard: &LibraryService, meta: FileMetadata) {
    let path_str = match meta.path.to_str() {
        Some(s) => s,
        None => {
            eprintln!("Path is not valid UTF-8");
            return;
        }
    };

    match guard.is_track_in_library(path_str) {
        Ok(true) => return,
        Ok(false) => {}
        Err(e) => {
            eprintln!("Library check failed: {:?}", e);
            return;
        }
    }

    if let Err(e) = guard.add_track_with_metadata(
        path_str,
        meta.title.as_deref(),
        meta.artist.as_deref(),
        meta.album_artist.as_deref(),
        meta.album.as_deref(),
        None,
        meta.genre.as_deref(),
        meta.duration,
        meta.year,
        meta.track_num,
        meta.disc_num,
        meta.bpm,
        meta.initial_key.as_deref(),
        meta.isrc.as_deref(),
        meta.lyrics.as_deref(),
        meta.composer.as_deref(),
    ) {
        eprintln!("{}", e);
        return;
    }

    // cover is already on disk, just need to rename it to the real album ID
    if let Some(temp_cover) = meta.cover_path {
        match guard.get_album_id_by_path(path_str) {
            Ok(Some(id)) => {
                let ext = temp_cover
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("jpg");
                let final_path = cover_cache().join(format!("{}.{}", id, ext));
                if let Err(e) = std::fs::rename(&temp_cover, &final_path) {
                    eprintln!("Failed to rename cover: {}", e);
                } else {
                    _ = guard.update_album_art(id, Some(final_path.to_str().unwrap()));
                }
            }
            Ok(None) => eprintln!("No album ID for {}", path_str),
            Err(e) => eprintln!("{}", e),
        }
    }
}

pub fn index_playlists() {
    println!("Begin indexing playlists");

    let files = get_m3u8_files();

    for file in files {
        let mut playlist_id: Option<i64> = None;

        if let Some(filename) = file.file_stem().and_then(|s| s.to_str()) {
            if let Ok(library) = library_service().lock() {
                if !library.playlist_exists(filename).unwrap() {
                    _ = library.create_playlist(filename, None);
                }

                if let Ok(result) = library.get_playlist_id_by_name(filename) {
                    playlist_id = result;
                }
            }
        }

        let audio_files = get_playlist_track_paths(file);

        for audio_file in audio_files {
            if let Ok(library) = library_service().lock() {
                if let Some(p_id) = playlist_id {
                    if !library
                        .is_track_in_playlist(p_id, audio_file.to_str().unwrap_or(""))
                        .unwrap_or(false)
                    {
                        if let Some(track_id) =
                            library.get_track_id_by_path(audio_file.to_str().unwrap())
                        {
                            _ = library.add_track_to_playlist(p_id, track_id);
                        }
                    }
                }
            }
        }
    }

    println!("End indexing playlists");
}

fn get_playlist_track_paths(m3u8_file: PathBuf) -> VecDeque<PathBuf> {
    let mut files: VecDeque<PathBuf> = VecDeque::new();
    let base_dir = m3u8_file.parent().unwrap_or(Path::new(""));

    if let Ok(file) = File::open(&m3u8_file) {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line) = line {
                if !line.is_empty() && !line.starts_with('#') {
                    let resolved = base_dir.join(&line);
                    if let Ok(canonical) = resolved.canonicalize() {
                        let path_str = canonical.to_string_lossy();
                        let clean = path_str.strip_prefix(r"\\?\").unwrap_or(&path_str);
                        files.push_back(PathBuf::from(clean));
                    }
                }
            }
        }
    }
    files
}

fn get_m3u8_files() -> VecDeque<PathBuf> {
    let mut files = VecDeque::<PathBuf>::new();
    let f_ext = "m3u8";
    let dirs_to_process = get_directories();

    for dir in dirs_to_process {
        for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                if let Some(ext) = entry.path().extension().and_then(|s| s.to_str()) {
                    if f_ext == ext {
                        files.push_back(entry.into_path());
                    }
                }
            }
        }
    }

    return files;
}

fn get_all_audio_files() -> VecDeque<PathBuf> {
    let mut files = VecDeque::<PathBuf>::new();
    let audio_extensions = ["mp3", "wav", "flac", "ogg", "m4a", "aac", "wma", "opus"];
    let ext_set: HashSet<&str> = audio_extensions.iter().cloned().collect();
    let dirs_to_process = get_directories();

    for dir in dirs_to_process {
        for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                if let Some(ext) = entry.path().extension().and_then(|s| s.to_str()) {
                    if ext_set.contains(ext) {
                        files.push_back(entry.into_path());
                    }
                }
            }
        }
    }

    return files;
}

fn get_directories() -> VecDeque<PathBuf> {
    if let Ok(library) = library_service().lock() {
        return library
            .get_directories()
            .unwrap_or_default()
            .into_iter()
            .filter(|p| p.exists())
            .collect();
    }
    VecDeque::new()
}
