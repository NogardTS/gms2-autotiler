use crate::modules::constants::*;
use crate::modules::structs::*;

use std::collections::HashMap;
use std::error::Error;

pub fn add_tileset_to_room<'a>(room: &'a mut Room, tileset: &'a Tileset, block: &'a Object) -> Result<&'a mut Room, Box<dyn Error>> {
    let blocks: Vec<Instance> = room.layers.iter().cloned().map(|layer| {
        let layer_blocks: Vec<Instance> = layer.instances.iter().cloned().filter(|instance| instance.obj_id == block.id).collect();
        layer_blocks
    }).flatten().collect();
    let mut block_offsets = HashMap::new();
    for block in blocks.into_iter() {
        let offset = (block.x as i64 % 32, block.y as i64 % 32);
        let these_blocks = block_offsets.entry(offset).or_insert(Vec::new());
        these_blocks.push(block);
    }
    for (offset, blocks) in block_offsets.iter() {
        let mut grid = vec![vec![false; room.room_settings.width as usize / 32]; room.room_settings.height as usize / 32];
        for block in blocks.iter() {
            for i in 0..(block.scale_x as i64) {
                for j in 0..(block.scale_y as i64) {
                    if block.y as i64 / 32 + j >= 0 && block.y as i64 / 32 + j < room.room_settings.height / 32 && block.x as i64 / 32 + i >= 0 && block.x as i64 / 32 + i < room.room_settings.width / 32 {
                        grid[(block.y as i64 / 32 + j) as usize][(block.x as i64 / 32 + i) as usize] = true;
                    }   
                }
            }
        }
        let mut canvas = vec![vec![EMPTY_TILE; room.room_settings.width as usize / 32 * 2]; room.room_settings.height as usize / 32 * 2];
        for (y, line) in grid.iter().enumerate() {
            for (x, pos) in line.iter().enumerate() {
                if *pos == true {
                    place_tile(x, y, &grid, &mut canvas);
                }
            }
        }
        let mut new_layer = DEFAULT_LAYER.clone();
        new_layer.id = uuid::Uuid::new_v4().to_string();
        new_layer.name = format!("Nogard tiles {}_{}", offset.0, offset.1);
        new_layer.tiles.as_mut().unwrap().serialise_height = canvas.len() as i64;
        new_layer.tiles.as_mut().unwrap().serialise_width = canvas[0].len() as i64;
        new_layer.tiles.as_mut().unwrap().tile_serialise_data = canvas.into_iter().flatten().collect();
        new_layer.tileset_id = Some(tileset.id.clone());
        new_layer.depth = -9971212; //TODO
        new_layer.x = Some(offset.0);
        new_layer.y = Some(offset.1);
        room.layers.push_front(new_layer);
    }
    Ok(room)
}

pub fn place_tile(x: usize, y: usize, grid: &[Vec<bool>], canvas: &mut [Vec<i64>]) {
    let h = grid.len();
    let w = grid.first().as_ref().unwrap().len();
    let mut neighbours = [[true; 3];3];
    if y > 0 &&     x > 0     {neighbours[0][0] = grid[y - 1][x - 1];}
    if y > 0                  {neighbours[0][1] = grid[y - 1][x    ];}
    if y > 0 &&     x + 1 < w {neighbours[0][2] = grid[y - 1][x + 1];}
    if              x > 0     {neighbours[1][0] = grid[y    ][x - 1];}
                              {neighbours[1][1] = grid[y    ][x    ];}
    if              x + 1 < w {neighbours[1][2] = grid[y    ][x + 1];}
    if y + 1 < h && x > 0     {neighbours[2][0] = grid[y + 1][x - 1];}
    if y + 1 < h              {neighbours[2][1] = grid[y + 1][x    ];}
    if y + 1 < h && x + 1 < w {neighbours[2][2] = grid[y + 1][x + 1];}
    if neighbours[0][0] == false && neighbours[0][1] == false && neighbours[1][0] == false {canvas[y * 2][x * 2] = 20;}
    if neighbours[0][0] == true  && neighbours[0][1] == false && neighbours[1][0] == false {canvas[y * 2][x * 2] = 20;}
    if neighbours[0][0] == false && neighbours[0][1] == true  && neighbours[1][0] == false {canvas[y * 2][x * 2] = 40;}
    if neighbours[0][0] == false && neighbours[0][1] == false && neighbours[1][0] == true  {canvas[y * 2][x * 2] = 22;}
    if neighbours[0][0] == true  && neighbours[0][1] == true  && neighbours[1][0] == false {canvas[y * 2][x * 2] = 40;}
    if neighbours[0][0] == false && neighbours[0][1] == true  && neighbours[1][0] == true  {canvas[y * 2][x * 2] = 48;}
    if neighbours[0][0] == true  && neighbours[0][1] == false && neighbours[1][0] == true  {canvas[y * 2][x * 2] = 22;}
    if neighbours[0][0] == true  && neighbours[0][1] == true  && neighbours[1][0] == true  {canvas[y * 2][x * 2] = 42;}

    if neighbours[0][2] == false && neighbours[0][1] == false && neighbours[1][2] == false {canvas[y * 2][x * 2 + 1] = 25;}
    if neighbours[0][2] == true  && neighbours[0][1] == false && neighbours[1][2] == false {canvas[y * 2][x * 2 + 1] = 25;}
    if neighbours[0][2] == false && neighbours[0][1] == true  && neighbours[1][2] == false {canvas[y * 2][x * 2 + 1] = 45;}
    if neighbours[0][2] == false && neighbours[0][1] == false && neighbours[1][2] == true  {canvas[y * 2][x * 2 + 1] = 23;}
    if neighbours[0][2] == true  && neighbours[0][1] == true  && neighbours[1][2] == false {canvas[y * 2][x * 2 + 1] = 45;}
    if neighbours[0][2] == false && neighbours[0][1] == true  && neighbours[1][2] == true  {canvas[y * 2][x * 2 + 1] = 47;}
    if neighbours[0][2] == true  && neighbours[0][1] == false && neighbours[1][2] == true  {canvas[y * 2][x * 2 + 1] = 23;}
    if neighbours[0][2] == true  && neighbours[0][1] == true  && neighbours[1][2] == true  {canvas[y * 2][x * 2 + 1] = 43;}

    if neighbours[2][0] == false && neighbours[2][1] == false && neighbours[1][0] == false {canvas[y * 2 + 1][x * 2] = 70;}
    if neighbours[2][0] == true  && neighbours[2][1] == false && neighbours[1][0] == false {canvas[y * 2 + 1][x * 2] = 70;}
    if neighbours[2][0] == false && neighbours[2][1] == true  && neighbours[1][0] == false {canvas[y * 2 + 1][x * 2] = 50;}
    if neighbours[2][0] == false && neighbours[2][1] == false && neighbours[1][0] == true  {canvas[y * 2 + 1][x * 2] = 72;}
    if neighbours[2][0] == true  && neighbours[2][1] == true  && neighbours[1][0] == false {canvas[y * 2 + 1][x * 2] = 50;}
    if neighbours[2][0] == false && neighbours[2][1] == true  && neighbours[1][0] == true  {canvas[y * 2 + 1][x * 2] = 38;}
    if neighbours[2][0] == true  && neighbours[2][1] == false && neighbours[1][0] == true  {canvas[y * 2 + 1][x * 2] = 72;}
    if neighbours[2][0] == true  && neighbours[2][1] == true  && neighbours[1][0] == true  {canvas[y * 2 + 1][x * 2] = 52;}

    if neighbours[2][2] == false && neighbours[2][1] == false && neighbours[1][2] == false {canvas[y * 2 + 1][x * 2 + 1] = 75;}
    if neighbours[2][2] == true  && neighbours[2][1] == false && neighbours[1][2] == false {canvas[y * 2 + 1][x * 2 + 1] = 75;}
    if neighbours[2][2] == false && neighbours[2][1] == true  && neighbours[1][2] == false {canvas[y * 2 + 1][x * 2 + 1] = 55;}
    if neighbours[2][2] == false && neighbours[2][1] == false && neighbours[1][2] == true  {canvas[y * 2 + 1][x * 2 + 1] = 73;}
    if neighbours[2][2] == true  && neighbours[2][1] == true  && neighbours[1][2] == false {canvas[y * 2 + 1][x * 2 + 1] = 55;}
    if neighbours[2][2] == false && neighbours[2][1] == true  && neighbours[1][2] == true  {canvas[y * 2 + 1][x * 2 + 1] = 37;}
    if neighbours[2][2] == true  && neighbours[2][1] == false && neighbours[1][2] == true  {canvas[y * 2 + 1][x * 2 + 1] = 73;}
    if neighbours[2][2] == true  && neighbours[2][1] == true  && neighbours[1][2] == true  {canvas[y * 2 + 1][x * 2 + 1] = 53;}
}
/*
 X X
XX X
  X 
 X  


  XX  XX
  XX  XX
XXXX  XX
XXXX  XX
    XX  
    XX  
  XX    
  XX    
*/