use std::{ sync::{ Arc, Mutex } };

use crate::{ chunk::{ biomes::Id, index, manager::LoadedChunk, tile::Tile } };

use super::{ Chunk, ChunkContent, WIDTH };

impl LoadedChunk {
    // Checks if this xyz is in this chunk (un peu crado)
    pub fn has(&self, (x, y, _z): (i32, i32, i32)) -> bool {
        let (x_min, x_max) = (
            self.pos.0 * (WIDTH as i32),
            (self.pos.0 + 1) * (WIDTH as i32) - 1,
        );
        let (y_min, y_max) = (
            self.pos.1 * (WIDTH as i32),
            (self.pos.1 + 1) * (WIDTH as i32) - 1,
        );

        if x > x_min && x < x_max && y > y_min && y < y_max {
            true
        } else {
            false
        }
    }
}
impl LoadedChunk {
    pub fn new(biome_id: Id, pos: (i32, i32)) -> Self {
        Self { biome_id, pos, c: Arc::new(Mutex::new(Chunk::new())) }
    }
    pub fn access_content(&self) -> ChunkContent {
        if let Some(c) = self.c.lock().ok() { c.content } else { panic!("Failed to lock chunk") }
    }
    pub fn access_tile(&self, pos: (i32, i32, i32)) -> Tile {
        if let Some(c) = self.c.lock().ok() {
            c.content[pos]
        } else {
            panic!("Failed to lock chunk")
        }
    }
    pub fn access_tile_from_index(&self, index: &usize) -> Tile {
        let pos = index::to_xyz(*index);
        if let Some(c) = self.c.lock().ok() {
            c.content[pos]
        } else {
            panic!("Failed to lock chunk")
        }
    }
}
