use std::path::Path;

pub type OSType = &'static str;

pub fn get_os_type() -> OSType {
    match std::env::consts::OS {
        "windows" => "win32",
        "macos" => "darwin",
        "linux" => "linux",
        _ => panic!("Unknown OS type"),
    }
}

pub fn is_path_exists(path: &Path) -> bool {
    match Path::try_exists(path) {
        Ok(value) => value,
        Err(_) => false
    }
}