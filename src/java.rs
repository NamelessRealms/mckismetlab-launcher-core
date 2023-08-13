use std::fs;
use std::path::PathBuf;

use crate::utils::{get_os_type, OSType};

pub fn search_java_paths() -> Vec<String> {
    let os_type: OSType = get_os_type();
    let java_versions_dir = get_java_versions_dir(os_type);
    if java_versions_dir.is_none() {
        panic!("Unknown OS type: {}", os_type);
    }
    let java_versions_dir = java_versions_dir.unwrap();
    let versions = get_java_versions(&java_versions_dir);
    let mut paths: Vec<String> = Vec::new();
    for version in versions {
        let java_path = get_java_path(os_type, &version);
        if fs::metadata(&java_path).is_ok() {
            paths.push(java_path.clone());
        }
    }
    paths
}

fn get_java_versions_dir(os_type: OSType) -> Option<String> {
    match os_type {
        "win32" => Some("C:\\Program Files\\Java".to_owned()),
        "darwin" => Some("/Library/Java/JavaVirtualMachines".to_owned()),
        "linux" => Some("/usr/lib/jvm".to_owned()),
        _ => None,
    }
}

fn get_java_versions(java_versions_dir: &str) -> Vec<String> {
    if fs::metadata(java_versions_dir).is_err() {
        return Vec::new();
    }
    let files = fs::read_dir(java_versions_dir).unwrap();
    files
        .filter_map(|entry| {
            if let Ok(entry) = entry {
                let file_name = entry.file_name().to_string_lossy().to_string();
                if file_name.starts_with("jdk") || file_name.starts_with("jre") {
                    Some(file_name)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect()
}

fn get_java_path(os_type: OSType, version: &str) -> String {
    match os_type {
        "win32" => {
            let mut path_buf = PathBuf::new();
            path_buf.push("C:\\Program Files\\Java");
            path_buf.push(version);
            path_buf.push("bin");
            path_buf.push("java.exe");
            path_buf.to_string_lossy().to_string()
        }
        "darwin" => {
            let mut path_buf = PathBuf::new();
            path_buf.push("/Library/Java/JavaVirtualMachines");
            path_buf.push(format!("{}.jdk", version));
            path_buf.push("Contents");
            path_buf.push("Home");
            path_buf.push("bin");
            path_buf.push("java");
            path_buf.to_string_lossy().to_string()
        }
        "linux" => {
            let mut path_buf = PathBuf::new();
            path_buf.push("/usr/lib/jvm");
            path_buf.push(version);
            path_buf.push("bin");
            path_buf.push("java");
            path_buf.to_string_lossy().to_string()
        }
        _ => panic!("Unknown OS type"),
    }
}
