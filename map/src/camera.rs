use std::{ collections::HashMap, sync::mpsc, time::Duration };

use biomes::BiomeConfig;
use chunk::{ thread::Status, Chunk, ChunkPath, CHUNK_SIZE };
use chunk_manager::{ ChunkManager, Clear, Update };
use coords::Coords;

use crate::{ renderer::TILE_SIZE, Directions, Map };

const DEFAULT_RENDER_DISTANCE: usize = 3;
const DEFAULT_SPEED: f32 = 0.6;
const DEFAULT_ZOOM: f32 = 1.0;

pub struct Camera {
    pub render_distance: usize,
    pub coords: Coords<f32>,
    pub speed: f32,
    pub zoom: f32,
}

impl Camera {
    pub fn new(x: f32, y: f32) -> Self {
        Camera {
            render_distance: DEFAULT_RENDER_DISTANCE,
            coords: Coords::new(x, y),
            speed: DEFAULT_SPEED,
            zoom: DEFAULT_ZOOM,
        }
    }

    /// Calcule le décalage de rendu pour la caméra en pixels
    pub fn get_offset(&self, window_width: u32, window_height: u32) -> (i32, i32) {
        let half_width = (window_width as i32) / 2;
        let half_height = (window_height as i32) / 2;

        let offset_x = ((self.coords.x() * (TILE_SIZE as f32)) as i32) - half_width;
        let offset_y = ((self.coords.y() * (TILE_SIZE as f32)) as i32) - half_height;

        (offset_x, offset_y)
    }

    /// Centre la caméra autour d'une position donnée
    pub fn center_on(&mut self, x: i32, y: i32) {
        self.coords = Coords::new(
            (x as f32) * (CHUNK_SIZE as f32),
            (y as f32) * (CHUNK_SIZE as f32)
        );
    }

    /// Déplace la caméra
    pub fn move_dir(&mut self, dir: Directions) {
        match dir {
            Directions::North => self.move_by(0.0, (-TILE_SIZE as f32) * self.speed), // Haut
            Directions::South => self.move_by(0.0, (TILE_SIZE as f32) * self.speed), // Bas
            Directions::West => self.move_by((-TILE_SIZE as f32) * self.speed, 0.0), // Gauche
            Directions::East => self.move_by((TILE_SIZE as f32) * self.speed, 0.0), // Droite
        }
    }

    /// Déplace la caméra
    pub fn move_by(&mut self, dx: f32, dy: f32) {
        self.coords = self.coords + Coords::new(dx / (TILE_SIZE as f32), dy / (TILE_SIZE as f32));
    }
}

impl Clear<&Map, Camera> for ChunkManager {
    fn clear_out_of_range(&mut self, visible_chunks: HashMap<(i32, i32), Status>) {
        // Retirer les chunks hors de portée
        self.chunks.retain(|&(x, y), _| visible_chunks.contains_key(&(x, y)));
    }
}

impl Update<Map, Camera> for ChunkManager {
    fn update(&mut self, map: &mut Map, camera: &Camera) {
        let (sender, receiver) = mpsc::channel();
        let visible_chunks: HashMap<(i32, i32), Status> = map.visible_chunks(camera, self);

        for ((x, y), _) in visible_chunks.iter() {
            let path = ChunkPath::build(map.path.clone(), *x, *y).expect("Failed to create chunks folder");

            match Chunk::load(path) {
                Ok(((x, y), status)) => {
                    self.chunks.insert((x, y), status);
                }
                Err(e) => panic!("{:?}", e),
            }
        }

        for ((x, y), status) in self.chunks.clone() {
            if status == Status::ToGenerate {
                Chunk::generate_async(x, y, map.seed, BiomeConfig::default(), sender.clone());

                let ((x, y), status) = receiver
                    .recv_timeout(Duration::new(1, 0))
                    .expect("Chunk took too long to generate");

                self.chunks.insert((x, y), status);
            }
        }

        self.clear_out_of_range(visible_chunks);
    }
}
