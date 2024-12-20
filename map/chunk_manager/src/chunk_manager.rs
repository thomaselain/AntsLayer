use biomes::BiomeConfig;
use chunk::thread::{ ChunkError, Status };
use chunk::Chunk;
use coords::aliases::TilePos;
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
            Sender<(TilePos, Status)>,
            Receiver<(TilePos, Status)>,
        ) = mpsc::channel();

        ChunkManager {
            sndr: Arc::new(Mutex::new(sndr)),
            rcvr: Arc::new(Mutex::new(rcvr)),
            loaded_chunks: HashMap::new(),
            visible_chunks: HashSet::new(),
        }
    }

    pub fn generate_chunk_no_thread(key: TilePos, seed: u32, biome_config: BiomeConfig) -> Chunk {
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
        key: TilePos,
        world_name: String
    ) -> Result<(TilePos, Status), (TilePos, ChunkError)> {
        if let Some(status) = self.loaded_chunks.get(&key).cloned() {
            let chunk = status.get_chunk().ok();

            match chunk {
                Some(chunk) => { Ok((key, Status::Ready(chunk))) }
                None => { Err((key, ChunkError::FailedToLoad))}
            }
        } else if let Ok((key, status)) = Chunk::new(key).load(world_name.clone()) {
            match status {
                Status::Visible(_) | Status::Ready(_) => Ok((key, status)),
                Status::Pending => { Err((key, ChunkError::StillLoading)) }
                Status::Error(_) => todo!(),
            }
        } else {
            eprintln!("OK");
            Err((key, ChunkError::FailedToLoad))
        }
    }
}
