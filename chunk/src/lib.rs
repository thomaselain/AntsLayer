#[cfg(test)]
mod tests;

pub mod debug;
pub mod thread;

use std::collections::HashMap;
use std::fs::{ self, File };
use std::path::Path;
use biomes::BiomeConfig;
use coords::aliases::TilePos;
use serde::{ Deserialize, Serialize };
use thread::{ ChunkError, Status };
use tile::{ Tile, TileFlags, TileType };
use unit::Unit;
use std::io::{ self, Read, Seek, SeekFrom };

pub const CHUNK_SIZE: usize = 16;
#[derive(Clone)]
pub struct ChunkPath(String, TilePos);

impl Default for ChunkPath {
    fn default() -> Self {
        Self::build("default", TilePos::new(0, 0)).expect("Failed to create default world path")
    }
}
impl ChunkPath {
    fn new(path: String, key: TilePos) -> Self {
        Self(path, key)
    }
    pub fn to_string(self) -> String {
        format!("{:?}/{:?}_{:?}.bin", self.0, self.1.x(), self.1.y()).to_string()
    }
    pub fn chunk_key(&self) -> TilePos {
        self.1
    }
    pub fn build(path: &str, key: TilePos) -> std::io::Result<Self> {
        let dir = path;
        if !Path::new(&dir).exists() {
            fs::create_dir_all(dir)?;
        }

        Ok(Self::new(path.to_string(), key))
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
                let value = biome.combined_noise(&perlin, nx, ny);

                // Détermine le type de tuile
                let tile_type = BiomeConfig::tile_type_from_noise(value, &biome);

                chunk.set_tile(
                    x,
                    y,
                    Tile::new(key, tile_type, 0, TileFlags::empty())
                );
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
        if self.is_dirty {
            if let Some(parent) = Path::new(&path.clone().to_string()).parent() {
                fs::create_dir_all(parent)?;
            }
            let file = std::fs::File::create(path.to_string())?;
            bincode::serialize_into(file, self).expect("Failed to serialize");
            Ok(())
        } else {
            Ok(())
        }
    }

    pub fn load(self, world_name: String) -> Result<(TilePos, Status), (TilePos, ChunkError)> {
        let path = ChunkPath::build(&world_name, self.key).ok().unwrap();
        let chunk_file = File::open(path.clone().to_string());
        let key = path.chunk_key();

        // eprintln!("{:?}", path.clone().to_string());

        if let Ok(file) = chunk_file {
            Ok((
                key,
                bincode
                    ::deserialize_from(file)
                    .map_err(|e| {
                        std::io::Error::new(
                            std::io::ErrorKind::Other,
                            format!("Deserialization error for chunk {:?}:\n{}\n", key, e)
                        )
                    })
                    .expect("Failed to deserialize"),
            ))
        } else {
            eprintln!("Failed to load chunk at {}", path.to_string());
            Err((key, ChunkError::FailedToLoad))
        }
    }
    pub fn skip_in_file<R: Read + Seek>(reader: &mut R) -> io::Result<()> {
        // Calcule la taille d'un chunk en octets
        let chunk_size_in_bytes = CHUNK_SIZE * CHUNK_SIZE * std::mem::size_of::<Tile>();

        // Saute les octets correspondants
        reader.seek(SeekFrom::Current(chunk_size_in_bytes as i64))?;
        Ok(())
    }
}
