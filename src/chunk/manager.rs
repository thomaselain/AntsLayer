use std::ops::Range;

use crate::{ chunk::biomes::Params, renderer::{ self, Renderer } };

use super::{ generation::STARTING_AREA, tile::Tile, Chunk, CHUNK_WIDTH };

pub struct Manager {
    pub loaded_chunks: Vec<LoadedChunk>,
    pub test_biome: Params,
}

#[derive(Hash, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct LoadedChunk {
    pub pos: (i32, i32),
    pub c: Chunk,
}

impl Manager {
    pub fn new() -> Self {
        let default_biome = Params::ocean();

        Self {
            loaded_chunks: Manager::generate_range(
                -STARTING_AREA..STARTING_AREA,
                -(STARTING_AREA / 2)..STARTING_AREA / 2,
                Some(default_biome.clone())
            ),
            test_biome: default_biome.clone(),
        }
    }
    pub fn render(&mut self, renderer: &mut Renderer, timestamp: f64) {
        for chunk in &self.loaded_chunks {
            chunk.c.render(renderer, chunk.pos, timestamp);
        }
    }
    pub fn tile_at(&self, p: (i32, i32, i32)) -> Option<Tile> {
        let chunk_pos = (p.0 / (CHUNK_WIDTH as i32), p.1 / (CHUNK_WIDTH as i32));
        for loaded_chunk in self.loaded_chunks.clone() {
            if loaded_chunk.pos == chunk_pos{
                return Some(loaded_chunk.c.content[p])
            }
        }
        // Could not find this tile in loaded chunks
        None
    }
}
