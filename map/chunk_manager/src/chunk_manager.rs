use biomes::BiomeConfig;
use chunk::thread::{ ChunkError, Status };
use chunk::{Chunk, ChunkPath};
use coords::aliases::{ChunkPos, TilePos};
use std::collections::{ HashMap, HashSet };
use std::sync::mpsc::{ Receiver, Sender };
use std::sync::{ mpsc, Arc, Mutex };

use crate::ChunkManager;

impl Default for ChunkManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ChunkManager {
    pub fn new() -> Self {
        let (sndr, rcvr): (
            Sender<(ChunkPos, Status)>,
            Receiver<(ChunkPos, Status)>,
        ) = mpsc::channel();

        ChunkManager {
            sndr: Arc::new(Mutex::new(sndr)),
            rcvr: Arc::new(Mutex::new(rcvr)),
            loaded_chunks: HashMap::new(),
            visible_chunks: HashSet::new(),
        }
    }

    pub fn generate_chunk_no_thread(key: ChunkPos, seed: u32, biome_config: BiomeConfig) -> Chunk {
        let (_key, chunk) = Chunk::generate_from_biome(key, seed, biome_config);
        chunk
    }

    // pub fn generate_chunk_with_thread(x: i32, y: i32, seed: u32, biome_config: BiomeConfig) -> Chunk {
    //
    // let ((_x, _y), chunk) = Chunk::generate_async(x, y, seed, biome_config, MapChannel);
    // chunk
    // }

    pub fn load_chunk(
        &mut self,
        path:ChunkPath,
    ) -> Result<(ChunkPos, Chunk), (ChunkPos, ChunkError)> {
        let key = path.chunk_key();
        
        if let Some(status) = self.loaded_chunks.get(&key).cloned() {
            let chunk = status.get_chunk().ok();

            match chunk {
                Some(chunk) => { Ok((key, chunk)) }
                None => { Err((key, ChunkError::FailedToLoad))}
            }
        } else if let Ok((key, status)) = Chunk::load(path) {
            match status {
                Status::Visible(chunk) | Status::Ready(chunk) => Ok((key, chunk)),
                Status::Pending => { Err((key, ChunkError::StillLoading)) }
                Status::Error(_) => todo!(),
            }
        } else {
            Err((key, ChunkError::FailedToLoad))
        }
    }
}
