use std::{ collections::btree_map::Range, fmt::{ self } };

use biomes::Params;
use manager::LoadedChunk;
use noise::{ Fbm, NoiseFn, Perlin };
use tile::Tile;

pub mod biomes;
pub mod generation;
pub mod manager;

/// Name export so it's not confused with Ant::Manager
#[allow(unused)]
pub use manager::Manager as ChunkManager;

pub mod tile;
pub mod index;

/// Chunk's data
#[derive(Hash, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct ChunkContent([Tile; FLAT_CHUNK_SIZE]);
const FLAT_CHUNK_SIZE: usize = CHUNK_WIDTH * CHUNK_WIDTH * CHUNK_HEIGHT;
pub const CHUNK_WIDTH: usize = 16;
pub const CHUNK_HEIGHT: usize = 128;
pub const SEA_LEVEL: usize = generation::SEA_LEVEL;

/// Allows ASCII display
impl fmt::Debug for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..FLAT_CHUNK_SIZE {
            write!(f, "{:?}", self.content.0[i])?;
            // if i % CHUNK_WIDTH == 0 {
            //     write!(f, "\n")?;
            // }
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
    pub fn index_to_xyz(index: usize) -> (i32, i32, i32) {
        (
            // X
            (index % CHUNK_WIDTH) as i32,
            // Y
            ((index / CHUNK_WIDTH) % CHUNK_WIDTH) as i32,
            // Z
            ((index / CHUNK_WIDTH.pow(2)) % CHUNK_HEIGHT) as i32,
        )
    }
    pub fn column(index: i32) -> Vec<(i32, i32, i32)> {
        assert!(index >= 0 && index < ((CHUNK_WIDTH * CHUNK_WIDTH * CHUNK_HEIGHT) as i32));
        let (x, y, _) = Self::index_to_xyz(index as usize);
        (0..CHUNK_HEIGHT as i32).map(|z| (x, y, z)).collect()
    }
}

#[derive(Hash, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct Chunk {
    pub content: ChunkContent,
}

impl Chunk {
    pub fn new(pos: (i32, i32)) -> LoadedChunk {
        LoadedChunk { pos, c: Chunk { content: ChunkContent::new() } }
    }
    pub fn content(self) -> [Tile; FLAT_CHUNK_SIZE] {
        return self.content.0;
    }
}

impl Chunk {
    pub fn generate(&mut self, pos: (i32, i32), b: Params, p: Fbm<Perlin>) {
        for z in 0..CHUNK_HEIGHT as i32 {
            for x in 0..CHUNK_WIDTH as i32 {
                for y in 0..CHUNK_WIDTH as i32 {
                    let (nx, ny) = (
                        (x as f64) + (pos.0 as f64) * (CHUNK_WIDTH as f64),
                        (y as f64) + (pos.1 as f64) * (CHUNK_WIDTH as f64),
                    );
                    let v = p.get([
                        b.noise.scale * nx,
                        b.noise.scale * ny,
                        b.noise.scale * (z as f64),
                    ]);

                    // eprintln!("{:.2?}", v);

                    self.content[(x, y, z)] = b.tile_at((x, y, z), v);
                }
            }
        }
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

    #[test]
    fn all_biomes() {
        let biomes = Params::all();

        for b in biomes {
            let chunk = Chunk::from_biome((0, 0), &b);
            println!("{:?}: \n{:?}\n", b.name, chunk.c);
        }
    }

    #[test]
    fn manager() {
        let biomes = Params::all();
        assert!(!biomes.is_empty());

        let mngr = ChunkManager::new();
        assert!(!mngr.loaded_chunks.is_empty());
    }
}
