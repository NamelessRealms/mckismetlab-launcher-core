// use mckismetlab_launcher_core::java;
// use mckismetlab_launcher_core::LauncherStore;
use mckismetlab_launcher_core::minecraft::minecraft_assets;

fn main() {
    // java::search_java_paths();
    // LauncherStore::init();

    minecraft_assets::assets("1.20.1");
}