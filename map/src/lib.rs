#[cfg(test)]
mod tests;

mod debug;
pub mod renderer;
pub mod camera;

pub extern crate chunk_manager;

use std::collections::HashMap;
use std::fs::{ self, File };
use std::future::{ Pending, Ready };
use std::io::{ self, Write };
use std::path::Path;
use std::thread::sleep_ms;
use std::time::Duration;
use biomes::{ BiomeConfig, Config };
use byteorder::ReadBytesExt;
use camera::Camera;
use chunk::threads::Status;
use chunk::{ Chunk, CHUNK_SIZE };
use crate::chunk_manager::ChunkManager;
use rand::Rng;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use serde::{ Serialize, Deserialize };
use tile::Tile;

#[derive(Clone, Serialize, Deserialize)]
pub struct Map {
    pub path: String,
    pub seed: u32,
    pub chunks: HashMap<(i32, i32), Chunk>, // Utilisation de coordonnées pour les chunks
}

pub const STARTING_AREA: i32 = 1;

pub enum Directions {
    North,
    East,
    South,
    West,
}

impl Map {
    pub fn generate_visible_chunks(&mut self, camera: &Camera, chunk_manager: &mut ChunkManager) {
        let start_chunk_x = (camera.coords.x() / (CHUNK_SIZE as f32)).floor() as i32;
        let start_chunk_y = (camera.coords.y() / (CHUNK_SIZE as f32)).floor() as i32;

        // Déterminer les chunks visibles autour de la caméra
        let end_chunk_x =
            ((camera.coords.x() as f32) + (camera.render_distance as f32)) / (CHUNK_SIZE as f32);
        let end_chunk_y =
            ((camera.coords.y() as f32) + (camera.render_distance as f32)) / (CHUNK_SIZE as f32);

        // Générer ou charger les chunks nécessaires dans la zone visible
        for x in start_chunk_x..=end_chunk_x as i32 {
            for y in start_chunk_y..=end_chunk_y as i32 {
                chunk_manager.load_chunk(x, y, self.seed);
            }
        }
    }

    pub fn get_dimensions(&self) -> (usize, usize) {
        let mut min_x = 0;
        let mut min_y = 0;
        let mut max_x = 0;
        let mut max_y = 0;

        // Parcours de tous les chunks et récupération des coordonnées maximales et minimales
        for &(x, y) in self.chunks.keys() {
            if x < min_x {
                min_x = x;
            }
            if y < min_y {
                min_y = y;
            }
            if x > max_x {
                max_x = x;
            }
            if y > max_y {
                max_y = y;
            }
        }

        // Multiplie par la taille d'un chunk pour obtenir les dimensions réelles
        (
            ((max_x - min_x + 1) * (CHUNK_SIZE as i32)) as usize,
            ((max_y - min_y + 1) * (CHUNK_SIZE as i32)) as usize,
        )
    }

    pub fn new(path: &str) -> Self {
        let mut rng = rand::thread_rng();
        let seed = rng.gen_range(0..10);
        let mut map = Map {
            seed,
            path: path.to_string(),
            chunks: HashMap::new(),
        };

        // Taille de la zone initiale (par exemple, une grille de 5x5 chunks)
        let half_size = STARTING_AREA / 2;

        // Générer les chunks de la zone de départ
        for x in -half_size..=half_size {
            for y in -half_size..=half_size {
                let biome_config = if x == 0 && y == 0 {
                    // Biome spécial pour le centre (ex : zone de départ sûre)
                    &BiomeConfig::default() // Vous pouvez personnaliser ici
                } else {
                    &BiomeConfig::default()
                };

                let ((x, y), new_chunk_status) = Chunk::generate_from_biome(
                    x,
                    y,
                    seed,
                    biome_config
                );

                'chunk_gen: loop {
                    match new_chunk_status {
                        // Just wait ? idk
                        Status::Pending => {
                            // std::thread::sleep(Duration::new(0, 500_000));
                        }

                        //Only add chunk to map when it is done loading
                        Status::Ready(chunk) => {
                            map.add_chunk(x, y, chunk);

                            break 'chunk_gen;
                        }
                    }
                }
            }
        }
        map.save();
        map
    }
    pub fn save(&self) {
        if let Some(parent) = Path::new(&self.path).parent() {
            fs::create_dir_all(parent).unwrap();
        }

        // Sauvegarder les données dans le fichier
        let file = std::fs::File::create(self.path.clone()).ok().unwrap();
        bincode::serialize_into(file, self).expect("Could not save map");
        println!("Map saved at {}", self.path);
    }
    // Ajouter un chunk
    pub fn add_chunk(&mut self, x: i32, y: i32, chunk: Chunk) {
        self.chunks.insert((x, y), chunk);
    }

    // Sauvegarder la map entière
    pub fn write_to_file(&self) -> std::io::Result<()> {
        let mut file = File::create(&self.path)?;

        for ((x, y), chunk) in &self.chunks {
            // Écriture des coordonnées
            bincode::serialize_into(&mut file, x);
            bincode::serialize_into(&mut file, y);

            // Écriture des données du chunk
            bincode::serialize_into(&mut file, chunk);
        }

        Ok(())
    }
    // Charger la map entière
    pub fn load_from_file(path: &str) -> Result<Map, io::Error> {
        let file = File::open(path).ok();
        let map = bincode::deserialize_from(file.expect("failed to open map file"));
        Ok(map.ok().expect("Error while loading map?"))
    }

    pub fn load_chunk_from_file(&self, chunk_x: i32, chunk_y: i32) -> Option<Chunk> {
        let file = File::open(&self.path).ok()?;
        let mut reader = std::io::BufReader::new(file);

        // Parcours du fichier pour chercher le chunk
        while let Ok(x) = bincode::deserialize_from::<_, i32>(&mut reader) {
            let y = bincode::deserialize_from::<_, i32>(&mut reader).ok()?;

            if x == chunk_x && y == chunk_y {
                return bincode::deserialize_from(&mut reader).ok();
            } else {
                Chunk::skip_in_file(&mut reader).ok()?; // Ignorer ce chunk
            }
        }

        None
    }

    /// Retourne les indices des chunks visibles.
    pub fn visible_chunks(
        &self,
        camera: &Camera,
        chunk_manager: &mut ChunkManager
    ) -> HashMap<(i32, i32), Status> {
        let chunk_x_start = camera.coords.x_i32() / (CHUNK_SIZE as i32);
        let chunk_y_start = camera.coords.y_i32() / (CHUNK_SIZE as i32);

        let mut chunks = HashMap::new();

        // Les coordonnées des chunks visibles doivent être calculées en termes de chunks, pas de pixels
        for x in chunk_x_start - (camera.render_distance as i32)..=chunk_x_start +
            (camera.render_distance as i32) {
            for y in chunk_y_start - (camera.render_distance as i32)..=chunk_y_start +
                (camera.render_distance as i32) {
                let chunk = chunk_manager.load_chunk(x, y, self.seed);
                chunks.insert((x, y), chunk);
                // println!("Chunk {},{} became visible", x, y);
            }
        }
        chunks
    }
}
