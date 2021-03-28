use crate::modules::constants::*;
use crate::modules::structs::*;

use anyhow::anyhow;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::Path;
use std::string::ToString;

pub fn load_last_files() -> anyhow::Result<LastFiles> {
    let mut f = File::open(&LAST_FILES)?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;
    let result = serde_jsonrc::from_str::<LastFiles>(&buffer).map_err(|e| e.into());
    println!("Loaded last files correctly");
    result
}

pub fn save_last_files(last_files: &LastFiles) -> anyhow::Result<()> {
    let mut file = File::create(&LAST_FILES)?;
    file.write_all(serde_jsonrc::to_string(&last_files)?.as_bytes())?;
    println!("Saved last files correctly");
    Ok(())
}

pub fn backup_room(room_path: &str) -> anyhow::Result<String> {
    let path = Path::new(room_path);
    let stem = path
        .file_stem()
        .ok_or(anyhow!("The room file should have a .yy extension"))?
        .to_str()
        .ok_or(anyhow!("Invalid file name"))?
        .to_string();
    fs::create_dir_all(&BACKUP_FOLDER)?;
    let now = chrono::offset::Utc::now()
        .format("%Y-%m-%d-%H-%M-%S")
        .to_string();
    let backup_room_path = BACKUP_FOLDER.to_string() + "\\" + &stem + "-" + &now + ".yy";
    println!("{}", room_path);
    println!("{}", backup_room_path);

    fs::copy(&room_path, &backup_room_path)?;
    println!("Backed up room correctly");
    Ok(backup_room_path)
}

pub fn get_room(room_path: &str) -> anyhow::Result<Room> {
    let mut f = File::open(&room_path)?;
    let mut encoded = String::new();
    f.read_to_string(&mut encoded)?;
    let result = serde_jsonrc::from_str(&encoded).map_err(|e| e.into());
    println!("Loaded room correctly");
    result
}

pub fn get_tileset(tileset_path: &str) -> anyhow::Result<TilesetId> {
    println!("{}", tileset_path);
    let path = Path::new(tileset_path);
    let stem = path
        .file_stem()
        .ok_or(anyhow!("The room file should have a .yy extension"))?
        .to_str()
        .ok_or(anyhow!("Invalid file name"))?
        .to_string();
    Ok(
        TilesetId {
            name: stem.clone(),
            path: format!("{}/{}/{}.yy", "tilesets", stem, stem),
            ..Default::default()
        }
    )
}

pub fn get_block(block_path: &str) -> anyhow::Result<String> {
    println!("{}", block_path);
    let path = Path::new(block_path);
    let stem = path
        .file_stem()
        .ok_or(anyhow!("The room file should have a .yy extension"))?
        .to_str()
        .ok_or(anyhow!("Invalid file name"))?
        .to_string();
    Ok(stem)
}

pub fn save_room(room: &Room, room_path: &str) -> anyhow::Result<()> {
    let mut file = File::create(&room_path)?;
    file.write_all(serde_jsonrc::to_string(&room)?.as_bytes())?;
    println!("Saved updated room correctly");
    Ok(())
}

pub fn pause() {
    use std::io::{self, prelude::*};
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();
    let _ = stdin.read(&mut [0u8]).unwrap();
}
