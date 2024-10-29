extern crate noise;
extern crate sdl2;

use sdl2::pixels::Color;

use rand::{self, Rng};
pub(crate) const TILE_SIZE: u32 = 3;

use crate::{
    camera::{self, Camera},
    window::{self, HEIGHT, WIDTH},
    Coords::{self},
};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MineralType {
    IRON,
    GOLD,
    ROCK,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TileType {
    Mineral(MineralType),
    AIR,
    WATER,
}

#[derive(Copy, Clone)]
pub struct Mineral {
    pub r#type: MineralType,
    pub iterations: u32,
    pub occurence: f64,
    pub color: u32,
}

#[derive(Clone)]
pub struct Terrain {
    pub minerals: Vec<Mineral>,
    pub data: Vec<Vec<TileType>>,
    pixel_buffer: Vec<u32>,
}

// Custom function to use Color::from(hexa value)
fn color_from_u32(hex: u32) -> Color {
    let r = ((hex >> 16) & 0xFF) as u8;
    let g = ((hex >> 8) & 0xFF) as u8;
    let b = (hex & 0xFF) as u8;
    Color::RGB(r, g, b)
}

impl Terrain {
    pub fn new() -> Terrain {
        let tiles: Vec<Vec<TileType>> =
            vec![vec![TileType::AIR; window::WIDTH as usize]; window::HEIGHT as usize];
        let pixel_buffer: Vec<u32> = vec![0; window::WIDTH as usize * window::HEIGHT as usize]; // Initialise un buffer avec des pixels noirs (0)

        Terrain {
            data: tiles,
            pixel_buffer,
            minerals: vec![
                Mineral {
                    r#type: MineralType::GOLD,
                    occurence: 0.0,
                    iterations: 1,
                    color: 0xffff1c,
                },
                Mineral {
                    r#type: MineralType::IRON,
                    occurence: 0.0,
                    iterations: 1,
                    color: 0xCCCCCC,
                },
                Mineral {
                    r#type: MineralType::ROCK,
                    occurence: 0.8,
                    iterations: 2,
                    color: 0x000000,
                },
            ],
        }
    }

    pub fn get_data(&mut self, x: usize, y: usize) -> Option<TileType> {
        if self.check_data(x, y) {
            Some(self.data[x][y])
        } else {
            None
        }
    }

    fn check_data(&mut self, x: usize, y: usize) -> bool {
        if x < self.data.len() && y < self.data[x].len() {
            true
        } else {
            false
        }
    }

    pub fn generate_caves(&mut self, mineral: &Mineral) {
        let mut rng = rand::thread_rng();

        // Remplissage initial avec des roches aléatoires
        for x in 0..window::WIDTH as usize {
            for y in 0..window::HEIGHT as usize {
                if self.check_data(x, y) {
                    if rng.gen_bool(mineral.occurence) {
                        self.data[x][y] = TileType::Mineral(mineral.r#type)
                    }
                }
            }
        }

        // Application des règles de l'automate cellulaire
        for _ in 0..mineral.iterations {
            let mut new_data = self.data.clone();

            for x in 1..(window::WIDTH as usize) {
                for y in 1..(window::HEIGHT as usize) {
                    let count = self.count_same_neighbors(x, y);
                    if self.get_data(x, y) == Some(TileType::AIR) && rng.gen_bool(mineral.occurence)
                    {
                        if count >= 5 {
                            new_data[x][y] = TileType::Mineral(mineral.r#type)
                        } else if count < 4 {
                            new_data[x][y] = TileType::AIR
                        }
                    }
                }
            }
            self.data = new_data;
        }
    }

    fn count_same_neighbors(&mut self, x: usize, y: usize) -> usize {
        let mut count = 0;
        let tile_type = self.get_data(x, y);

        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if self.get_data(nx as usize, ny as usize) == tile_type {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn generate(&mut self) {
        self.minerals
            .sort_by(|a, b| a.occurence.partial_cmp(&b.occurence).unwrap());
        let minerals_copy = self.minerals.clone();

        for m in minerals_copy {
            self.generate_caves(&m);
        }

        for x in 0..window::WIDTH as usize {
            for y in 0..window::HEIGHT as usize {
                let color = match self.get_data(x, y) {
                    Some(TileType::AIR) => 0xAAAAAA,   // Blanc pale
                    Some(TileType::WATER) => 0x0000FF, // Bleu
                    Some(TileType::Mineral(_)) => {
                        let mineral = self
                            .minerals
                            .iter()
                            .find(|m| {
                                self.data[x as usize][y as usize] == TileType::Mineral(m.r#type)
                            })
                            .unwrap();
                        mineral.color
                    }
                    None => 0x000000,
                };
                self.pixel_buffer[x * window::HEIGHT as usize + y] = color;
            }
        }
    }

    pub fn draw(
        &mut self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        camera: &Camera,
    ) {
        // Calculer les limites de la vue en fonction de la caméra et du zoom
        let start_x = (camera.position.x as f32 / camera.tile_size as f32) as usize;
        let start_y = (camera.position.y as f32 / camera.tile_size as f32) as usize;
        let end_x = ((camera.position.x as f32 + canvas.viewport().width() as f32)
            / camera.tile_size as f32)
            .ceil() as usize;
        let end_y = ((camera.position.y as f32 + canvas.viewport().height() as f32)
            / camera.tile_size as f32)
            .ceil() as usize;

        let end_x = end_x.min(WIDTH as usize);
        let end_y = end_y.min(HEIGHT as usize);

        for y in start_y..end_y {
            for x in start_x..end_x {
                let tile_coords = Coords {
                    x: x as i32,
                    y: y as i32,
                };

                let color = match self.get_data(x, y) {
                    Some(TileType::AIR) => 0x303030,   // Gris (le sol de la caverne)
                    Some(TileType::WATER) => 0x0000FF, // Bleu
                    Some(TileType::Mineral(_)) => {
                        let mineral = self
                            .minerals
                            .iter()
                            .find(|m| self.data[x][y] == TileType::Mineral(m.r#type))
                            .unwrap();
                        mineral.color
                    }
                    None => 0x000000,
                };

                let draw_x: f32 = start_x as f32
                    + (tile_coords.x as f32 * TILE_SIZE as f32 * camera.zoom)
                    - camera.position.x as f32;
                let draw_y: f32 = start_y as f32
                    + (tile_coords.y as f32 * TILE_SIZE as f32 * camera.zoom)
                    - camera.position.y as f32;

                canvas.set_draw_color(color_from_u32(color));

                canvas
                    .fill_rect(sdl2::rect::Rect::new(
                        draw_x as i32,
                        draw_y as i32,
                        TILE_SIZE as u32,
                        TILE_SIZE as u32,
                    ))
                    .unwrap();
            }
        }
    }
}
