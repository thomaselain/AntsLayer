use std::fmt::{ self };

use biomes::Params;
use noise::{ Fbm, NoiseFn, Perlin };
use tile::Tile;

pub mod biomes;
mod generation;
pub mod manager;
pub mod tile;

/// Chunk's data
#[derive(Clone, Copy)]
pub struct ChunkContent([Tile; FLAT_CHUNK_SIZE]);
const FLAT_CHUNK_SIZE: usize = CHUNK_WIDTH * CHUNK_WIDTH * CHUNK_HEIGHT;
pub const CHUNK_WIDTH: usize = 16;
pub const CHUNK_HEIGHT: usize = 128;
pub const SEA_LEVEL: usize = generation::SEA_LEVEL;
mod index;

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
}

#[derive(Clone, Copy)]
pub struct Chunk {
    pub content: ChunkContent,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            content: ChunkContent::new(),
        }
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
    use crate::chunk::biomes::Biomes;
    use crate::chunk::*;
    use crate::Manager;

    #[test]
    fn all_biomes() {
        let biomes = Params::all();

        for b in biomes {
            let chunk = Chunk::from_biome((0, 0), &b);
            println!("{:?}: \n{:?}\n", b.name, chunk);
        }
    }

    #[test]
    fn manager() {
        let biomes = Biomes::load();
        assert!(!biomes.params.is_empty());

        let mngr = Manager::new();
        assert!(!mngr.loaded_chunks.is_empty());
    }
}
