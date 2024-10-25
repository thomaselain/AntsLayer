extern crate noise;
extern crate sdl2;

use noise::{NoiseFn, Perlin};

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use rand;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MineralType {
    IRON,
    GOLD,
    ROCK,
}

#[derive(Copy, Clone)]
pub enum TileType {
    Mineral(MineralType),
    AIR,
    WATER,
}

#[derive(Copy, Clone)]
pub struct Mineral {
    pub r#type: MineralType,
    pub rarity: f64,
    pub color: Color,
}

#[derive(Clone)]
pub struct Terrain {
    data: Vec<Vec<TileType>>,
}

impl Terrain {
    pub fn new(width: usize, height: usize) -> Terrain {
        let tiles: Vec<Vec<TileType>> = vec![vec![TileType::AIR; height]; width];

        Terrain { data: tiles }
    }

    pub fn generate(&mut self) {
        let noise_level = 125.0;
        let noise_scale = 0.045;
        let perlin = Perlin::new(rand::random());

        let minerals = vec![
            Mineral {
                r#type: MineralType::IRON,
                rarity: 50.0,
                color: Color::RGB(169, 169, 169),
            },
            Mineral {
                r#type: MineralType::GOLD,
                rarity: 25.0,
                color: Color::RGB(255, 215, 0),
            },
            Mineral {
                r#type: MineralType::ROCK,
                rarity: 170.0,
                color: Color::RGB(105, 105, 105),
            },
        ];

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let nx = noise_scale * x as f64;
                let ny = noise_scale * y as f64;

                let c = noise_level * perlin.get([nx, ny]);
                for mineral in &minerals {
                    if (mineral.r#type == MineralType::IRON && c < mineral.rarity / 2.0)
                        || c <= mineral.rarity
                    {
                        if x < WIDTH && y < HEIGHT {
                            self.data[y as usize][x as usize] = TileType::Mineral(mineral.r#type);
                        }
                    }
                }
            }
        }
    }

    fn store_pixel(&self, canvas: &mut Canvas<Window>, tile_type: TileType, x: u32, y: u32) {
        let color = {
            match tile_type {
                TileType::AIR => Color::RGB(150, 150, 150),
                TileType::WATER => Color::BLUE,
                _ => Color::BLACK, /*           Mineral{} => Color::RGB(169, 169, 169), // Iron color
                                            MineralType::GOLD => Color::RGB(255, 215, 0),   // Gold color
                                            MineralType::ROCK => Color::RGB(105, 105, 105), // Rocks color
                                   */
            }
        };
        canvas.set_draw_color(color);
        canvas
            .fill_rect(Rect::new(x as i32, y as i32, 1, 1))
            .unwrap();
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                self.store_pixel(canvas, self.data[y as usize][x as usize], x, y);
            }
        }
    }
}
