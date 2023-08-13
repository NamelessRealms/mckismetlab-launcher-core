// use mckismetlab_launcher_core::java;
// use mckismetlab_launcher_core::LauncherStore;
use mckismetlab_launcher_core::{minecraft::{minecraft_assets, minecraft_start_parameter}, assets_installer, game_process};

fn main() {
    // java::search_java_paths();
    // LauncherStore::init();

    let minecraft_assets = minecraft_assets::assets("1.16.5");
    assets_installer::validate_installer(&minecraft_assets);
    let minecraft_java_start_parameter = minecraft_start_parameter::get_minecraft_java_start_parameters(minecraft_assets);
    game_process::start(minecraft_java_start_parameter);
}