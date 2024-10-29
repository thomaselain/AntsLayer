extern crate noise;
extern crate sdl2;

use noise::{NoiseFn, Perlin};

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use rand::{self, Rng};

use crate::window;

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
    pub occurence: f64,
    pub color: u32,
}

#[derive(Clone)]
pub struct Terrain {
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

    pub fn generate_caves(&mut self, fill_percentage: f64, iterations: usize) {
        let mut rng = rand::thread_rng();

        // Remplissage initial avec des roches aléatoires
        for x in 0..window::WIDTH as usize {
            for y in 0..window::HEIGHT as usize {
                if self.check_data(x, y) {
                    self.data[x][y] = if rng.gen_bool(fill_percentage) {
                        TileType::Mineral(MineralType::ROCK)
                    } else {
                        TileType::AIR
                    };
                }
            }
        }

        // Application des règles de l'automate cellulaire
        for _ in 0..iterations {
            let mut new_data = self.data.clone();

            for x in 1..(window::WIDTH as usize) {
                for y in 1..(window::HEIGHT as usize) {
                    let count = self.count_solid_neighbors(x, y);
                    if self.check_data(x, y) {
                        new_data[x][y] = if count >= 5 {
                            TileType::Mineral(MineralType::ROCK)
                        } else {
                            TileType::AIR
                        };
                    }
                }
            }
            self.data = new_data;
        }
    }

    fn count_solid_neighbors(&mut self, x: usize, y: usize) -> usize {
        let mut count = 0;
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if self.check_data(nx as usize, ny as usize) {
                    if let TileType::Mineral(_) = self.data[nx as usize][ny as usize] {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    pub fn generate(&mut self) {
        let minerals = vec![
            Mineral {
                r#type: MineralType::GOLD,
                occurence: 0.00001,
                color: 0xFFD700,
            },
            Mineral {
                r#type: MineralType::IRON,
                occurence: 0.1,
                color: 0x454545,
            },
            Mineral {
                r#type: MineralType::ROCK,
                occurence: 0.5,
                color: 0x232323,
            },
        ];
        self.generate_caves(0.6, 3);
        for x in 0..window::WIDTH as usize {
            for y in 0..window::HEIGHT as usize {
                let color = match self.get_data(x, y) {
                    Some(TileType::AIR) => 0xAAAAAA,   // Blanc pale
                    Some(TileType::WATER) => 0x0000FF, // Bleu
                    Some(TileType::Mineral(_)) => {
                        let mineral = minerals
                            .iter()
                            .find(|m| {
                                self.data[x as usize][y as usize] == TileType::Mineral(m.r#type)
                            })
                            .unwrap();
                        mineral.color
                    },
                    None => 0x000000,
                };
                self.pixel_buffer[x * window::HEIGHT as usize + y] = color;

            }
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        for x in 0..window::WIDTH {
            for y in 0..window::HEIGHT {
                let index = x * window::HEIGHT + y;
                let mut color: u32 = 0x000000;

                if (index as usize) < self.pixel_buffer.len() {
                    color = self.pixel_buffer[index as usize];
                }
                let sdl_color = color_from_u32(color);
                canvas.set_draw_color(sdl_color);
                canvas
                    .fill_rect(Rect::new(x as i32, y as i32, 1, 1))
                    .unwrap();
            }
        }
    }
}
