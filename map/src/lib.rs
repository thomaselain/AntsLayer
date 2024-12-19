#[cfg(test)]
mod tests;

mod debug;
pub mod renderer;
pub mod camera;
pub mod thread;
pub mod world;
pub extern crate chunk_manager;

use std::collections::{ HashMap, HashSet };
// use biomes::BiomeConfig;
use camera::Camera;
use chunk::thread::{ ChunkKey, X, Y };
use chunk::{ Chunk, CHUNK_SIZE };
use serde::{ Serialize, Deserialize };

pub const WORLD_STARTING_AREA: i32 = 4;
pub const WORLDS_FOLDER: &str = "data/worlds/";

#[derive(Clone, Serialize, Deserialize)]
pub struct Map {
    pub path: String,
    pub seed: u32,
    pub chunks: HashMap<(i32, i32), Chunk>, // Utilisation de coordonnées pour les chunks
}

pub enum Directions {
    North,
    East,
    South,
    West,
}

impl Map {
    pub fn new(name: &str) -> Result<Self, String> {
        Self::init_world_folder(name)?;
        Self::init_world(name)
    }
    // Ajouter un chunk
    pub fn add_chunk(&mut self, key: ChunkKey, chunk: Chunk) -> std::io::Result<()> {
        self.chunks.insert((key.x(), key.y()), chunk);
        // chunk.save(ChunkPath::build(self.path.clone(), x, y).expect("Failed to save chunk"))?;
        Ok(())
    }

    pub fn get_chunk(&self, key: ChunkKey) -> Result<Chunk, ()> {
        if !self.chunks.contains_key(&key) { Err(()) } else { Ok(*self.chunks.get(&key).unwrap()) }
    }

    pub fn visible_chunks(camera: &Camera) -> HashSet<ChunkKey> {
        let chunk_x_start = camera.coords.x_i32() / (CHUNK_SIZE as i32);
        let chunk_y_start = camera.coords.y_i32() / (CHUNK_SIZE as i32);

        let mut visible = HashSet::new();
        
        for x in chunk_x_start - (camera.render_distance as i32)..=chunk_x_start +
            (camera.render_distance as i32) {
            for y in chunk_y_start - (camera.render_distance as i32)..=chunk_y_start +
                (camera.render_distance as i32) {
                let key = (x, y);
                visible.insert(key.clone());
            }
        }
        visible
    }
}
