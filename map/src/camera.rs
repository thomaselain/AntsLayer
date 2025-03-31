use chunk::{ CHUNK_HEIGHT, CHUNK_WIDTH };
use coords::Coords;

use crate::{ renderer::TILE_SIZE, Directions };

const DEFAULT_RENDER_DISTANCE: usize = 3;
const DEFAULT_SPEED: f32 = 10.0;
const DEFAULT_ZOOM: f32 = 1.0;

pub struct Camera {
    pub render_distance: usize,
    pub coords: Coords<f32>,
    pub speed: f32,
    pub zoom: f32,
}

impl Camera {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Camera {
            render_distance: DEFAULT_RENDER_DISTANCE,
            coords: Coords::new(x, y, z),
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
    pub fn center_on(&mut self, x: i32, y: i32, z: i32) {
        self.coords = Coords::new(
            (x as f32) * (CHUNK_WIDTH as f32),
            (y as f32) * (CHUNK_WIDTH as f32),
            (z as f32) * (CHUNK_HEIGHT as f32)
        );
    }

    /// Déplace la caméra
    pub fn move_dir(&mut self, dir: Directions) {
        match dir {
            Directions::Up => self.move_by(0.0, 0.0, 1.0),
            Directions::Down => self.move_by(0.0, 0.0, -1.0),

            Directions::North => self.move_by(0.0, -self.speed, 0.0),
            Directions::South => self.move_by(0.0, self.speed, 0.0),
            Directions::West => self.move_by(-self.speed, 0.0, 0.0),
            Directions::East => self.move_by(self.speed, 0.0, 0.0),
        }
    }

    /// Déplace la caméra
    pub fn move_by(&mut self, dx: f32, dy: f32, dz: f32) {
        self.coords =
            self.coords +
            Coords::new(dx / (TILE_SIZE as f32), dy / (TILE_SIZE as f32), dz / (TILE_SIZE as f32));
    }
}
