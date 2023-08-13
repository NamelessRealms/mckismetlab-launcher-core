use std::path::{PathBuf, Path};
use uuid::Uuid;

use crate::{global_path, utils};

use super::minecraft_assets::{MinecraftAssets, Libraries, LibrariesType};

#[derive(Debug)]
pub struct MinecraftJavaStartParameters {
    pub java_vm_path: PathBuf,
    pub natives_dir_path: PathBuf,
    pub natives_files: Vec<Libraries>,
    pub parameters: Vec<String>
}

pub fn get_minecraft_java_start_parameters(minecraft_assets: MinecraftAssets) -> MinecraftJavaStartParameters {
    
    let java_buildStart_parameters = JavaBuildStartParameters {
        minecraft_assets: minecraft_assets,
        natives_dir_path: global_path::get_common_dir_path().join("bin").join(Uuid::new_v4().to_string())
    };
    
    let parameters = java_buildStart_parameters.get_build_113above();

    MinecraftJavaStartParameters {
        java_vm_path: Path::new("/usr/lib/jvm/java-17-openjdk-amd64/bin/java").to_path_buf(),
        natives_dir_path: java_buildStart_parameters.natives_dir_path,
        natives_files: get_natives_file_paths(java_buildStart_parameters.minecraft_assets.libraries),
        parameters: parameters
    }
}

fn get_natives_file_paths(libraries: Vec<Libraries>) -> Vec<Libraries> {
    let mut natives_libraries: Vec<Libraries> = Vec::<Libraries>::new();
    for lib in libraries {
        if !matches!(lib.r#type, LibrariesType::Natives) { continue; };
        natives_libraries.push(lib);
    }
    return natives_libraries;
}

#[derive(Debug)]
struct JavaBuildStartParameters {
    minecraft_assets: MinecraftAssets,
    natives_dir_path: PathBuf
}

impl JavaBuildStartParameters {

    // Get Minecraft Java 1.13 above
    fn get_build_113above(&self) -> Vec<String> {
        let mut parameters = Vec::new();

        // jvm
        parameters.extend(self.get_jvm_113later());

        // parameter
        parameters.extend(self.jvm_parameters());

        parameters.push(self.minecraft_assets.main_class.clone());

        parameters.extend(self.minecraft_arguments_113later());

        return parameters;
    }

    fn minecraft_arguments_113later(&self) -> Vec<String> {

        // println!("{:?}", self.minecraft_assets.arguments.game.arguments_rules);

        let game: &crate::minecraft::minecraft_assets::ArgumentGame = &self.minecraft_assets.arguments.game;
        let mut game_arguments = Vec::<String>::new();

        for arg in &game.arguments {
            let val = match arg.value.as_str() {
                "${auth_player_name}" => format!("{}={}", arg.argument, "Yu_Cheng"),
                "${version_name}" => format!("{}={}", arg.argument, self.minecraft_assets.minecraft_version),
                "${game_directory}" => format!("{}={}", arg.argument, global_path::get_instances_dir_path().join("mckismetlab-main-server").to_string_lossy().to_string()),
                "${assets_root}" => format!("{}={}", arg.argument, global_path::get_common_dir_path().join("assets").to_string_lossy().to_string()),
                "${assets_index_name}" => format!("{}={}", arg.argument, self.minecraft_assets.assets_version),
                "${auth_uuid}" => format!("{}={}", arg.argument, "93ea0589-ec75-4cad-8619-995164382e8d"),
                "${auth_access_token}" => format!("{}={}", arg.argument, "null_token"),
                "${user_type}" => format!("{}={}", arg.argument, "mojang"),
                "${version_type}" => format!("{}={}", arg.argument, "release"),
                _ => continue,
            };
            game_arguments.push(val);
        }

        return game_arguments;
    }

    fn jvm_parameters(&self) -> Vec<String> {

        let mut arguments = Vec::<String>::new();

        let ram_size_max = 4096;
        let ram_size_min = 1024;
        
        if ram_size_max != 0 {
            arguments.push(format!("-Xmx{}M", ram_size_max));
        } else {
            arguments.push("-Xmx2048M".to_string());
        }

        if ram_size_min != 0 {
            arguments.push(format!("-Xms{}M", ram_size_min));
        } else {
            arguments.push("-Xms1024M".to_string());
        }

        return arguments;
    }

    fn get_jvm_113later(&self) -> Vec<String> {


        let jvm = &self.minecraft_assets.arguments.jvm;
        let mut jvm_arguments = Vec::<String>::new();

        // println!("{:?}", jvm.arguments_rules);

        for arg in &jvm.arguments_rules {
            
            for rule in &arg.rules {

                if rule.action != "allow" { continue; };
                
                let os_name = &rule.os.name;
                let os_arch = &rule.os.arch;

                if os_name.is_none() || os_name.clone().unwrap() != self.get_os_type() { continue; };

                for val in &arg.value {
                    jvm_arguments.push(val.clone());
                }

                if os_arch.is_none() || os_arch.clone().unwrap() != std::env::consts::ARCH { continue; };
            
                for val in &arg.value {
                    jvm_arguments.push(val.clone());
                }
            }

        }

        for arg in &jvm.arguments {
            let val = match arg.value.as_str() {
                "${natives_directory}" => format!("{}={}", arg.argument, self.natives_dir_path.to_str().unwrap()),
                "${launcher_name}" => format!("{}={}", arg.argument, "mcKismetLab"),
                "${launcher_version}" => format!("{}={}", arg.argument, "v0.5.0"),
                "${classpath}" => format!("{} {}", arg.argument, self.combination_library_path()),
                _ => continue,
            };
            jvm_arguments.push(val);
        }

        // println!("{:?}", jvm_arguments);

        return jvm_arguments;
    }

    fn combination_library_path(&self) -> String {

        let mut libraries: Vec<String>;

        libraries = self.minecraft_assets.libraries.iter().map(|item| item.file_path.to_string_lossy().to_string()).collect();
        libraries.push(self.minecraft_assets.client.file_path.to_string_lossy().to_string());

        if utils::get_os_type() == "win32" {
            return libraries.join(";");
        } else {
            return libraries.join(":");
        }
    }

    fn get_os_type(&self) -> String {
        match utils::get_os_type() {
            "win32" => "windows".to_string(),
            "linux" => "linux".to_string(),
            "darwin" => "osx".to_string(),
            _ => "unknown".to_string()
        }
    }
}