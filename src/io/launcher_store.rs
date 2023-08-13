use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
struct IProfileData {
    microsoft_auth: MicrosoftAuth,
    minecraft_auth: MinecraftAuth,
    auth_type: String,
    user: User,
    player: Player,
    remember_status: bool,
    date: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct MicrosoftAuth {
    mc_account_token: String,
    access_token: String,
    refresh_token: String,
    expires_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct MinecraftAuth {
    access_token: String,
    client_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct User {
    username: String,
    id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Player {
    name: String,
    uuid: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ILauncherSettings {
    language: String,
    general: GeneralSettings,
    java: Vec<JavaSettings>,
    display_position: i32,
    launcher_keep_open: bool,
    selected_server_start: String,
    date: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct GeneralSettings {
    open_game_keep_launcher_state: bool,
    game_start_open_monitor_log: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct JavaSettings {
    server_name: String,
    java_path: String,
    ram_size_max: i32,
    ram_size_min: i32,
    java_parameter: String,
    is_built_in_java_vm: bool,
    ram_checked: Option<bool>,
    java_path_checked: Option<bool>,
    java_parameter_checked: Option<bool>,
}

pub struct LauncherStore {
    common_dir_path: String,
    profile_data_path: String,
    launcher_settings_path: String,
    profile_data: IProfileData,
    launcher_settings: ILauncherSettings,
}

impl LauncherStore {
    pub fn init() -> Self {
        let common_dir_path = "C:\\Users\\quasi\\Desktop".to_string();
        let profile_data_path = format!("{}/profileData.json", &common_dir_path);
        let launcher_settings_path = format!("{}/launcherSettings.json", &common_dir_path);

        if !Path::new(&common_dir_path).exists() {
            fs::create_dir_all(&common_dir_path).expect("無法創建通用目錄。");
        }

        if !Path::new(&profile_data_path).exists() {
            let profile_data = IProfileData {
                microsoft_auth: MicrosoftAuth {
                    mc_account_token: String::new(),
                    access_token: String::new(),
                    refresh_token: String::new(),
                    expires_at: String::new(),
                },
                minecraft_auth: MinecraftAuth {
                    access_token: String::new(),
                    client_token: String::new(),
                },
                auth_type: String::from("microsoft"),
                user: User {
                    username: String::new(),
                    id: String::new(),
                },
                player: Player {
                    name: String::new(),
                    uuid: String::new(),
                },
                remember_status: true,
                date: String::new(),
            };

            let profile_data_json = serde_json::to_string_pretty(&profile_data).unwrap();

            let mut file = File::create(&profile_data_path).unwrap();
            file.write_all(profile_data_json.as_bytes()).unwrap();
        }

        let profile_data_content =
            fs::read_to_string(&profile_data_path).expect("無法讀取個人檔案資料檔案。");
        let profile_data: IProfileData =
            serde_json::from_str(&profile_data_content).expect("無法解析個人檔案資料檔案。");

        if !Path::new(&launcher_settings_path).exists() {
            let launcher_settings = ILauncherSettings {
                language: String::from("zh_TW"),
                general: GeneralSettings {
                    open_game_keep_launcher_state: true,
                    game_start_open_monitor_log: false,
                },
                java: vec![
                    JavaSettings {
                        server_name: String::from("global"),
                        java_path: String::new(),
                        ram_size_max: 1024,
                        ram_size_min: 1024,
                        java_parameter: String::new(),
                        is_built_in_java_vm: true,
                        ram_checked: None,
                        java_path_checked: None,
                        java_parameter_checked: None,
                    },
                    JavaSettings {
                        server_name: String::from("mckismetlab-main-server"),
                        java_path: String::new(),
                        ram_size_max: 2048,
                        ram_size_min: 2048,
                        java_parameter: String::new(),
                        is_built_in_java_vm: true,
                        ram_checked: Some(false),
                        java_path_checked: Some(false),
                        java_parameter_checked: Some(false),
                    },
                    JavaSettings {
                        server_name: String::from("mckismetlab-deputy-server"),
                        java_path: String::new(),
                        ram_size_max: 2048,
                        ram_size_min: 2048,
                        java_parameter: String::new(),
                        is_built_in_java_vm: true,
                        ram_checked: Some(false),
                        java_path_checked: Some(false),
                        java_parameter_checked: Some(false),
                    },
                    JavaSettings {
                        server_name: String::from("mckismetlab-test-server"),
                        java_path: String::new(),
                        ram_size_max: 2048,
                        ram_size_min: 2048,
                        java_parameter: String::new(),
                        is_built_in_java_vm: true,
                        ram_checked: Some(false),
                        java_path_checked: Some(false),
                        java_parameter_checked: Some(false),
                    },
                ],
                display_position: 0,
                launcher_keep_open: true,
                selected_server_start: String::from("mckismetlab-main-server"),
                date: String::new(),
            };

            let launcher_settings_json = serde_json::to_string_pretty(&launcher_settings).unwrap();

            let mut file = File::create(&launcher_settings_path).unwrap();
            file.write_all(launcher_settings_json.as_bytes()).unwrap();
        }

        let launcher_settings_content =
            fs::read_to_string(&launcher_settings_path).expect("無法讀取啟動器設定檔案。");
        let launcher_settings: ILauncherSettings =
            serde_json::from_str(&launcher_settings_content).expect("無法解析啟動器設定檔案。");

        LauncherStore {
            common_dir_path,
            profile_data_path,
            launcher_settings_path,
            profile_data,
            launcher_settings,
        }
    }
}