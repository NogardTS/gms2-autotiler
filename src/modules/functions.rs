use crate::modules::constants::*;
use crate::modules::structs::*;

use std::error::Error;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::Path;
use std::string::ToString;

pub fn load_last_files() -> Result<LastFiles, Box<dyn Error>> {
    let mut f = File::open(&LAST_FILES)?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;
    serde_json::from_str::<LastFiles>(&buffer).map_err(|e| e.into())
}

pub fn save_last_files(last_files: &LastFiles) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(&LAST_FILES)?;
    file.write_all(serde_json::to_string(&last_files)?.as_bytes())?;
    Ok(())
}

pub fn backup_room(room_path: &str) -> Result<String, Box<dyn Error>> {
    let path = Path::new(room_path);
    let stem = path.file_stem().ok_or("The room file should have a .yy extension")?.to_str().ok_or("Invalid file name")?.to_string();
    fs::create_dir_all(&BACKUP_FOLDER)?;
    let now = chrono::offset::Utc::now().format("%Y-%m-%d-%H-%M-%S").to_string();
    let backup_room_path = BACKUP_FOLDER.to_string() + "\\" + &stem + "-" + &now + ".yy";
    println!("{}", room_path);
    println!("{}", backup_room_path);
    
    fs::copy(&room_path, &backup_room_path)?;  // Copy foo.txt to bar.txt
    Ok(backup_room_path)
}

pub fn get_room(room_path: &str) -> Result<Room, Box<dyn Error>> {
    let mut f = File::open(&room_path)?;
    let mut encoded = String::new();
    f.read_to_string(&mut encoded)?;
    serde_json::from_str(&encoded).map_err(|e| e.into())
}

pub fn get_tileset(tileset_path: &str) -> Result<Tileset, Box<dyn Error>> {
    let mut f = File::open(&tileset_path)?;
    let mut encoded = String::new();
    f.read_to_string(&mut encoded)?;
    serde_json::from_str(&encoded).map_err(|e| e.into())
}

pub fn get_block(block_path: &str) -> Result<Object, Box<dyn Error>> {
    let mut f = File::open(&block_path)?;
    let mut encoded = String::new();
    f.read_to_string(&mut encoded)?;
    serde_json::from_str(&encoded).map_err(|e| e.into())
}

pub fn save_room(room: &Room, room_path: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(&room_path)?;
    file.write_all(serde_json::to_string(&room)?.as_bytes())?;
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