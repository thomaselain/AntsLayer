#[cfg(test)]
mod tests;

pub mod debug;
pub mod thread;

use std::collections::HashMap;
use std::default;
use std::fmt::Display;
use std::fs::{ self, File };
use std::path::Path;
use biomes::{ BiomeConfig, Config };
use coords::aliases::{ ChunkPos, TilePos };
use coords::Coords;
use serde::{ Deserialize, Serialize };
use thread::{ ChunkError, Status };
use tile::{ Tile, TileFlags, TileType };
use unit::Unit;
use std::io::{ self, Read, Seek, SeekFrom };

pub const CHUNK_WIDTH: usize = 10;
pub const CHUNK_HEIGHT: usize = 5;

#[derive(Clone)]
pub struct ChunkPath(String, ChunkPos);

impl Display for ChunkPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}_{}.bin", self.0.as_str(), self.1.0, self.1.1)
    }
}
impl Into<String> for ChunkPath {
    fn into(self) -> String {
        format!("{}", self)
    }
}
impl Default for ChunkPath {
    fn default() -> Self {
        Self::new("default", (0, 0))
    }
}
impl ChunkPath {
    pub fn new(path: &str, key: ChunkPos) -> Self {
        Self(path.trim().to_string(), key)
    }
    pub fn chunk_key(&self) -> ChunkPos {
        self.1.into()
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub struct ChunkLayer {
    tiles: [[Tile; CHUNK_WIDTH]; CHUNK_WIDTH],
}

impl ChunkLayer {
    pub fn empty(key: ChunkPos) -> [[Tile; CHUNK_WIDTH]; CHUNK_WIDTH] {
        [[Tile::new_empty(key.into_world_()); CHUNK_WIDTH]; CHUNK_WIDTH]
    }
    pub fn new(pos: Coords<i32>, z: i32) -> Self {
        // Fill new chunk with empty traversable tiles
        // But with 0,0,0 as coords, it will be set in the loop later (Syntax could probably be improved a lot)
        let empty_tile = Tile::new(Coords::new(0, 0, 0), TileType::Empty, TileFlags::TRAVERSABLE);
        let mut new = [[empty_tile; CHUNK_WIDTH]; CHUNK_WIDTH];
        //

        // Coords setting in loop
        for x in 0..CHUNK_WIDTH as i32 {
            for y in 0..CHUNK_WIDTH as i32 {
                new[x as usize][y as usize].coords = Coords::new(
                    x + pos.x() * (CHUNK_WIDTH as i32),
                    y + pos.y() * (CHUNK_WIDTH as i32),
                    z
                );
                // eprintln!("{:?}", new[x as usize][y as usize].coords);
            }
        }
        //

        ChunkLayer { tiles: new }
    }

    pub fn set_tile(mut self, x: i32, y: i32, tile: Tile) {
        self.tiles[x as usize][y as usize] = tile;
    }
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Chunk {
    // pub units: HashMap<TilePos, Unit>,
    pub key: ChunkPos,
    pub is_dirty: bool,
    pub layers: [ChunkLayer; CHUNK_HEIGHT],
}

impl Default for Chunk {
    fn default() -> Self {
        Self::new((0, 0))
    }
}

#[test]
fn new_chunk() {
    let chunk = Chunk::generate_default((0, 0));
}

impl Chunk {
    pub fn new(key: ChunkPos) -> Self {
        let layers = [ChunkLayer::empty(key); CHUNK_HEIGHT];

        for z in 0..CHUNK_HEIGHT {
            layers[z]
        }

        Self { key, is_dirty: true, layers }
    }

    /// Generate a specific layer from a chunk
    pub fn generate_layer(key: ChunkPos, seed: u32, biome: &BiomeConfig, z: i32) -> ChunkLayer {
        let perlin = BiomeConfig::noise_from_seed(seed);
        let layer = ChunkLayer::new(key.into(), z);

        for x in 0..CHUNK_WIDTH as i32 {
            for y in 0..CHUNK_WIDTH as i32 {
                let tile_pos = layer.tiles[x as usize][y as usize].coords;

                let value = biome.clone().combined_noise(&perlin, tile_pos);
                // eprintln!("value at {:?} : {:?}", tile_pos, value);

                // Détermine le type de tuile
                let tile_type = biome.clone().tile_type_from_noise(value);
                let new_tile = Tile::new(tile_pos, tile_type, TileFlags::empty());
                layer.set_tile(x, y, new_tile);
                eprintln!("{:?}", new_tile);
            }
        }

        layer
    }

    /// Generate a chunk without multi threading
    pub fn generate_default(pos: ChunkPos) -> (TilePos, Status) {
        let (pos, chunk) = Self::generate_from_biome(pos, 0, BiomeConfig::default());
        (pos.into(), Status::Ready(chunk))
    }

    pub fn generate_from_biome(key: ChunkPos, seed: u32, biome: BiomeConfig) -> (ChunkPos, Chunk) {
        let mut chunk = Chunk::new(key);

        for z in 0..CHUNK_HEIGHT as i32 {
            let layer = Chunk::generate_layer(key, seed, &biome, z);

            chunk.layers[z as usize] = layer;
        }

        (key, chunk)
    }

    pub fn get_tile(&self, x: usize, y: usize, z: usize) -> Result<&Tile, ()> {
        if x < CHUNK_WIDTH && y < CHUNK_WIDTH && z < CHUNK_HEIGHT && z >= 0 {
            Ok(&self.layers[z].tiles[x][y])
        } else {
            Err(())
        }
    }

    pub fn set_tile(&mut self, x: usize, y: usize, z: usize, tile: Tile) {
        if x < CHUNK_WIDTH && y < CHUNK_WIDTH && z < CHUNK_HEIGHT && z >= 0 {
            self.layers[z].tiles[x][y] = tile;
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

    pub fn load(path: ChunkPath) -> Result<(ChunkPos, Status), (ChunkPos, ChunkError)> {
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
        let chunk_size_in_bytes = CHUNK_WIDTH * CHUNK_WIDTH * std::mem::size_of::<Tile>();

        // Saute les octets correspondants
        reader.seek(SeekFrom::Current(chunk_size_in_bytes as i64))?;
        Ok(())
    }
}
