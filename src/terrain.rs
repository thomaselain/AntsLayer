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
    data: Vec<Vec<TileType>>,
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
    pub fn new(width: usize, height: usize) -> Terrain {
        let tiles: Vec<Vec<TileType>> = vec![vec![TileType::AIR; height]; width];
        let pixel_buffer: Vec<u32> = vec![0; width * height]; // Initialise un buffer avec des pixels noirs (0)

        Terrain {
            data: tiles,
            pixel_buffer,
        }
    }

    pub fn generate(&mut self) {
        let noise_scale = 0.06;

        let minerals = vec![
            Mineral {
                r#type: MineralType::GOLD,
                occurence: 0.001,
                color: 0xFFD700,
            },
            Mineral {
                r#type: MineralType::IRON,
                occurence: 0.1,
                color: 0x454545,
            },
            Mineral {
                r#type: MineralType::ROCK,
                occurence: 0.7,
                color: 0x232323,
            },
        ];

        let perlin = Perlin::new(rand::random());

        for y in 0..HEIGHT as usize {
            for x in 0..WIDTH as usize {
                let nx = noise_scale * x as f64;
                let ny = noise_scale * y as f64;

                let mut tile_type = TileType::AIR; // Par défaut, définir comme AIR

                for m in &minerals {
                    let noise_value = perlin.get([nx, ny]);
                    let c = (noise_value + 1.0) / 2.0;
                    if c <= m.occurence {
                        tile_type = TileType::Mineral(m.r#type);
                        break; // Sortir de la boucle une fois qu'on a trouvé un minerai
                    }
                }
                self.data[x as usize][y as usize] = tile_type;

                let color = match self.data[x as usize][y as usize] {
                    TileType::AIR => 0xAAAAAA,   // Blanc pale
                    TileType::WATER => 0x0000FF, // Bleu
                    TileType::Mineral(_) => {
                        let mineral = minerals
                            .iter()
                            .find(|m| {
                                self.data[x as usize][y as usize] == TileType::Mineral(m.r#type)
                            })
                            .unwrap();
                        mineral.color
                    }
                };
                self.pixel_buffer[y * WIDTH as usize + x] = color;
            }
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        for y in 0..HEIGHT - 1 {
            for x in 0..WIDTH - 1 {
                let color = self.pixel_buffer[(y * WIDTH + x) as usize];

                let sdl_color = color_from_u32(color);
                canvas.set_draw_color(sdl_color);
                canvas
                    .fill_rect(Rect::new(x as i32, y as i32, 1, 1))
                    .unwrap();
            }
        }
    }
}
