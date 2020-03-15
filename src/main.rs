#[macro_use] extern crate lazy_static;

use tinyfiledialogs::*;

mod modules;

use crate::modules::functions::*;
use crate::modules::structs::*;
use crate::modules::add_tileset_to_room::*;

use std::error::Error;

fn main() {
    let do_the_job = || -> Result<(), Box<dyn Error>> {
        let mut files = load_last_files().unwrap_or(LastFiles::default());
        files.room_path = open_file_dialog("Select the room you want to add a tileset to", &files.room_path, None).ok_or("No room file has been selected")?;
        files.block_path = open_file_dialog("Select the block object", &files.block_path, None).ok_or("No block file has been selected")?;
        files.tileset_path = open_file_dialog("Select the tileset you want to use", &files.tileset_path, None).ok_or("No tileset file has been selected")?;
        save_last_files(&files).map_err(|_| "Unable to save config file to disk")?;
        backup_room(&files.room_path).map_err(|_| "Unable to save backup room to disk")?;
        let mut room = get_room(&files.room_path).map_err(|e| format!("Unable to read room file from disk. Error: {}", e))?;
        let tileset = get_tileset(&files.tileset_path).map_err(|e| format!("Unable to read tileset file from disk. Error: {}", e))?;
        let block = get_block(&files.block_path).map_err(|e| format!( "Unable to read block file from disk. Error: {}", e))?;
        add_tileset_to_room(&mut room, &tileset, &block).map_err(|e| format!("Unable to add tiles to room. Error: {}", e))?;
        save_room(&room, &files.room_path).map_err(|e| format!("Unable to save the room with tiles to disk. Error: {}", e))?;
        Ok(())
    };

    match do_the_job() {
        Ok(_) => {
            println!("Tiles added successfully!");
            pause();
        }
        Err(e) => {
            println!("{}", e);
            pause();
        }
    }
}

    
