use std::{fs::{File, self}, io::{Write, self}, path::PathBuf, cmp::min};
use futures::StreamExt;
use reqwest::Client;
use sha1::{Sha1, Digest};

use crate::{minecraft::minecraft_assets::{MinecraftAssets, MinecraftClient, AssetObjects, Libraries}, utils};

struct DownloadAsset {
    file_name: String,
    file_path: PathBuf,
    sha1: String,
    size: i32,
    url: String,
}

pub fn validate_installer(minecraft_assets: &MinecraftAssets) -> Result<(), Box<dyn std::error::Error>> {

    // validate minecraft client
    minecraft_client_install(&minecraft_assets.client)?;
    // validate minecraft assets
    minecraft_assets_install(&minecraft_assets.assets_objects)?;
    // validate libraries
    minecraft_libraries_install(&minecraft_assets.libraries)?;

    return Ok(());
}

fn minecraft_libraries_install(libraries: &Vec<Libraries>) -> Result<(), Box<dyn std::error::Error>> {
    let mut download_libraries: Vec<DownloadAsset> = Vec::<DownloadAsset>::new();
    for lib in libraries {
        download_libraries.push(DownloadAsset {
            file_name: lib.file_name.clone(),
            file_path: lib.file_path.clone(),
            sha1: lib.sha1.clone(),
            size: lib.size.clone(),
            url: lib.url.clone(),
        });
    }
    validate_download_assets(download_libraries, 5)?;
    return Ok(());
}

fn minecraft_assets_install(assets_objects: &Vec<AssetObjects>) -> Result<(), Box<dyn std::error::Error>> {
    let mut download_assets = Vec::<DownloadAsset>::new();
    for asset_object in assets_objects {
        download_assets.push(DownloadAsset {
            file_name: asset_object.file_name.clone(),
            file_path: asset_object.file_path.clone(),
            sha1: asset_object.sha1.clone(),
            size: asset_object.size.clone(),
            url: asset_object.url.clone(),
        });
    }
    validate_download_assets(download_assets, 5)?;
    return Ok(());
}

#[tokio::main]
async fn minecraft_client_install(client: &MinecraftClient) -> Result<(), Box<dyn std::error::Error>> {
    if !utils::is_path_exists(&client.file_path) || !sha1_exists(&client.file_path, &client.sha1)? {
        println!("Local file SHA-1 does not match, expected: {}", client.file_name);
        download_file(&client.url, &client.file_path, &client.sha1).await?;
        println!("Downloaded file to {:?}", client.file_path);
    } else {
        // println!("Local file SHA-1 matches: {}", &client.file_name);
    }
    return Ok(());
}

#[tokio::main]
async fn validate_download_assets(assets: Vec<DownloadAsset>, limit: usize) -> Result<(), Box<dyn std::error::Error>> {
    
    for (i, asset) in assets.iter().enumerate() {
        
        let file_name = &asset.file_name;
        let file_path = &asset.file_path;
        let download_url =& asset.url;
        let sha1 = &asset.sha1;

        if utils::is_path_exists(&file_path) {
           if sha1_exists(&file_path, sha1)? {
                // println!("Local file SHA-1 matches: {}", file_name);
                continue;
            } else {
                println!("Local file SHA-1 does not match, expected: {}", file_name);
            }
        }

        download_file(&download_url, file_path, sha1).await?;
        println!("Downloaded file finish! {:?}", file_path);
    }

    // let mut download_queue = Vec::new();

    // for (i, asset) in assets.iter().enumerate() {
    //     // println!("{} {}", i, asset.file_name);

    //     let file_path = asset.file_path.clone();
    //     let url = asset.url.clone();

    //     let download_task = tokio::spawn(async move {
    //         if !utils::is_path_exists(&file_path) {
    //             // download_file(&url, &file_path);
    //         }
    //     });

    //     download_queue.push(download_task);

    //     // 檢查限制並行下載數量的條件
    //     if download_queue.len() >= limit || i + 1 >= assets.len() {
    //         // 等待當前佇列中的所有下載任務完成
    //         futures::future::join_all(download_queue);
    //         download_queue.clear();
    //     }
    // }

    // 完成所有下載
    return Ok(());
}

pub async fn download_file(url: &str, path: &PathBuf, sha1: &String) -> Result<(), Box<dyn std::error::Error>> {
    
    let response = Client::new().get(url).send().await.or(Err(format!("Failed to GET from '{}'", &url)))?;

    // 檢查響應是否成功
    if !response.status().is_success() {
        return Err(format!("Failed to GET from '{}', status: {:?}", url, response.status()).into());
    }

    let total_size: u64 = response.content_length().ok_or(format!("Failed to get content length from '{}'", &url))?;

    if let Err(error) = fs::create_dir_all(path.parent().unwrap().join("..")) {
        return Err(error.into());
    }

    let mut file = File::create(path).or(Err(format!("Failed to create file '{}'", path.to_str().unwrap())))?;
    let mut downloaded: u64 = 0;
    let mut stream = response.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item.or(Err(format!("Error while downloading file")))?;
        file.write_all(&chunk).or(Err(format!("Error while writing to file")))?;
        let new = min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;

        // println!("File: {}, Downloaded Progress: {}/{}", path.parent().unwrap().to_str().unwrap().split("/").last().unwrap(), new, total_size);
        
        // let progress = format!(
        //     "File: {}, Downloaded Progress: {}/{}",
        //     path.file_name().unwrap().to_str().unwrap(),
        //     downloaded,
        //     total_size
        // );
        // print!("\r{:.<100}", progress);
    }

    // 檢查下载文件的 SHA-1 哈希值是否匹配
    if !sha1_exists(path, sha1)? {
        return Err(format!("Error while downloading file, SHA-1 hash does not match"))?;
    }

    return Ok(());
}

fn sha1_exists(path: &PathBuf, sha1: &String) -> Result<bool, Box<dyn std::error::Error>> {
    let mut local_file = fs::File::open(path)?;
    let mut hasher = Sha1::new();
    io::copy(&mut local_file, &mut hasher)?;
    let local_hash = format!("{:x}", hasher.finalize());
    return Ok(local_hash == sha1.to_string());
}