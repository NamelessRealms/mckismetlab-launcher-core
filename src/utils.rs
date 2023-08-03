pub type OSType = &'static str;

pub fn get_os_type() -> OSType {
    match std::env::consts::OS {
        "windows" => "win32",
        "macos" => "darwin",
        "linux" => "linux",
        _ => panic!("Unknown OS type"),
    }
}