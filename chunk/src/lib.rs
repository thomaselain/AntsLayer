#[cfg(test)]
mod tests;

pub mod debug;
pub mod thread;

use std::collections::HashMap;
use std::fmt::Display;
use std::fs::{ self, File };
use std::path::Path;
use biomes::{BiomeConfig, Config};
use coords::aliases::TilePos;
use serde::{ Deserialize, Serialize };
use thread::{ ChunkError, Status };
use tile::{ Tile, TileFlags, TileType };
use unit::Unit;
use std::io::{ self, Read, Seek, SeekFrom };

pub const CHUNK_SIZE: usize = 32;

#[derive(Clone)]
pub struct ChunkPath(String, TilePos);

impl Display for ChunkPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}_{}.bin", self.0.as_str(), self.1.x(), self.1.y())
    }
}
impl Into<String> for ChunkPath {
    fn into(self) -> String {
        format!("{}", self)
    }
}
impl Default for ChunkPath {
    fn default() -> Self {
        Self::new("default", TilePos::new(0, 0))
    }
}
impl ChunkPath {
    pub fn new(path: &str, key: TilePos) -> Self {
        Self(path.trim().to_string(), key)
    }
    pub fn chunk_key(&self) -> TilePos {
        self.1
    }
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Chunk {
    pub key: TilePos,
    pub units: HashMap<TilePos, Unit>,
    pub is_dirty: bool,
    pub tiles: [[Tile; CHUNK_SIZE]; CHUNK_SIZE], // Stockage linéaire pour optimiser la mémoire cache
}

impl Default for Chunk {
    fn default() -> Self {
        Self::new(TilePos::default())
    }
}

impl Chunk {
    pub fn new(key: TilePos) -> Self {
        let default_tile = Tile::new(key, TileType::Empty, 0, TileFlags::empty());
        Self {
            key,
            units: HashMap::new(),
            is_dirty: true,
            tiles: [[default_tile; CHUNK_SIZE]; CHUNK_SIZE],
        }
    }

    /// Generate a chunk without multi threading
    pub fn generate_default(pos: TilePos) -> (TilePos, Status) {
        let (pos, chunk) = Self::generate_from_biome(pos, 0, BiomeConfig::default());
        (pos, Status::Ready(chunk))
    }

    pub fn generate_from_biome(key: TilePos, seed: u32, biome: BiomeConfig) -> (TilePos, Chunk) {
        let mut chunk = Chunk::new(key);
        let perlin = BiomeConfig::noise_from_seed(seed);
        let chunk_offset = TilePos::new(
            key.x() * (CHUNK_SIZE as i32),
            key.y() * (CHUNK_SIZE as i32)
        );

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                let nx = ((x as f64) + (chunk_offset.x() as f64)) / (CHUNK_SIZE as f64);
                let ny = ((y as f64) + (chunk_offset.y() as f64)) / (CHUNK_SIZE as f64);

                // Combinaison des couches de bruit
                let value = biome.clone().combined_noise(seed, &perlin, (nx, ny));

                // if value < -1.0 || value > 1.0 {
                //     panic!("v = {:.2}", value);
                // }
                // Détermine le type de tuile
                let tile_type = biome.clone().tile_type_from_noise(value * Config::height_at((x as i32,y as i32),seed) );

                chunk.set_tile(x, y, Tile::new(key, tile_type, 0, TileFlags::empty()));
            }
        }

        (key, chunk)
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Result<&Tile, ()> {
        if x < CHUNK_SIZE && y < CHUNK_SIZE { Ok(&self.tiles[x][y]) } else { Err(()) }
    }

    pub fn set_tile(&mut self, x: usize, y: usize, tile: Tile) {
        if x < CHUNK_SIZE && y < CHUNK_SIZE {
            self.tiles[x][y] = tile;
        }
    }

    pub fn save(&self, path: ChunkPath) -> Result<(), std::io::Error> {
        // eprintln!("Saving chunk at {}", path);
        let binding = path.to_string();
        let path = binding.as_str();
        // eprintln!("path as str: {}", path);

        if self.is_dirty {
            let p = Path::new(path);

            if let Some(parent) = p.parent() {
                fs::create_dir_all(parent)?;
            }
            let str_path: String = path.into();

            let file = match std::fs::File::create(str_path.clone()) {
                Ok(file) => { file }
                Err(e) => {
                    // println!("Chunk::save() failed to create file at {}\nreason :{}", str_path, e);
                    return Err(e);
                }
            };

            bincode::serialize_into(file, self).expect("Failed to serialize");
            Ok(())
        } else {
            Ok(())
        }
    }

    pub fn load(path: ChunkPath) -> Result<(TilePos, Status), (TilePos, ChunkError)> {
        // println!("Loading chunk at {}", path);
        let chunk_file = File::open(path.clone().to_string());
        let key = path.chunk_key();

        // eprintln!("{:?}", path.clone().to_string());
        if let Ok(file) = chunk_file {
            let file_chunk = bincode::deserialize_from::<File, Chunk>(file);

            match file_chunk {
                Ok(chunk) => {
                    // println!("{:?}", chunk);
                    Ok((key, Status::Ready(chunk)))
                }
                Err(_) => { Err((key, ChunkError::FailedToLoad)) }
            }
        } else {
            return Err((key, ChunkError::NoFile));
        }

        // eprintln!("Failed to load chunk at {}", path.to_string());
    }

    pub fn skip_in_file<R: Read + Seek>(reader: &mut R) -> io::Result<()> {
        // Calcule la taille d'un chunk en octets
        let chunk_size_in_bytes = CHUNK_SIZE * CHUNK_SIZE * std::mem::size_of::<Tile>();

        // Saute les octets correspondants
        reader.seek(SeekFrom::Current(chunk_size_in_bytes as i64))?;
        Ok(())
    }
}
