#[cfg(test)]
mod tests;

pub mod debug;
pub mod thread;

use std::fs::{ self, File };
use std::path::Path;
use biomes::BiomeConfig;
use serde::{ Deserialize, Serialize };
use thread::Status;
use tile::{ Tile, TileFlags, TileType };
use std::io::{ self, Read, Seek, SeekFrom };

pub const CHUNK_SIZE: usize = 16;

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub struct Chunk {
    pub is_dirty: bool,
    pub tiles: [[Tile; CHUNK_SIZE]; CHUNK_SIZE], // Stockage linéaire pour optimiser la mémoire cache
}

impl Chunk {
    pub fn new() -> Self {
        let default_tile = Tile::new((0, 0), TileType::Empty, 0, TileFlags::empty());
        Self {
            is_dirty: true,
            tiles: [[default_tile; CHUNK_SIZE]; CHUNK_SIZE],
        }
    }

    pub fn generate_default(x: i32, y: i32) {
        Self::generate_from_biome(x, y, 0, BiomeConfig::default());
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
                let tile_type = if value < biome_config.liquid_threshold {
                    TileType::Liquid
                } else if value < biome_config.floor_threshold {
                    TileType::Floor
                } else if value < biome_config.wall_threshold {
                    TileType::Wall
                } else {
                    TileType::Custom(0) // Exemple de type personnalisé pour des cas rares
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

    pub fn save(&self, path: &str) -> Result<(), std::io::Error> {
        if self.is_dirty {
            if let Some(parent) = Path::new(path).parent() {
                fs::create_dir_all(parent)?;
            }
            let file = std::fs::File::create(path)?;
            bincode::serialize_into(file, self).expect("Failed to serialize");
            Ok(())
        } else {
            Ok(()) // Pas besoin de sauvegarder si non modifié
        }
    }

    pub fn load(file_path: &str) -> Result<Status, ()> {
        let file = File::open(file_path).expect(
            &format!("Failed to load chunk at {}", file_path).to_string()
        );
        Ok(
            Status::Ready(
                bincode
                    ::deserialize_from(file)
                    .map_err(|e| {
                        std::io::Error::new(
                            std::io::ErrorKind::Other,
                            format!("Deserialization error: {}", e)
                        )
                    })
                    .expect("Failed to deserialize")
            )
        )
    }
    pub fn skip_in_file<R: Read + Seek>(reader: &mut R) -> io::Result<()> {
        // Calcule la taille d'un chunk en octets
        let chunk_size_in_bytes = CHUNK_SIZE * CHUNK_SIZE * std::mem::size_of::<Tile>();

        // Saute les octets correspondants
        reader.seek(SeekFrom::Current(chunk_size_in_bytes as i64))?;
        Ok(())
    }
}
