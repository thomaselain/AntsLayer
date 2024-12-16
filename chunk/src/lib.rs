#[cfg(test)]
mod tests;

pub mod debug;
pub mod thread;

use std::fs::{ self, File };
use std::path::Path;
use std::sync::mpsc::{ self };
use std::thread::sleep;
use std::time::Duration;
use biomes::BiomeConfig;
use serde::{ Deserialize, Serialize };
use thread::{ ChunkError, ChunkKey, Status };
use tile::{ FluidType, Tile, TileFlags, TileType };
use std::io::{ self, Read, Seek, SeekFrom };

pub const CHUNK_SIZE: usize = 15;

#[derive(Clone)]
pub struct ChunkPath(String, i32, i32);

impl ChunkPath {
    fn new(path: String, x: i32, y: i32) -> Self {
        Self { 0: path, 1: x, 2: y }
    }
    pub fn to_string(self) -> String {
        format!("{}/{}_{}.bin", self.0, self.1, self.2).to_string()
    }
    pub fn build(path: String, x: i32, y: i32) -> std::io::Result<Self> {
        let dir = path.clone();
        if !Path::new(&dir).exists() {
            fs::create_dir_all(dir)?;
        }

        Ok(Self::new(path, x, y))
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub struct Chunk {
    pub x: i32,
    pub y: i32,
    pub is_dirty: bool,
    pub tiles: [[Tile; CHUNK_SIZE]; CHUNK_SIZE], // Stockage linéaire pour optimiser la mémoire cache
}

impl Chunk {
    pub fn new() -> Self {
        let default_tile = Tile::new((0, 0), TileType::Empty, 0, TileFlags::empty());
        Self {
            x: 0,
            y: 0,
            is_dirty: true,
            tiles: [[default_tile; CHUNK_SIZE]; CHUNK_SIZE],
        }
    }

    /// Generate a chunk without multi threading
    pub fn generate_default(x: i32, y: i32) -> ((i32, i32), Status) {
        let ((_x, _y), chunk) = Self::generate_from_biome(x, y, 0, BiomeConfig::default());

        ((x, y), Status::Ready(chunk))
    }

    /// Génère un chunk basé sur la configuration d'un biome
    pub fn generate_from_biome(
        x: i32,
        y: i32,
        seed: u32,
        biome_config: BiomeConfig
    ) -> ((i32, i32), Chunk) {
        let mut chunk = Chunk::new();
        let perlin = noise::Perlin::new(seed);
        let chunk_offset_x = x * (CHUNK_SIZE as i32);
        let chunk_offset_y = y * (CHUNK_SIZE as i32);

        // sleep(Duration::new(0, 5_000));

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                let nx = ((x as f64) + (chunk_offset_x as f64)) / (CHUNK_SIZE as f64);
                let ny = ((y as f64) + (chunk_offset_y as f64)) / (CHUNK_SIZE as f64);

                // Utilise le bruit pour déterminer la valeur de la tuile
                // Avec des octaves
                let value =
                    (noise::NoiseFn::get(&perlin, [
                        nx * biome_config.scale,
                        ny * biome_config.scale,
                    ]) +
                        0.5 *
                            noise::NoiseFn::get(&perlin, [
                                2.0 * nx * biome_config.scale,
                                2.0 * ny * biome_config.scale,
                            ])) /
                    1.5;
                // let value = (value / 1.5).clamp(-1.0, 1.0);

                // Attribue des types de tuiles selon les seuils du biome
                let tile_type = if value < biome_config.magma_threshold {
                    TileType::Fluid(FluidType::Magma)
                } else if value < biome_config.water_threshold {
                    TileType::Fluid(FluidType::Water)
                } else if value < biome_config.grass_threshold {
                    TileType::Grass
                } else if value < biome_config.dirt_threshold {
                    TileType::Dirt
                } else if value < biome_config.rock_threshold {
                    TileType::Rock
                } else {
                    TileType::Floor
                };

                chunk.set_tile(
                    x,
                    y,
                    Tile::new((x as i32, y as i32), tile_type, 0, TileFlags::empty())
                );
            }
        }
        ((x, y), chunk)
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

    pub fn load(path: ChunkPath, seed: u32) -> Result<(ChunkKey, Status), (ChunkKey, ChunkError)> {
        let chunk_file = File::open(path.clone().to_string());
        let (x, y) = (path.1, path.2);

        println!("{:?}", path.clone().to_string());

        if chunk_file.is_err() {
            let (sender, receiver) = mpsc::channel();
            Chunk::generate_async(x, y, seed, BiomeConfig::default(), sender);

            while let Ok(((_x, _y), status)) = receiver.recv_timeout(Duration::new(5, 0)) {
                match status {
                    Status::Ready(_chunk) => {
                        Chunk::save(&status.clone().get_chunk().ok().unwrap(), path.clone()).expect(
                            "Failed to write new chunk file"
                        );
                    }
                    Status::Error(e) => panic!("{:?}", e),
                    _ => {}
                }
            }

            // println!("{:?}", chunk_file);
            // return Ok(((x, y), status));
        }
        if let Ok(file) = chunk_file {
            Ok((
                (x, y),
                bincode
                    ::deserialize_from(file)
                    .map_err(|e| {
                        std::io::Error::new(
                            std::io::ErrorKind::Other,
                            format!("Deserialization error for chunk ({},{}):\n{}\n", x, y, e)
                        )
                    })
                    .expect("Failed to deserialize"),
            ))
        } else {
            println!("Failed to load chunk at {}", path.to_string());
            Err(((x, y), ChunkError::FailedToLoad))
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
