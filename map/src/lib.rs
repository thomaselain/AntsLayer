#[cfg(test)]
mod tests;

mod debug;
pub mod renderer;
pub mod camera;

pub extern crate chunk_manager;

use std::collections::HashMap;
use std::fs::File;
// use biomes::BiomeConfig;
use camera::Camera;
use chunk::thread::Status;
use chunk::{ Chunk, ChunkPath, CHUNK_SIZE };
use crate::chunk_manager::ChunkManager;
use rand::Rng;
use serde::{ Serialize, Deserialize };

pub const WORLD_STARTING_AREA: i32 = 4;
pub const WORLDS_FOLDER: &str = "data/worlds/";

#[derive(Clone, Serialize, Deserialize)]
pub struct Map {
    pub path: String,
    pub seed: u32,
    pub chunks: HashMap<(i32, i32), Chunk>, // Utilisation de coordonnées pour les chunks
}

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

    pub fn new(name: &str) -> Result<Self, String> {
        let mut rng = rand::thread_rng();
        let seed = rng.gen_range(0..10);

        let mut map = Map {
            seed,
            path: format!("{}{}", WORLDS_FOLDER, name).to_string(),
            chunks: HashMap::new(),
        };

        // Vérifier si le dossier "data/worlds" existe, sinon le créer
        if !std::fs::metadata(format!("{}{}", WORLDS_FOLDER, name).to_string()).is_ok() {
            // Créer le dossier "data/"
            if
                let Err(e) = std::fs::DirBuilder
                    ::new()
                    .recursive(true)
                    .create(format!("{}{}", WORLDS_FOLDER, name).to_string())
            {
                eprintln!("Erreur lors de la création du dossier '{}': {}", WORLDS_FOLDER, e);
                return Err(format!("{}{}", WORLDS_FOLDER, name).to_string());
            }
        }

        let half_size = WORLD_STARTING_AREA / 2;
        for x in -half_size..=half_size {
            for y in -half_size..=half_size {
                let chunk = Chunk::new();
                map.add_chunk(x, y, chunk).expect("Failed to add chunk");
            }
        }
        Ok(map)
    }

    // Ajouter un chunk
    pub fn add_chunk(&mut self, x: i32, y: i32, chunk: Chunk) -> std::io::Result<()> {
        self.chunks.insert((x, y), chunk);
        // chunk.save(ChunkPath::build(self.path.clone(), x, y).expect("Failed to save chunk"))?;
        Ok(())
    }

    // Sauvegarder la map entière
    pub fn save(&self) -> std::io::Result<()> {
        for ((x, y), chunk) in &self.chunks {
            chunk.save(ChunkPath::build(self.path.clone(), *x, *y).expect("Failed to save chunk"))?;
        }
        Ok(())
    }

    // Charger la map entière
    // pub fn load(path: &str) -> Result<Map, io::Error> {
    //     let file = File::open(path).ok();
    //     let map = bincode::deserialize_from(file.expect("failed to open map file"));
    //     Ok(map.ok().expect("Error while loading map?"))
    // }

    pub fn get_chunk(&self, x: i32, y: i32) -> Result<Chunk, ()> {
        if !self.chunks.contains_key(&(x, y)) {
            Err(())
        } else {
            Ok(*self.chunks.get(&(x, y)).unwrap())
        }
    }

    pub fn load_chunk(&self, chunk_x: i32, chunk_y: i32) -> Option<Chunk> {
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

        for x in chunk_x_start - (camera.render_distance as i32)..=chunk_x_start +
            (camera.render_distance as i32) {
            for y in chunk_y_start - (camera.render_distance as i32)..=chunk_y_start +
                (camera.render_distance as i32) {
                let status = chunk_manager.load_chunk(x, y, self.seed);

                match status.get_chunk() {
                    Ok(chunk) => {
                        chunks.insert((x, y), Status::Visible(chunk));
                    }
                    Err(status) => {
                        match status {
                            Status::Visible(_) | Status::Ready(_) => {
                                // println!("Chunk {},{} became visible", x, y);
                            }
                            Status::Pending | Status::ToGenerate => {}
                            Status::Error(e) => panic!("{:?}", e),
                        }
                    }
                }
            }
        }
        chunks
    }
}
