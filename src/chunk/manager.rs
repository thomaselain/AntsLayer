use std::collections::HashMap;

use noise::Perlin;

use crate::chunk::biomes::Params;

use super::Chunk;

pub struct Manager {
    pub loaded_chunks: HashMap<(i32, i32), Chunk>,
}

// Default empty chunks
impl Manager {
    pub fn _2by2_empty() -> HashMap<(i32, i32), Chunk> {
        [
            ((0, 0), Chunk::new()),
            ((0, 1), Chunk::new()),
            ((1, 0), Chunk::new()),
            ((1, 1), Chunk::new()),
        ]
        .into()
    }
    pub fn _2by2_plains() -> HashMap<(i32, i32), Chunk> {
        [
            ((0, 0), Chunk::from_biome((0, 0), &Params::plain())),
            ((0, 1), Chunk::from_biome((0, 1), &Params::plain())),
            ((1, 0), Chunk::from_biome((1, 0), &Params::plain())),
            ((1, 1), Chunk::from_biome((1, 1), &Params::plain())),
        ]
        .into()
    }
}

impl Manager {
    pub fn new() -> Self {
        Self {
            loaded_chunks: Manager::_2by2_empty(),
        }
    }
}
