use std::{ collections::HashMap, ops::Range };

use noise::Perlin;

use crate::chunk::biomes::Params;

use super::Chunk;

pub struct Manager {
    pub loaded_chunks: HashMap<(i32, i32), Chunk>,
    pub test_biome: Params,
}

// Default empty chunks
impl Manager {
    pub fn generate_range(
        x_range: Range<i32>,
        y_range: Range<i32>,
        p: Option<Params>
    ) -> HashMap<(i32, i32), Chunk> {
        let mut m = HashMap::new();
        for j in y_range {
            for i in x_range.clone() {
                // Biome as arg : use it
                if let Some(ref p) = p {
                    m.insert((i, j), Chunk::from_biome((i, j), &p));
                } else {
                    // No biome given : generate empty
                    m.insert((i, j), Chunk::new());
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
            loaded_chunks: Manager::generate_range(-10..10, -10..10, Some(default_biome.clone())),
            test_biome: default_biome.clone(),
        }
    }
}
