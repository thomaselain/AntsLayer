use sdl2::{rect::Rect, render::WindowCanvas, video::Window, Sdl};

use crate::{
    camera::Camera,
    terrain::{Terrain, TileType, TILE_SIZE},
    units::Unit,
};
pub const WIDTH: u32 = 1200;
pub const HEIGHT: u32 = 900;

pub fn init_sdl2_window() -> (sdl2::Sdl, Window) {
    // Initialiser SDL2
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    // Créer une fenêtre avec SDL2
    (
        sdl_context,
        video_subsystem
            .window("AntsLayer", WIDTH, HEIGHT)
            .opengl()
            .resizable()
            .build()
            .unwrap(),
    )
}

pub enum BufferType {
    Terrain,
    Buildings,
    Units,
}

pub struct Buffer<BufferType> {
    pub buffer: Vec<u32>,
    pub needs_update: bool,
    pub typ: BufferType,
}

/// Access buffer
impl Buffer<BufferType> {
    pub fn new(b: Vec<u32>, needs_update: bool, t: BufferType) -> Buffer<BufferType> {
        Buffer::<BufferType> {
            buffer: b,
            needs_update: needs_update,
            typ: t,
        }
    }
    pub fn clear_buffer(&mut self) {
        self.buffer.fill(0);
    }
}

impl Buffer<BufferType> {
    pub fn draw_unit(&mut self, u: &Unit) {
        for x in u.coords.x..(TILE_SIZE as i32 + u.coords.x) {
            for y in u.coords.y..(TILE_SIZE as i32 + u.coords.y) {
                let pixel_x: usize = u.coords.x as usize + x as usize;
                let pixel_y: usize = u.coords.y as usize + y as usize;
                if x < WIDTH as i32 && y < HEIGHT as i32 {
                    let pixel_index: usize = y as usize * WIDTH as usize + x as usize as usize;
                    self.buffer[pixel_index] = u.color;
                    self.needs_update = true;
                }
            }
        }
    }
    pub fn draw_units(&mut self, units: &[Unit]) {
        self.buffer.fill(0);
        for u in units {
            self.draw_unit(u);
        }
    }

    pub fn draw_terrain(&mut self, terrain: &Terrain) {
        if !self.needs_update {
            //return;
        }

        for x in 0..WIDTH as usize - 1 {
            for y in 0..HEIGHT as usize - 1 {
                let color = match terrain.get_data(x, y) {
                    Some(TileType::AIR) => 0x2b180cff,
                    Some(TileType::WATER) => 0x0000FFFF,
                    Some(TileType::Mineral(_)) => {
                        let mineral = terrain
                            .minerals
                            .iter()
                            .find(|m| terrain.data[x][y] == m.r#type)
                            .unwrap();
                        mineral.color
                    }
                    None => 0x00000000,
                };
                let pixel_index: usize = y as usize * WIDTH as usize + x as usize as usize;
                self.buffer[pixel_index] = color;
        }
        }
        self.needs_update = false;
    }
}

pub struct Renderer {
    pub canvas: WindowCanvas,
    texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    pub terrain: Buffer<BufferType>,
    pub units: Buffer<BufferType>,
    pub buildings: Buffer<BufferType>,
}

impl Renderer {
    pub fn new((_, win): (Sdl, Window), width: usize, height: usize) -> Self {
        let canvas = win.into_canvas().present_vsync().build().unwrap();
        let texture_creator = canvas.texture_creator();

        Self {
            canvas,
            texture_creator,
            terrain: Buffer::<BufferType> {
                buffer: vec![0; width * height],
                needs_update: false,
                typ: BufferType::Terrain,
            },
            buildings: Buffer::<BufferType> {
                buffer: vec![0; width * height],
                needs_update: false,
                typ: BufferType::Buildings,
            },
            units: Buffer::<BufferType> {
                buffer: vec![0; width * height],
                needs_update: false,
                typ: BufferType::Units,
            },
        }
    }

    /* TODO
        pub fn draw_buildings(&mut self, buildings: &[Building]) {
            // Remplissage du buffer bâtiment avec les données de `buildings`
        }
    */
    fn combine_buffers(&self) -> Vec<u32> {
        let mut combined_buffer = vec![0x00000000; WIDTH as usize * HEIGHT as usize];

        for (i, &color) in self.terrain.buffer.iter().enumerate() {
            if color != 0x00000000 {
                // Couleur non transparente
                combined_buffer[i] = color;
            }
        }

        for (i, &color) in self.buildings.buffer.iter().enumerate() {
            if color != 0x00000000 {
                combined_buffer[i] = color;
            }
        }

        for (i, &color) in self.units.buffer.iter().enumerate() {
            if color != 0x00000000 {
                combined_buffer[i] = color;
            }
        }

        combined_buffer
    }

    pub fn draw(&mut self, camera: &Camera) {
        /*        // RESIZING (todo : dupe it for each buffer)
                if self.pixel_buffer.len() != (viewport_width * viewport_height) as usize {
                    self.pixel_buffer = vec![0; (viewport_width * viewport_height) as usize];
                    // Recréer le buffer
                }
        */
        let combined_buffers = self.combine_buffers();
        self.update_pixel_buffer(&combined_buffers, camera);
    }

    fn update_pixel_buffer(&mut self, pixel_buffer: &[u32], camera: &Camera) {
        // chose wich buffer to access

        if pixel_buffer.is_empty() {
            panic!("CACA");
        } // Dont show fully empty buffers
        let start_x = camera.position.x as f32;
        let start_y = camera.position.y as f32;

        let viewport_width = WIDTH as f32 / camera.zoom;
        let viewport_height = HEIGHT as f32 / camera.zoom;
        /* TODO : zoom handling
                let width = (WIDTH as f32 / camera.zoom) as usize;
                let height = (HEIGHT as f32 / camera.zoom) as usize;
        let mut texture = self
            .texture_creator
            .create_texture_target(None, WIDTH, HEIGHT)
            .expect("Failed to create texture");
        */

        let mut texture = self
            .texture_creator
            .create_texture_streaming(
                sdl2::pixels::PixelFormatEnum::RGBA8888,
                viewport_width as u32,
                viewport_height as u32,
            )
            .unwrap();

        texture
            .with_lock(None, |buffer: &mut [u8], _pitch: usize| {
                for (i, pixel) in pixel_buffer.iter().enumerate() {
                    let color = *pixel;
                    if color != 0x00000000 {
                        buffer[i * 4 + 0] = ((color >> 0) & 0xFF) as u8; // R
                        buffer[i * 4 + 1] = ((color >> 8) & 0xFF) as u8; // G
                        buffer[i * 4 + 2] = ((color >> 16) & 0xFF) as u8; // B
                        buffer[i * 4 + 3] = ((color >> 24) & 0xFF) as u8; // A
                    } else {
                        //println!("pixel at ({:?},{:?}) is empty", i / 4, i % 4)
                    }
                }
            })
            .unwrap();

        let dest_rect = Rect::new(
            start_x as i32,
            start_y as i32,
            viewport_width as u32,
            viewport_height as u32,
        );

        self.canvas.copy(&texture, None, dest_rect).unwrap();
        self.canvas.present();
    }
}
