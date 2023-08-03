use std::path::PathBuf;

use crate::utils;

fn get_game_data_dir_path() -> Option<PathBuf> {
    let home_dir = home::home_dir().unwrap();
    match utils::get_os_type() {
        "win32" => Some(home_dir.join("AppData").join("Roaming").join(".mckismetlab")),
        "darwin" => Some(home_dir.join("/Library/Application Support/mckismetlab")),
        _ => None,
    }
}

pub fn get_instances_dir_path() -> PathBuf {
    get_game_data_dir_path().unwrap().join("instances")
}

pub fn get_common_dir_path() -> PathBuf {
    get_game_data_dir_path().unwrap().join("common")
}