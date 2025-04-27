use std::{ sync::{ mpsc::Sender, Arc, Mutex }, thread };

use biomes::BiomeConfig;
use coords::aliases::{ ChunkPos, TilePos };
use serde::{ Deserialize, Serialize };

use crate::{ Chunk, CHUNK_HEIGHT };

// type ChunkKey = TilePos;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum ChunkError {
    FailedToLoad,
    FailedToGenerate,
    NoFile,
    NotVisible,
    Loading,
}

impl ChunkError {
    pub fn to_string(self) -> String {
        let e = "";
        e.to_string()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Status {
    Pending,
    Visible(Chunk),
    Ready(Chunk),
    Error(ChunkError),
}

impl Status {
    pub fn get_chunk(&self) -> Result<Chunk, Self> {
        match self {
            Status::Ready(chunk) | Status::Visible(chunk) => Ok(chunk.clone()),
            Status::Pending => Err(self.clone()),
            _ => Err(self.clone()),
        }
    }
    pub fn visible(self) -> Result<Self, ()> {
        match self {
            Status::Ready(chunk) | Status::Visible(chunk) => Ok(Self::Visible(chunk)),
            _ => Err(()),
        }
    }
}

impl Chunk {
    pub fn generate_async(
        key: ChunkPos,
        seed: u32,
        biome: &BiomeConfig,
        sender: Sender<(ChunkPos, Status)>
    ) {
        let chunk = Arc::new(Mutex::new(Chunk::new(key)));

        // Envoyer l'état Pending avant de commencer la génération
        sender.clone().send((key.into(), Status::Pending)).unwrap();

        let biome = biome.clone();
        thread::spawn(move || {
            let (chunk, sender) = (Arc::clone(&chunk), sender.clone());
            for z in 0..CHUNK_HEIGHT as i32 {
                let layer = Chunk::generate_layer(key, seed, &biome, z);

                chunk.lock().unwrap().layers[z as usize] = layer;

                // PENDING
                // sender.send((key, Status::Pending)).unwrap();
            }

            // READY
            sender.send((key, Status::Ready(chunk.lock().unwrap().clone()))).unwrap();
        });
    }
}
