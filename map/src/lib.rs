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
use chunk::{ Chunk, ChunkPath, CHUNK_WIDTH };
use coords::aliases::{ChunkPos, TilePos};
use serde::{ Serialize, Deserialize };

pub const WORLD_STARTING_AREA: i32 = 5;
pub const WORLDS_FOLDER: &str = "data/worlds/";

#[derive(Clone, Serialize, Deserialize)]
pub struct Map {
    pub path: String,
    pub seed: u32,
    pub chunks: HashMap<ChunkPos, Chunk>, // Utilisation de coordonnées pour les chunks
}

pub enum Directions {
    Up,
    Down,
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
    pub fn add_chunk(&mut self, key: ChunkPos, chunk: Chunk) -> std::io::Result<()> {
        let path = ChunkPath::new(&self.path, key.into());
        self.chunks.insert(key.into(), chunk.clone());

        chunk.save(path)?;
        Ok(())
    }

    pub fn get_chunk(&self, key: ChunkPos) -> Result<Chunk, ()> {
        if !self.chunks.contains_key(&key) {
            Err(())
        } else {
            Ok(self.chunks.get(&key).unwrap().clone())
        }
    }

    pub fn visible_chunks(camera: &Camera) -> HashSet<ChunkPos> {
        let chunk_x_start = camera.coords.x_i32() / (CHUNK_WIDTH as i32);
        let chunk_y_start = camera.coords.y_i32() / (CHUNK_WIDTH as i32);

        let mut visible = HashSet::new();

        for x in chunk_x_start - (camera.render_distance as i32)..=chunk_x_start +
            (camera.render_distance as i32) {
            for y in chunk_y_start - (camera.render_distance as i32)..=chunk_y_start +
                (camera.render_distance as i32) {
                visible.insert((x, y));
            }
        }
        visible
    }
}
