use std::{ sync::{ mpsc::Sender, Arc, Mutex }, thread };

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
    FailedToOpenFile,
    NotVisible,
}
impl ChunkError {
    pub fn to_string(self) -> String {
        let e = match self {
            _ => "",
        };
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
    pub fn get_chunk(self) -> Result<Chunk, Self> {
        match self {
            Status::Ready(chunk) | Status::Visible(chunk) => Ok(chunk),
            Status::Pending => Err(self),
            _ => Err(self),
        }
    }
    pub fn visible(self) -> Result<Self, ()> {
        match self {
            Status::Ready(chunk) | Status::Visible(chunk) => Ok(Self::Visible(chunk.clone())),
            _ => Err(()),
        }
    }
}

impl Chunk {
    pub fn generate_async(
        key: (i32, i32),
        seed: u32,
        biome_config: BiomeConfig,
        sender: Sender<(ChunkKey, Status)>
    ) {
        let (x, y) = key;

        // Envoyer l'état Pending avant de commencer la génération
        sender
            .clone()
            .send(((x, y), Status::Pending))
            .unwrap();

        thread::spawn(move || {
            let ((x, y), chunk) = Self::generate_from_biome(x, y, seed, biome_config);
            // let ((x, y), chunk) = Self::generate_from_biome(x, y, seed, biome_config)?;

            // Envoyer l'état Ready en verrouillant le Mutex
            sender.send(((x as i32, y as i32), Status::Ready(chunk))).unwrap();
        });
    }
}

pub trait X {
    fn x(self) -> i32;
}
impl X for ChunkKey {
    fn x(self) -> i32 {
        self.0
    }
}
pub trait Y {
    fn y(self) -> i32;
}
impl Y for ChunkKey {
    fn y(self) -> i32 {
        self.1
    }
}
