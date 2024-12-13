use biomes::BiomeConfig;
use chunk::threads:: Status ;
use chunk::Chunk;
use std::collections::HashMap;

use crate::ChunkManager;

impl ChunkManager {
    pub fn new() -> Self {
        ChunkManager {
            chunks: HashMap::new(),
        }
    }

    pub fn generate_chunk_no_thread(x: i32, y: i32, seed: u32, biome_config: BiomeConfig) -> Chunk {
        let ((_x, _y), chunk) = Chunk::generate_from_biome(x, y, seed, biome_config);
        chunk
    }

    pub fn load_chunk(&mut self, x: i32, y: i32, seed: u32) -> Status {
        if let Some(status) = self.chunks.get(&(x, y)).cloned() {
            match status {
                Status::Pending => {
                    println!("Chunk ({}, {}) est encore en attente...", x, y);
                    status
                }
                Status::Ready(_) => status,
            }
        } else {
            // println!("Génération du chunk ({}, {}) ...", x, y);
            let ((_, _), chunk) = Chunk::generate_from_biome(x, y, seed, BiomeConfig::default());
            let chunk = Status::Ready(chunk);

            self.chunks.insert((x, y), chunk.clone());
            chunk
        }
    }

    pub fn has(&self, x: i32, y: i32) -> Option<Status> {
        self.chunks.get(&(x, y)).cloned()
    }
}
