use std::{fs::{File, self}, path::PathBuf, process::{Command, Stdio}, os::unix::process::CommandExt, sync::mpsc, thread::{self, sleep}, io::{self, BufRead, BufReader}, time::Duration};

use crate::{minecraft::{minecraft_start_parameter::MinecraftJavaStartParameters, minecraft_assets::Libraries}, global_path};

pub fn start(java_start_parameters: MinecraftJavaStartParameters) -> Result<(), Box<dyn std::error::Error>> {

    copy_native_libraries(java_start_parameters.natives_files, &java_start_parameters.natives_dir_path);

    // 以免發生 cwd ENOENT error
    fs::create_dir_all(global_path::get_instances_dir_path().join("mckismetlab-main-server"));

    // let child = Command::new("ping")
    //     .arg("google.com")
    //     .stdout(Stdio::piped())
    //     .spawn()
    //     .expect("Failed to start ping process");

    // thread::spawn(move || {
    //     let mut f = BufReader::new(child.stdout.unwrap());
    //     loop {
    //         let mut buf = String::new();
    //         match f.read_line(&mut buf) {
    //             Ok(_) => {
    //                 println!("{}", buf.as_str())
    //             }
    //             Err(e) => println!("an error!: {:?}", e),
    //         }
    //     }
    // });

    println!("{:?}", java_start_parameters.java_vm_path);
    println!("{:?}", java_start_parameters.natives_dir_path);
    println!("{:?}", java_start_parameters.parameters.join(" "));

    // let child = Command::new(java_start_parameters.java_vm_path)
    //     .arg(java_start_parameters.parameters.join(" "))
    //     .current_dir(global_path::get_instances_dir_path().join("mckismetlab-main-server"))
    //     .stdout(Stdio::piped())
    //     .stderr(Stdio::piped())
    //     .spawn()
    //     .expect("Failed to start ping process");

    // thread::spawn(move || {
    //     let mut f = BufReader::new(child.stdout.unwrap());
    //     loop {
    //         let mut buf = String::new();
    //         match f.read_line(&mut buf) {
    //             Ok(_) => {
    //                 println!("{}", buf.as_str())
    //             }
    //             Err(e) => println!("an error!: {:?}", e),
    //         }
    //     }
    // });

    // sleep(Duration::from_secs(300));

    return Ok(());
}

fn copy_native_libraries(natives_files: Vec<Libraries>, natives_dir_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {

    for native_lib in natives_files {
        let zip_file = File::open(native_lib.file_path)?;
        let mut archive = zip::ZipArchive::new(zip_file)?;
        fs::create_dir_all(natives_dir_path.parent().unwrap());
        archive.extract(natives_dir_path);
    }

    println!("Extraction natives completed.");

    return Ok(());
}