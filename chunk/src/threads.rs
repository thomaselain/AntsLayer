use std::{
    sync::{ mpsc::{ self, Receiver, Sender }, Arc, Mutex },
    thread::{ self, sleep },
    time::Duration,
};

use biomes::BiomeConfig;
use noise::{ NoiseFn, Perlin };
use serde::{ Deserialize, Serialize };
use tile::{ Tile, TileFlags, TileType };

use crate::{ Chunk, CHUNK_SIZE };

/// multithreading key
/// (x, y)
pub type ChunkKey = (i32, i32);

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Status {
    Pending, // En attente de génération
    Ready(Chunk), // Prêt à être dessiné
}

impl Status {
    pub fn get_chunk(self) -> Result<Self, String> {
        match self {
            Status::Pending => Ok(Self::Pending),
            Status::Ready(chunk) => Ok(Self::Ready(chunk)),
            _ => Err("Invalid status".to_string()),
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
        // sender.send(((x, y), Status::Pending)).unwrap();

        thread::spawn(move || {
            let mut chunk = Chunk::new();
            let perlin = Perlin::new(seed);
            let chunk_offset_x = x * (CHUNK_SIZE as i32);
            let chunk_offset_y = y * (CHUNK_SIZE as i32);

            for x in 0..CHUNK_SIZE {
                for y in 0..CHUNK_SIZE {
                    let nx = ((x as f64) + (chunk_offset_x as f64)) / (CHUNK_SIZE as f64);
                    let ny = ((y as f64) + (chunk_offset_y as f64)) / (CHUNK_SIZE as f64);

                    // Utilise le bruit pour déterminer la valeur de la tuile
                    // Avec des octaves
                    let value =
                        (perlin.get([nx * biome_config.scale, ny * biome_config.scale]) +
                            0.5 *
                                perlin.get([
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
            // thread::sleep(Duration::new(0, 100_000_000));
            sender.send(((x as i32, y as i32), Status::Ready(chunk))).unwrap();
        });
    }
}
