use lofty::{prelude::*};
use lofty::picture::PictureType;
use lofty::probe::Probe;
use walkdir::WalkDir;

use std::collections::{HashSet, VecDeque};
use std::path::{PathBuf};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use crate::constants::cover_cache;
use crate::{constants};
use crate::library_service::library_service;

#[tauri::command]
#[specta::specta]
pub async fn index() {

    println!("Begin Indexing");

    let files = get_all_audio_files();
    for file in files {

        let service = library_service();

        let guard = match service.lock() {
            Ok(g) => {
                g
            }
            Err(e) => {
                eprintln!("Mutex poisoned: {:?}", e);
                return; // or handle properly
            }
        };

        let path_str = match file.to_str() {
            Some(s) => {
                s
            }
            None => {
                eprintln!("Path is not valid UTF-8");
                return;
            }
        };

        let result = match guard.is_track_in_library(path_str) {
            Ok(v) => {
                v
            }
            Err(e) => {
                eprintln!("Library check failed: {:?}", e);
                return;
            }
        };

        if !result {
            
            //Extract Tags
            let tagged_file = Probe::open(file.clone())
                .expect("ERROR: Bad Path provided")
                .read()
                .expect("ERROR: Failed to read file");

            let tag = match tagged_file.primary_tag() {
                Some(primary_tag) => primary_tag,
                None => tagged_file.first_tag().expect("ERROR: No tags found")
            };

            let props = tagged_file.properties();

            let parse_tag = |key: ItemKey| {
                tag.get_string(key)
                    .and_then(|s| s.parse::<i64>().ok())
            };

            let year = tag.get_string(ItemKey::RecordingDate)
            .or_else(|| tag.get_string(ItemKey::Year))
            .and_then(|date_str| {
                date_str.chars().take(4).collect::<String>().parse::<i64>().ok()
            });
            let track_num = parse_tag(ItemKey::TrackNumber);
            let disc_num = parse_tag(ItemKey::DiscNumber);
            let bpm = parse_tag(ItemKey::Bpm);

            let result = guard.add_track_with_metadata(
                file.to_str().unwrap(),
                tag.title().as_deref(), 
                tag.artist().as_deref(), 
                tag.get_string(ItemKey::AlbumArtist), 
                tag.album().as_deref(), 
                None, 
                tag.genre().as_deref(), 
                Some(props.duration().as_secs() as i64), 
                year, 
                track_num, 
                disc_num, 
                bpm, 
                tag.get_string(ItemKey::InitialKey), 
                tag.get_string(ItemKey::Isrc), 
                tag.get_string(ItemKey::Lyrics), 
                tag.get_string(ItemKey::Composer)
            );

            match result {
                Ok(()) => {},
                Err(e) => {
                    eprint!("{}", e);
                    continue;
                }
            };

            //Generate Image Cache
            let picture = tag
                .pictures()
                .iter()
                .find(|p| p.pic_type() == PictureType::CoverFront)
                .or_else(|| tag.pictures().first())
                .ok_or("No embedded pictures found");

            match picture {
                Ok(pic) => {
                    let album_id = guard.get_album_id_by_path(file.to_str().unwrap());

                    match album_id {
                        
                        Ok(id) => {
                            let mime_type = pic.mime_type();

                            if let Some(mime) = mime_type {
                                let ext = mime.as_str().split('/').last().unwrap_or("jpg");
                                let output_path = cover_cache().join(format!("{}.{}", id.unwrap(), ext));
                                let file = File::create(output_path);
                                
                                match file {
                                    Ok(mut file) => {
                                        let res = file.write_all(pic.data());
                                        if res.is_err() {
                                            eprintln!("Failed to write image: {}", res.err().unwrap())
                                        }
                                    },

                                    Err(e) => {
                                        eprintln!("{}", e);
                                        continue;
                                    }
                                }
                            }
                            
                        },

                        Err(e) => {
                            eprintln!("{}", e);
                            continue;
                        }
                    }
                },

                Err(e) => {
                    eprintln!("{}", e);
                    continue;
                }
            }
        }
    }

    println!("Done indexing");
}

fn get_all_audio_files() -> VecDeque<PathBuf> {
    let mut files = VecDeque::<PathBuf>::new();
    let audio_extensions = ["mp3", "wav", "flac", "ogg", "m4a", "aac", "wma", "opus"];
    let ext_set: HashSet<&str> = audio_extensions.iter().cloned().collect();
    let dirs_to_process = get_directories();
    
    for dir in dirs_to_process {
        for entry in WalkDir::new(dir)
            .into_iter()
            .filter_map(|e| e.ok()) 
        {
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
    let mut directories = VecDeque::<PathBuf>::new();
    
    let file = File::open(constants::dir_file());
    let reader = BufReader::new(file.unwrap());

    for line in reader.lines() {
        match line {
            Ok(line) => {
                let path = PathBuf::from(line);
                if path.exists() {
                    directories.push_back(path);
                }
            },
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }

    return directories;
}