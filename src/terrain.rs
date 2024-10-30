extern crate noise;
extern crate sdl2;

use sdl2::pixels::Color;

use noise::{NoiseFn, Perlin};
use rand::{self, Rng};
pub(crate) const TILE_SIZE: u32 = 1;

use crate::{
    automaton::{self, Automaton},
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
    pub r#type: TileType,
    pub automaton: Automaton,
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
                    r#type: TileType::Mineral(MineralType::GOLD),
                    color: 0xffff1cff,
                    automaton: Automaton::new(4, 6, 3, 0.05, 0.05, 1.0),
                },
                Mineral {
                    r#type: TileType::Mineral(MineralType::IRON),
                    color: 0xAAAAAAff,
                    automaton: Automaton::new(4, 5, 4, 0.05, 0.1, 1.0),
                },
                Mineral {
                    r#type: TileType::Mineral(MineralType::ROCK),
                    color: 0x303030FF,
                    automaton: Automaton::new(4, 4, 5, 0.035, 0.35, 1.0),
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

    pub fn check_data(&mut self, x: usize, y: usize) -> bool {
        if x < self.data.len() && y < self.data[x].len() {
            true
        } else {
            false
        }
    }

    pub fn generate_caves(&mut self, mineral: &Mineral) {
        let mut rng = rand::thread_rng();
        let noise: Perlin = Perlin::new(rng.gen());
        println!("Color = {:#X}", mineral.color); // Ajout pour le debug

        for x in 0..WIDTH as usize {
            for y in 0..HEIGHT as usize {
                if self.check_data(x, y) {
                    let noise_value = noise.get([
                        x as f64 * mineral.automaton.perlin_scale,
                        y as f64 * mineral.automaton.perlin_scale,
                    ]);
                    if noise_value.abs()
                        < mineral.automaton.perlin_threshold * mineral.automaton.occurence
                    {
                        self.data[x][y] = mineral.r#type;
                    }
                }
            }
        }
        // Application des règles de l'automate cellulaire
        mineral.automaton.apply_rules(self, mineral.r#type);
    }

    pub fn count_same_neighbors(&mut self, x: usize, y: usize, tile_type: TileType) -> usize {
        let mut count = 0;

        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if self.get_data(nx as usize, ny as usize) == Some(tile_type) {
                    count += 1;
                }
            }
        }
        count
    }

    fn clear_tiles(&mut self) {
        self.data = vec![vec![TileType::AIR; window::WIDTH as usize]; window::HEIGHT as usize];
    }

    pub fn generate(&mut self) {
        self.minerals.sort_by(|b, a| {
            b.automaton
                .occurence
                .partial_cmp(&a.automaton.occurence)
                .unwrap()
        });
        self.clear_tiles();

        let minerals_copy: Vec<Mineral> = self.minerals.clone();

        for m in minerals_copy {
            self.generate_caves(&m);
        }

        self.update_pixel_buffer();
    }

    fn update_pixel_buffer(&mut self) {
        for x in 0..WIDTH as usize {
            for y in 0..HEIGHT as usize {
                let color = match self.get_data(x, y) {
                    Some(TileType::AIR) => 0x2b180cff,
                    Some(TileType::WATER) => 0x0000FFFF,
                    Some(TileType::Mineral(_)) => {
                        let mineral = self
                            .minerals
                            .iter()
                            .find(|m| self.data[x as usize][y as usize] == m.r#type)
                            .unwrap();
                        mineral.color
                    }
                    None => 0x000000ff,
                };
                self.pixel_buffer[y * WIDTH as usize + x] = color;
            }
        }
    }

    pub fn draw(
        &mut self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        texture_creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>,
        camera: &Camera,
    ) {
        let mut texture = texture_creator
            .create_texture_streaming(
                sdl2::pixels::PixelFormatEnum::ARGB8888,
                window::WIDTH,
                window::HEIGHT,
            )
            .unwrap();

        // Mettre à jour la texture avec le contenu du pixel_buffer
        texture
            .with_lock(None, |buffer: &mut [u8], _pitch: usize| {
                for (i, pixel) in self.pixel_buffer.iter().enumerate() {
                    let color = *pixel;
                    buffer[i * 4 + 3] = ((color >> 0) & 0xFF) as u8; // A
                    buffer[i * 4 + 0] = ((color >> 8) & 0xFF) as u8; // R
                    buffer[i * 4 + 1] = ((color >> 16) & 0xFF) as u8; // G
                    buffer[i * 4 + 2] = ((color >> 24) & 0xFF) as u8; // B
                }
            })
            .unwrap();

        // Calculer la zone visible en fonction de la caméra et du zoom
        let start_x = camera.position.x as f32 / camera.zoom;
        let start_y = camera.position.y as f32 / camera.zoom;

        let viewport_width = (canvas.viewport().width() as f32 / camera.zoom) as u32;
        let viewport_height = (canvas.viewport().height() as f32 / camera.zoom) as u32;

        let dest_rect = sdl2::rect::Rect::new(
            start_x as i32,
            start_y as i32,
            viewport_width,
            viewport_height,
        );

        // Afficher uniquement la partie visible de la texture
        self.update_pixel_buffer();
        canvas.copy(&texture, None, dest_rect).unwrap();
    }
}
