use dirs;
use std::{fs, path::PathBuf};

pub fn ensure_paths_created() {
    _ = fs::create_dir_all(app_data());
    _ = fs::create_dir_all(app_cache());
    _ = fs::create_dir_all(cover_cache());
}

pub fn app_data() -> PathBuf {
    dirs::data_local_dir().unwrap().join("aurex")
}

pub fn app_cache() -> PathBuf {
    dirs::cache_dir().unwrap().join(".aurex").join("cache")
}

pub fn dir_file() -> PathBuf {
    app_data().join("directories.txt")
}

pub fn cover_cache() -> PathBuf {
    app_cache().join("covers")
}
