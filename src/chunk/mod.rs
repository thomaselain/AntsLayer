use std::{ fmt::{ self } };

use tile::Tile;

pub mod biomes;
pub mod generation;
pub mod manager;

/// Name export so it's not confused with Ant::Manager
#[allow(unused)]
pub use manager::Manager as ChunkManager;

pub mod tile;
pub mod index;
pub mod thread;

/// Chunk's data
#[derive(Hash, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct ChunkContent([Tile; FLAT_CHUNK_SIZE]);
const FLAT_CHUNK_SIZE: usize = CHUNK_WIDTH * CHUNK_WIDTH * CHUNK_HEIGHT;

pub const CHUNK_WIDTH: usize = if cfg!(test) { 8 } else { 8 };
pub const CHUNK_HEIGHT: usize = if cfg!(test) { 128 } else { 128 };
pub const SEA_LEVEL: usize = generation::SEA_LEVEL;

/// Allows ASCII display
impl fmt::Debug for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..FLAT_CHUNK_SIZE {
            write!(f, "{:?}", self.get(index::to_xyz(i)))?;
        }
        Ok(())
    }
}

impl ChunkContent {
    pub fn new() -> Self {
        Self([Tile::air(); FLAT_CHUNK_SIZE])
    }
    pub fn len() -> usize {
        FLAT_CHUNK_SIZE
    }
    pub fn column(index: i32) -> Vec<(i32, i32, i32)> {
        assert!(index >= 0 && index < ((CHUNK_WIDTH * CHUNK_WIDTH * CHUNK_HEIGHT) as i32));
        let (x, y, _) = index::to_xyz(index as usize);
        (0..CHUNK_HEIGHT as i32).map(|z| (x, y, z)).collect()
    }
}

// #[derive(Hash, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct Chunk {
    pub content: ChunkContent,
}

impl Chunk {
    pub fn get(&self, p: (i32, i32, i32)) -> Tile {
        self.content[index::flatten_index_i32(p)]
    }
    pub fn set(&mut self, p: (i32, i32, i32), tile: Tile) {
        self.content[index::flatten_index_i32(p)] = tile;
    }
}

//
//
//
// TESTS
//
//
//
#[cfg(test)]
mod tests {
    use crate::chunk::*;
    use crate::ChunkManager;
    use super::biomes::Params;

    #[test]
    fn all_biomes() {
        let biomes = Params::all();

        for b in biomes {
            let chunk = Chunk::from_biome((0, 0), &b);
            let chunk = chunk.join().ok().unwrap();
            println!("{:?}: \n{:?}\n", b.id, chunk.c);
        }
    }

    #[test]
    fn manager() {
        let biomes = Params::all();
        assert!(!biomes.is_empty());

        // Filled manager
        let mngr = ChunkManager::default();
        assert!(!mngr.loaded_chunks.is_empty());
    }

    #[test]
    fn generation() {
        let mngr = ChunkManager::default();
        assert!(!mngr.loaded_chunks.is_empty());

        let len = mngr.loaded_chunks.len();
        for c in mngr.loaded_chunks {
            let (_pos, chunk) = c;
            println!("{:?}\n", chunk.c.lock().unwrap());
        }

        println!("Built a manager with {len} chunks in it");
    }
}
