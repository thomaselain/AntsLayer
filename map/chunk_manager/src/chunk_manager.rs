use biomes::BiomeConfig;
use chunk::thread::{ChunkKey, Status, X, Y};
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

    // pub fn generate_chunk_with_thread(x: i32, y: i32, seed: u32, biome_config: BiomeConfig) -> Chunk {
        // 
        // let ((_x, _y), chunk) = Chunk::generate_async(x, y, seed, biome_config, MapChannel);
        // chunk
    // }

    pub fn load_chunk(&mut self, key: ChunkKey, seed: u32) -> Status {
        if let Some(status) = self.chunks.get(&key).cloned() {
            match status {
                Status::Pending => {
                    println!("Chunk {:?} est encore en attente...",key);
                    status
                }
                Status::Ready(_) | Status::Visible(_) => status,
                Status::ToGenerate => {
                    let ((_x, _y), chunk) = Chunk::generate_from_biome(
                        key.x(),
                        key.y(),
                        seed,
                        BiomeConfig::default()
                    );
                    self.chunks.insert(key, Status::Ready(chunk));
                    Status::Ready(chunk)
                }

                Status::Error(e) => panic!("{}", e.to_string()),
            }
        } else {
            // println!("Génération du chunk ({}, {}) ...", x, y);
            let ((_, _), chunk) = Chunk::generate_from_biome(key.x(), key.y(), seed, BiomeConfig::default());
            self.chunks.insert(key, Status::Ready(chunk));
            Status::Ready(chunk)
}
    }

    pub fn has(&self, x: i32, y: i32) -> Option<Status> {
        self.chunks.get(&(x, y)).cloned()
    }
}
