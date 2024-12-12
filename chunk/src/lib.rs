#[cfg(test)]
mod tests;

pub mod debug;
pub mod threads;

use std::fs::{ self, File };
use std::path::Path;
use std::sync::mpsc::{ self, Receiver, Sender };
use biomes::BiomeConfig;
use serde::{ Deserialize, Serialize };
use threads::{ ChunkKey, Status };
use tile::{ Tile, TileFlags, TileType };
use std::io::{ self, Read, Seek, SeekFrom };

pub const CHUNK_SIZE: usize = 16;

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub struct Chunk {
    pub is_dirty:bool,
    pub tiles: [[Tile; CHUNK_SIZE]; CHUNK_SIZE], // Stockage linéaire pour optimiser la mémoire cache
}

impl Chunk {
    pub fn new() -> Self {
        let default_tile = Tile::new((0, 0), TileType::Empty, 0, TileFlags::empty());
        Self {
            is_dirty:true,
            tiles: [[default_tile; CHUNK_SIZE]; CHUNK_SIZE],
        }
    }

    /// Génère un chunk basé sur la configuration d'un biome
    pub fn generate_from_biome(
        x: i32,
        y: i32,
        seed: u32,
        biome_config: &BiomeConfig
    ) -> ((i32, i32), Status) {
        let (sender, receiver): (
            Sender<(ChunkKey, Status)>,
            Receiver<(ChunkKey, Status)>,
        ) = mpsc::channel();

        Self::generate_async(x, y, seed, biome_config.clone(), sender);
        // sleep(Duration::new(0, 100_000_000));
        return receiver.recv().expect(" !! ! !");
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Result<&Tile, ()> {
        if x < CHUNK_SIZE && y < CHUNK_SIZE { Ok(&self.tiles[y][x]) } else { Err(()) }
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
                    .unwrap()
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
