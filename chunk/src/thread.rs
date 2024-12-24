use std::{ sync::mpsc::Sender, thread };

use biomes::BiomeConfig;
use coords::aliases::TilePos;
use serde::{ Deserialize, Serialize };

use crate::Chunk;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum ChunkError {
    FailedToLoad,
    FailedToGenerate,
    FailedToOpenFile,
    NotVisible,
    StillLoading,
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
        key: TilePos,
        seed: u32,
        biome_config: BiomeConfig,
        sender: Sender<(TilePos, Status)>
    ) {
        // Envoyer l'état Pending avant de commencer la génération
        sender.clone().send((key, Status::Pending)).unwrap();

        thread::spawn(move || {
            let (key, chunk) = Self::generate_from_biome(key, seed, biome_config);
            // let ((x, y), chunk) = Self::generate_from_biome(x, y, seed, biome_config)?;

            // Envoyer l'état Ready en verrouillant le Mutex
            sender.send((key, Status::Ready(chunk))).unwrap();
        });
    }
}
