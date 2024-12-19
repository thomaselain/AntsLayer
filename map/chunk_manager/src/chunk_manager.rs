use biomes::BiomeConfig;
use chunk::thread::{ ChunkError, ChunkKey, Status, X, Y };
use chunk::{ Chunk, ChunkPath };
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
            Sender<(ChunkKey, Status)>,
            Receiver<(ChunkKey, Status)>,
        ) = mpsc::channel();

        ChunkManager {
            sndr: Arc::new(Mutex::new(sndr)),
            rcvr: Arc::new(Mutex::new(rcvr)),
            loaded_chunks: HashMap::new(),
            visible_chunks: HashSet::new(),
        }
    }

    pub fn generate_chunk_no_thread(key: ChunkKey, seed: u32, biome_config: BiomeConfig) -> Chunk {
        let ((_x, _y), chunk) = Chunk::generate_from_biome(key, seed, biome_config);
        chunk
    }

    // pub fn generate_chunk_with_thread(x: i32, y: i32, seed: u32, biome_config: BiomeConfig) -> Chunk {
    //
    // let ((_x, _y), chunk) = Chunk::generate_async(x, y, seed, biome_config, MapChannel);
    // chunk
    // }

    pub fn load_chunk(
        &mut self,
        key: ChunkKey,
        world_name: String
    ) -> Result<(ChunkKey, Status), (ChunkKey, ChunkError)> {
        if let Some(status) = self.loaded_chunks.get(&key).cloned() {
            match status {
                Status::Pending => {
                    println!("Chunk {:?} est encore en attente...", key);
                    Ok((key, status))
                }
                Status::Ready(_) | Status::Visible(_) => Ok((key, status)),
                Status::Error(e) => panic!("{}", e.to_string()),
            }
        } else if let Ok((key, status)) = Chunk::new().load(world_name.clone()) {
            match status {
                Status::Visible(_) | Status::Ready(_) => Ok((key, status)),
                Status::Pending => { Err((key, ChunkError::StillLoading)) }
                Status::Error(_) => todo!(),
            }
        } else {
            Err((key, ChunkError::FailedToLoad))
        }
    }

    pub fn has(&self, x: i32, y: i32) -> Option<Status> {
        self.loaded_chunks.get(&(x, y)).cloned()
    }
}
