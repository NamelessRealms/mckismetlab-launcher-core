use std::path::{Path, PathBuf};

use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::{global_path, utils};

// version manifests struct
#[derive(Debug, Deserialize)]
struct VersionManifest {
    latest: LatestVersions,
    versions: Vec<VersionInfo>,
}

#[derive(Debug, Deserialize)]
struct LatestVersions {
    release: String,
    snapshot: String,
}

#[derive(Debug, Deserialize)]
struct VersionInfo {
    id: String,
    #[serde(rename = "url")]
    version_url: String,
}

#[derive(Debug, Deserialize)]
struct ArgumentsGameRulesValues {
    action: String,
    features: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct ArgumentsGameRules {
    rules: Vec<ArgumentsGameRulesValues>,
    value: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Argument {
    name: String,
    value: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ArgumentGame {
    arguments: Vec<Argument>,
    arguments_rules: Vec<ArgumentsGameRules>,
}

#[derive(Debug, Deserialize)]
struct Arguments {
    game: ArgumentGame,
    jvm: ArgumentJvm,
}

#[derive(Debug, Deserialize)]
struct ArgumentsJvmRulesOSValues {
    name: Option<String>,
    arch: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ArgumentsJvmRulesValues {
    action: String,
    os: ArgumentsJvmRulesOSValues,
}

#[derive(Debug, Deserialize)]
struct ArgumentsJvmRules {
    rules: Vec<ArgumentsJvmRulesValues>,
    value: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct ArgumentJvm {
    arguments: Vec<Argument>,
    arguments_rules: Vec<ArgumentsJvmRules>,
}

#[derive(Debug, Deserialize)]
struct AssetIndex {
    id: String,
    sha1: String,
    size: i32,
    #[serde(rename = "totalSize")]
    total_size: i32,
    url: String
}

#[derive(Debug, Deserialize)]
struct AssetObjects {
    file_name: String,
    file_path: PathBuf,
    sha1: String,
    size: i32,
    download_url: String
}

#[derive(Debug, Deserialize)]
struct AssetObjectsObjectsData {
    hash: String,
    size: i32,
}

#[derive(Debug, Deserialize)]
struct AssetObjectsObjects {
    objects: std::collections::HashMap<String, AssetObjectsObjectsData>,
}

#[derive(Debug, Deserialize)]
struct LibrariesArtifact {
    path: String,
    sha1: String,
    size: i32,
    url: String
}

// #[derive(Debug, Deserialize)]
// struct LibrariesRulesOS {
//     name: String
// }

// #[derive(Debug, Deserialize)]
// struct LibrariesRules {
//     action: String,
//     os: LibrariesRulesOS
// }

#[derive(Debug, Deserialize)]
enum LibrariesType {
    Artifact,
    Natives
}

#[derive(Debug, Deserialize)]
struct Libraries {
    r#type: LibrariesType,
    file_name: String,
    file_path: PathBuf,
    sha1: String,
    size: i32,
    url: String
}

#[derive(Debug, Deserialize)]
struct ManifestClient {
    sha1: String,
    size: i32,
    url: String
}

#[derive(Debug, Deserialize)]
struct Client {
    file_name: String,
    file_path: PathBuf,
    sha1: String,
    size: i32,
    url: String
}

#[derive(Debug, Deserialize)]
pub struct MinecraftAssets {
    arguments: Arguments,
    assets_objects: Vec<AssetObjects>,
    libraries: Vec<Libraries>,
    client: Client,
    main_class: String
}

pub fn assets(version: &str) -> MinecraftAssets {
    let manifest_result = get_minecraft_manifest(version);

    let manifest = match manifest_result {
        Ok(value) => value,
        Err(error) => panic!("{}", error),
    };

    let arguments = get_arguments(manifest.get("arguments").unwrap().clone());
    let assets_objects = get_assets_objects(serde_json::from_value(manifest.get("assetIndex").unwrap().clone()).unwrap());
    let libraries = get_libraries(manifest.get("libraries").unwrap().as_array().unwrap());
    let client = get_client(serde_json::from_value::<ManifestClient>(manifest.get("downloads").unwrap().get("client").unwrap().clone()).unwrap(), &version);

    MinecraftAssets {
        arguments: arguments,
        assets_objects: assets_objects.unwrap(),
        libraries: libraries,
        client: client,
        main_class: manifest.get("mainClass").unwrap().to_string()
    }
}

fn get_client(manifest_client: ManifestClient, version: &str) -> Client {
    return Client {
        file_name: format!("{}.jar", version),
        file_path: Path::new(&global_path::get_common_dir_path()).join("versions").join(version).join(format!("{}.jar", version)),
        sha1: manifest_client.sha1,
        size: manifest_client.size,
        url: manifest_client.url
    };
}

fn get_libraries(manifest_libraries: &Vec<serde_json::Value>) -> Vec<Libraries> {

    let libraries_path = Path::new(&global_path::get_common_dir_path()).join("libraries");
    let mut libraries = Vec::<Libraries>::new();

    for lib in manifest_libraries {

        let rules = match lib.get("rules") {
            None => None,
            Some(value) => Some(serde_json::from_value::<Vec<serde_json::Value>>(value.clone()).unwrap())
        };

        let natives = match lib.get("natives") {
            None => None,
            Some(value) => Some(serde_json::from_value::<serde_json::Value>(value.clone()).unwrap())
        };

        if is_lib_rules(rules, &natives) {
            
            let downloads = lib.get("downloads").unwrap();

            if lib.get("natives").is_none() {

                let artifact = serde_json::from_value::<LibrariesArtifact>(downloads.get("artifact").unwrap().clone()).unwrap();

                libraries.push(Libraries {
                    r#type: LibrariesType::Artifact,
                    file_name: artifact.path.split("/").collect::<Vec<&str>>().last().unwrap().to_string(),
                    file_path: libraries_path.join(artifact.path),
                    sha1: artifact.sha1,
                    size: artifact.size,
                    url: artifact.url
                });

            } else {
                let system_type = match utils::get_os_type() {
                    "win32" => get_natives_sys_obj_value(&natives, "windows"),
                    "linux" => get_natives_sys_obj_value(&natives, "linux"),
                    "darwin" => get_natives_sys_obj_value(&natives, "osx"),
                    _ => None
                };
                let classifiers = serde_json::from_value::<LibrariesArtifact>(downloads.get("classifiers").unwrap().get(system_type.unwrap()).unwrap().clone()).unwrap();

                libraries.push(Libraries {
                    r#type: LibrariesType::Natives,
                    file_name: classifiers.path.split("/").collect::<Vec<&str>>().last().unwrap().to_string(),
                    file_path: libraries_path.join(classifiers.path),
                    sha1: classifiers.sha1,
                    size: classifiers.size,
                    url: classifiers.url
                });
            }
        }
    };

    return libraries;
}

fn is_lib_rules(rules: Option<Vec<serde_json::Value>>, natives: &Option<serde_json::Value>) -> bool {

    if rules.is_none() {
        if natives.is_some(){
            let system_value = match utils::get_os_type() {
                "win32" => get_natives_sys_obj_value(&natives, "windows"),
                "linux" => get_natives_sys_obj_value(&natives, "linux"),
                "darwin" => get_natives_sys_obj_value(&natives, "osx"),
                _ => None
            };
            println!("{:#?}", system_value);
            return system_value.is_some();
        }
        return true;
    }

    for rule in rules.unwrap() {
        let action = rule.get("action");
        let os_prop = rule.get("os");

        if action.is_some() && os_prop.is_some() {
            let os_name = os_prop.unwrap().get("name").unwrap().to_string();
            return match action.unwrap().as_str() {
                Some("allow") => os_name == get_os_type(),
                Some("disallow") => os_name != get_os_type(),
                _ => false
            };
            // if action.unwrap().to_string() == "allow" {
            //     println!("allow return: {}", os_name == get_os_type());
            //     return os_name == get_os_type();
            // } else if action.unwrap().to_string() == "disallow".to_string() {
            //     println!("disallow return: {}", os_name != get_os_type());
            //     return os_name != get_os_type();
            // }
        }
    }

    return true;
}

fn get_os_type() -> String {
    match utils::get_os_type() {
        "win32" => "windows".to_string(),
        "linux" => "linux".to_string(),
        "darwin" => "osx".to_string(),
        _ => "unknown".to_string()
    }
}

fn get_natives_sys_obj_value(natives: &Option<serde_json::Value>, system: &str) -> Option<String> {
    match natives.as_ref().unwrap().get(system) {
        None => None,
        Some(value) => Some(value.as_str().unwrap().to_string())
    }
}

#[tokio::main]
async fn get_assets_objects(manifest_asset_index: AssetIndex) -> Result<Vec<AssetObjects>, Box<dyn std::error::Error>> {
    
    let objects_json_value = reqwest::get(manifest_asset_index.url).await?.json::<serde_json::Value>().await?;
    let objects: AssetObjectsObjects = serde_json::from_value(objects_json_value).unwrap();

    let objects_path = Path::new(&global_path::get_common_dir_path()).join("assets").join("objects");

    let asset_objects: Vec<AssetObjects> = objects.objects.into_iter().map(|(_index, object)| {
        // substring directory
        let dir_name: String = object.hash.chars().take(2).collect();

        AssetObjects {
            file_name: object.hash.clone(),
            file_path: objects_path.join(&dir_name).join(&object.hash),
            sha1: object.hash.clone(),
            size: object.size,
            download_url: format!("https://resources.download.minecraft.net/{}/{}", dir_name, object.hash)
        }
    }).collect();

    Ok(asset_objects)
}

fn get_arguments(manifest_arguments: serde_json::Value) -> Arguments {
    let game_arguments = get_game_arguments(
        manifest_arguments
            .get("game")
            .unwrap()
            .as_array()
            .unwrap()
            .to_vec(),
    );

    let jvm_arguments = get_jvm_arguments(
        manifest_arguments
            .get("jvm")
            .unwrap()
            .as_array()
            .unwrap()
            .to_vec(),
    );

    Arguments {
        game: game_arguments,
        jvm: jvm_arguments,
    }
}

fn get_jvm_arguments(arguments_jvm: Vec<serde_json::Value>) -> ArgumentJvm {
    let mut arguments: Vec<Argument> = Vec::new();
    let mut arguments_rules: Vec<ArgumentsJvmRules> = Vec::new();

    for argument_jvm in arguments_jvm {
        let mut rules: Vec<ArgumentsJvmRulesValues> = Vec::new();
        let mut value: Vec<String> = Vec::new();

        // if !argument_jvm.is_object() && argument_jvm.is_string() {
        //     let argument_value = argument_jvm.as_str().unwrap().to_string();
        //     if Regex::new(r"=\$\{[^}]*\}")
        //         .unwrap()
        //         .is_match(&argument_value)
        //     {
        //         let value_split: Vec<&str> = argument_value.split("=${").collect();
        //         arguments.push(Argument {
        //             name: value_split.get(0).unwrap().to_string(),
        //             value: None,
        //         });
        //     } else if !Regex::new(r"\$\{[^}]*\}")
        //         .unwrap()
        //         .is_match(&argument_value)
        //     {
        //         arguments.push(Argument {
        //             name: argument_value,
        //             value: None,
        //         });
        //     }

        // Chat GPT ---------------------------------------------------------------
        if let Some(argument_value) = argument_jvm.as_str() {
            let argument_value = argument_value.to_string();
            if Regex::new(r"=\$\{[^}]*\}").unwrap().is_match(&argument_value) {
                let value_split: Vec<&str> = argument_value.split("=${").collect();
                arguments.push(Argument {
                    name: value_split.get(0).unwrap().to_string(),
                    value: None,
                });
            } else if !Regex::new(r"\$\{[^}]*\}").unwrap().is_match(&argument_value) {
                arguments.push(Argument {
                    name: argument_value,
                    value: None,
                });
            }
        // ------------------------------------------------------------------------
        } else {
            let argument_jvm_rules = argument_jvm.get("rules").unwrap().as_array().unwrap();
            let argument_jvm_vales = argument_jvm.get("value").unwrap();

            let os_vales = argument_jvm_rules.get(0).unwrap().get("os").map(|v| v.clone()).unwrap_or_default();

            rules.push(ArgumentsJvmRulesValues {
                action: argument_jvm_rules
                    .get(0)
                    .unwrap()
                    .get("action")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .to_string(),
                os: ArgumentsJvmRulesOSValues {
                    name: os_vales.get("name").and_then(|v| v.as_str()).map(|v| v.to_string()),
                    arch: os_vales.get("arch").and_then(|v| v.as_str()).map(|v| v.to_string()),
                },
            });

            if argument_jvm_vales.is_array() {
                for jvm_vales in argument_jvm_vales.as_array().unwrap() {
                    value.push(jvm_vales.as_str().unwrap().to_string());
                }
            } else {
                value.push(argument_jvm_vales.as_str().unwrap().to_string());
            }
        }

        if !rules.is_empty() && !value.is_empty() {
            arguments_rules.push(ArgumentsJvmRules { rules, value });
        }
    }

    ArgumentJvm {
        arguments,
        arguments_rules,
    }
}

fn get_game_arguments(arguments_game: Vec<serde_json::Value>) -> ArgumentGame {
    let mut arguments: Vec<Argument> = Vec::new();
    let mut arguments_rules: Vec<ArgumentsGameRules> = Vec::new();

    for argument_game in arguments_game {
        let mut rules: Vec<ArgumentsGameRulesValues> = Vec::new();
        let mut value: Vec<String> = Vec::new();

        // if !argument_game.is_object() && argument_game.is_string() {
        //     let argument_value = argument_game.as_str().unwrap().to_string();
        //     let re = Regex::new(r"\$\{[^}]*\}").unwrap();
        //     if !re.is_match(&argument_value) {
        //         arguments.push(Argument {
        //             name: argument_value,
        //             value: None,
        //         });
        //     }

        // Chat GPT ---------------------------------------------------------------
        if let Some(argument_value) = argument_game.as_str() {
            let re = Regex::new(r"\$\{[^}]*\}").unwrap();
            if !re.is_match(&argument_value) {
                arguments.push(Argument {
                    name: argument_value.to_string(),
                    value: None,
                });
            }
        // ------------------------------------------------------------------------
        } else {
            let argument_game_rules = argument_game.get("rules").unwrap().as_array().unwrap();
            let argument_game_value = argument_game.get("value").unwrap();

            rules.push(ArgumentsGameRulesValues {
                action: argument_game_rules
                    .get(0)
                    .unwrap()
                    .get("action")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .to_string(),
                features: argument_game_rules
                    .get(0)
                    .unwrap()
                    .get("features")
                    .unwrap()
                    .clone(),
            });

            if argument_game_value.is_array() {
                for game_value in argument_game_value.as_array().unwrap() {
                    value.push(game_value.as_str().unwrap().to_string());
                }
            } else {
                value.push(argument_game_value.as_str().unwrap().to_string());
            }
        }

        if !rules.is_empty() && !value.is_empty() {
            arguments_rules.push(ArgumentsGameRules { rules, value });
        }
    }

    ArgumentGame {
        arguments,
        arguments_rules,
    }
}

#[tokio::main]
async fn get_minecraft_manifest(
    version: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let version_manifest =
        reqwest::get("https://launchermeta.mojang.com/mc/game/version_manifest.json")
            .await?
            .json::<VersionManifest>()
            .await?;

    if let Some(manifest_url_data) = version_manifest.versions.iter().find(|v| v.id == version) {
        let url = &manifest_url_data.version_url;
        let manifest = reqwest::get(url).await?.json::<serde_json::Value>().await?;
        return Ok(manifest);
    }

    Err("Error: Version not found.".into())
}
