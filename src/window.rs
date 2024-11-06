use sdl2::{rect::Rect, render::WindowCanvas, video::Window, Sdl};

use crate::{
    buildings::BuildingType, camera::Camera, terrain::{ Terrain, TileType}, units::Unit
};
pub const WIDTH: u32 = 1000;
pub const HEIGHT: u32 = 1000;

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
}

impl Buffer<BufferType> {
    pub fn draw_unit(&mut self, u: &Unit) {
        let x: usize = u.coords.x as usize;
        let y: usize = u.coords.y as usize;

        if x < WIDTH as usize && y < HEIGHT as usize {
            self.draw_tile(x, y, u.color);
        }
    }
    pub fn draw_units(&mut self, units: &[Unit]) {
        if !self.needs_update {
            //return;
        }
        self.buffer.fill(0);
        for u in units {
            self.draw_unit(u);
        }
    }

    pub fn draw_terrain(&mut self, terrain: &Terrain, camera: Camera) {
        let start_x = (camera.position.x as f32 / camera.zoom).max(0.0) as usize;
        let start_y = (camera.position.y as f32 / camera.zoom).max(0.0) as usize;
        let end_x = ((camera.position.x as f32 + camera.screen_width as f32) * camera.zoom)
            .min(WIDTH as f32) as usize;
        let end_y = ((camera.position.y as f32 + camera.screen_height as f32) * camera.zoom)
            .min(HEIGHT as f32) as usize;

        for x in start_x..end_x {
            for y in start_y..end_y {
                let color = match terrain.get_data(x, y) {
                    Some(TileType::AIR) => 0x101010ff,
                    Some(TileType::WATER) => 0x0000FFFF,
                    Some(TileType::Mineral(_)) => {
                        let mineral = terrain
                            .minerals
                            .iter()
                            .find(|m| terrain.data[x][y] == m.r#type)
                            .unwrap();
                        mineral.color
                    },
                    Some(TileType::Building(t)) => { match t {
                        BuildingType::Hearth => 0xe36505FF,
                        BuildingType::Stockpile => 0x064f28FF,
                    } },
                    None => 0x00000000,
                };
                self.draw_tile(x, y, color);
            }
        }
    }

    fn draw_tile(&mut self, x: usize, y: usize, color: u32) {
        let pixel_index: usize = y * WIDTH as usize + x;
        self.buffer[pixel_index] = color;
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
                needs_update: true,
                typ: BufferType::Terrain,
            },
            buildings: Buffer::<BufferType> {
                buffer: vec![0; width * height],
                needs_update: true,
                typ: BufferType::Buildings,
            },
            units: Buffer::<BufferType> {
                buffer: vec![0; width * height],
                needs_update: true,
                typ: BufferType::Units,
            },
        }
    }

    /* TODO
        pub fn draw_buildings(&mut self, buildings: &[Building]) {
            // Remplissage du buffer bâtiment avec les données de `buildings`
        }
    */
    pub fn all_need_update(&mut self) {
        self.terrain.needs_update = true;
        self.buildings.needs_update = true;
        self.units.needs_update = true;
    }

    fn combine_buffers(&self) -> Vec<u32> {
        let mut combined_buffer = vec![0x00000000; WIDTH as usize * HEIGHT as usize];

        if self.terrain.needs_update {
            for (i, &color) in self.terrain.buffer.iter().enumerate() {
                if color != 0x00000000 {
                    // Couleur non transparente
                    combined_buffer[i] = color;
                }
            }
        }

        if self.buildings.needs_update {
            for (i, &color) in self.buildings.buffer.iter().enumerate() {
                if color != 0x00000000 {
                    combined_buffer[i] = color;
                }
            }
        }

        if self.units.needs_update {
            for (i, &color) in self.units.buffer.iter().enumerate() {
                if color != 0x00000000 {
                    combined_buffer[i] = color;
                }
            }
        }

        combined_buffer
    }

    pub fn draw(&mut self, terrain: &Terrain, units: &Vec<Unit>, camera: &Camera) {
        /*        // RESIZING (todo : dupe it for each buffer)
                if self.pixel_buffer.len() != (viewport_width * viewport_height) as usize {
                    self.pixel_buffer = vec![0; (viewport_width * viewport_height) as usize];
                    // Recréer le buffer
                }
        */
        self.terrain.draw_terrain(&terrain, *camera);
        self.units.draw_units(&units);

        let combined_buffers = self.combine_buffers();
        self.update_pixel_buffer(&combined_buffers, camera);
        self.canvas.present();
    }

    fn update_pixel_buffer(&mut self, pixel_buffer: &[u32], camera: &Camera) {
        if pixel_buffer.is_empty() {
            panic!("Invalid Bufffer !!!");
        }

        let viewport_width = (WIDTH as f32 * camera.zoom).ceil() as u32;
        let viewport_height = (HEIGHT as f32 * camera.zoom).ceil() as u32;
    
        let mut texture = self
            .texture_creator
            .create_texture_streaming(
                sdl2::pixels::PixelFormatEnum::RGBA8888,
                viewport_width,
                viewport_height,
            )
            .unwrap();

        texture
            .with_lock(None, |buffer: &mut [u8], _pitch: usize| {
                for x in 0..viewport_width as usize - 1 {
                    for y in 0..viewport_height as usize - 1 {
                        let src_x = ((camera.position.x as f32 * camera.zoom) + x as f32) as usize;
                        let src_y = ((camera.position.y as f32 * camera.zoom) + y as f32) as usize;
    
                        if src_x < WIDTH as usize
                            && src_y < HEIGHT as usize
                            && src_x > 0
                            && src_y > 0
                        {
                            let pixel_index = src_y * WIDTH as usize + src_x;
                            let color = pixel_buffer[pixel_index];

                            let dest_index = (y * viewport_width as usize + x) * 4;
                            buffer[dest_index + 0] = ((color >> 0) & 0xFF) as u8; // R
                            buffer[dest_index + 1] = ((color >> 8) & 0xFF) as u8; // G
                            buffer[dest_index + 2] = ((color >> 16) & 0xFF) as u8; // B
                            buffer[dest_index + 3] = ((color >> 24) & 0xFF) as u8;
                            // A
                        }
                    }
                }
            })
            .unwrap();

        let dest_rect = Rect::new(0, 0, WIDTH, HEIGHT);

        self.canvas.copy(&texture, None, dest_rect).unwrap();
    }
}
