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
use chunk::{ Chunk, ChunkPath, CHUNK_SIZE };
use coords::aliases::TilePos;
use serde::{ Serialize, Deserialize };

pub const WORLD_STARTING_AREA: i32 = 10;
pub const WORLDS_FOLDER: &str = "data/worlds/";

#[derive(Clone, Serialize, Deserialize)]
pub struct Map {
    pub path: String,
    pub seed: u32,
    pub chunks: HashMap<TilePos, Chunk>, // Utilisation de coordonnées pour les chunks
}

pub enum Directions {
    North,
    East,
    South,
    West,
}
impl Default for Map {
    fn default() -> Self {
        Self::new("default").ok().unwrap()
    }
}

impl Map {
    pub fn init_test() -> Self {
        Self::new("test").ok().unwrap()
    }

    pub fn new(name: &str) -> Result<Self, String> {
        Self::init_world_folder(name)?;

        Ok(Self::init_world(name).ok().expect("Failed to generate starting zone"))
    }

    // Ajouter un chunk
    pub fn add_chunk(&mut self, key: TilePos, chunk: Chunk) -> std::io::Result<()> {
        let path = ChunkPath::new(&self.path, key);
        self.chunks.insert(key, chunk.clone());

        chunk.save(path)?;
        Ok(())
    }

    pub fn get_chunk(&self, key: TilePos) -> Result<Chunk, ()> {
        if !self.chunks.contains_key(&key) {
            Err(())
        } else {
            Ok(self.chunks.get(&key).unwrap().clone())
        }
    }

    pub fn visible_chunks(camera: &Camera) -> HashSet<TilePos> {
        let chunk_x_start = camera.coords.x_i32() / (CHUNK_SIZE as i32);
        let chunk_y_start = camera.coords.y_i32() / (CHUNK_SIZE as i32);

        let mut visible = HashSet::new();

        for x in chunk_x_start - (camera.render_distance as i32)..=chunk_x_start +
            (camera.render_distance as i32) {
            for y in chunk_y_start - (camera.render_distance as i32)..=chunk_y_start +
                (camera.render_distance as i32) {
                visible.insert(TilePos::new(x, y));
            }
        }
        visible
    }
}
