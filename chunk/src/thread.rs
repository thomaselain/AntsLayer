use std::{ sync::mpsc::Sender, thread };

use biomes::BiomeConfig;
use serde::{ Deserialize, Serialize };

use crate::Chunk;

/// multithreading key
/// (x, y)
pub type ChunkKey = (i32, i32);

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum ChunkError {
    FailedToLoad,
    FailedToGenerate,
}
impl ChunkError {
    pub fn to_string(self) -> String {
        let e = match self {
            ChunkError::FailedToLoad => "Failed to load chunk",
            ChunkError::FailedToGenerate => "Failed to generate chunk",
        };
        e.to_string()
    }
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Status {
    ToGenerate,
    Pending,
    Visible(Chunk),
    Ready(Chunk),
    Error(ChunkError),
}

impl Status {
    pub fn get_chunk(self) -> Result<Chunk, Self> {
        match self {
            Status::Ready(chunk) | Status::Visible(chunk) => Ok(chunk),
            Status::Pending => Err(self),
            _ => Err(self),
        }
    }
}

impl Chunk {
    pub fn generate_async(
        x: i32,
        y: i32,
        seed: u32,
        biome_config: BiomeConfig,
        sender: Sender<(ChunkKey, Status)>
    ) {
        // Envoyer l'état Pending avant de commencer la génération
        sender.send(((x, y), Status::Pending)).unwrap();

        thread::spawn(move || {
            let ((x, y), chunk) = Self::generate_from_biome(x, y, seed, biome_config);
            // let ((x, y), chunk) = Self::generate_from_biome(x, y, seed, biome_config)?;

            sender.send(((x as i32, y as i32), Status::Ready(chunk))).unwrap();
        });
    }
}
