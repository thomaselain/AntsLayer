use std::ops::Range;

use crate::chunk::biomes::Params;

use super::Chunk;

pub struct Manager {
    pub loaded_chunks: Vec<LoadedChunk>,
    pub test_biome: Params,
}

#[derive(Hash, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct LoadedChunk {
    pub pos: (i32, i32),
    pub c: Chunk,
}

// Default empty chunks
impl Manager {
    pub fn generate_range(
        x_range: Range<i32>,
        y_range: Range<i32>,
        p: Option<Params>
    ) -> Vec<LoadedChunk> {
        let mut m = Vec::new();

        for j in y_range {
            for i in x_range.clone() {
                // Biome as arg : use it
                if let Some(ref p) = p {
                    m.push(Chunk::from_biome((i, j), &p));
                } else {
                    // No biome given : generate empty
                    m.push(Chunk::new((i, j)));
                }
            }
        }

        m
    }
}

impl Manager {
    pub fn new() -> Self {
        let default_biome = Params::ocean();

        Self {
            loaded_chunks: Manager::generate_range(-3..3, -3..3, Some(default_biome.clone())),
            test_biome: default_biome.clone(),
        }
    }
}
