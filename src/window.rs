use rand::{self, Rng};
use rusttype::{point, Font, PositionedGlyph, Scale};
use sdl2::pixels::PixelFormatEnum;
use sdl2::surface::Surface;
use sdl2::{rect::Rect, render::WindowCanvas, video::Window, Sdl};

use crate::minerals::{Mineral, MineralType};
use crate::terrain::{Terrain, Tile, TileType};
use crate::{buildings::BuildingType, camera::Camera, units::Unit};
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
            return;
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
                let mut color = MineralType::ROCK.color() & 0xff;

                match terrain.get_tile(x, y) {
                    Some(tile) => {
                        match tile {
                            Tile(_, Some(Mineral(mineral_type)), _) => {
                                color |= mineral_type.color();
                            }
                            _ => {}
                        }
                        // println!("{:X?}",mineral_type.0.color());
                    }
                    None => {
                        color = 0x00000000;
                    }
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
                    combined_buffer[i] |= color;
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

    pub fn draw(&mut self, terrain: Terrain, units: Vec<Unit>, camera: &Camera) {
        /*        // RESIZING (todo : dupe it for each buffer)
                if self.pixel_buffer.len() != (viewport_width * viewport_height) as usize {
                    self.pixel_buffer = vec![0; (viewport_width * viewport_height) as usize];
                    // Recréer le buffer
                }
        */
        self.units.draw_units(&units);
        self.terrain.draw_terrain(&terrain, *camera);

        let combined_buffers = self.combine_buffers();
        self.update_pixel_buffer(&combined_buffers, camera);
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

    pub fn render_text(&mut self, text: &str, x: i32, y: i32) -> Result<(), String> {
        // Charge la police OTF depuis le dossier 'fonts'
        let font_data = std::fs::read("fonts/roboto-3/RobotoCondensed-Regular.ttf")
            .map_err(|e| format!("Failed to read font file: {}", e))?;
        let font =
            Font::try_from_bytes(&font_data).ok_or_else(|| "Failed to load font".to_string())?;

        // Configure la taille du texte
        let scale = Scale::uniform(25.0);

        // Calcule la largeur et la hauteur totale du texte
        let mut glyphs = Vec::new();
        let mut max_height = 100;
        let mut max_width = 0;

        for glyph in font.layout(text, scale, point(25.0, 25.0)) {
            if let Some(bb) = glyph.pixel_bounding_box() {
                max_height = max_height.max(bb.max.y);
                max_width = max_width.max(bb.max.x);
            }
            glyphs.push(glyph);
        }

        // Crée une surface avec la bonne taille en RGBA8888
        let mut surface = Surface::new(
            max_width as u32,
            max_height as u32,
            PixelFormatEnum::RGBA8888,
        )
        .map_err(|e| format!("Failed to create surface: {}", e))?;

        // Positionne et dessine chaque glyphe
        let mut pixel_data = vec![0u8; (max_width * max_height * 4) as usize]; // Buffer pour les pixels (RGBA8888)

        let mut x_offset = 0;
        let y_offset = 0;

        for glyph in glyphs.iter().rev() {
            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                glyph.draw(|gx, gy, v| {
                    let x = gx as i32 + bounding_box.min.x + x_offset;
                    let y = gy as i32 + bounding_box.min.y + y_offset;
                    if x >= 0 && y >= 0 && x < max_width && y < max_height {
                        let pixel_index = ((y as usize) * max_width as usize + x as usize) * 4;
                        let intensity = (v * 255.0) as u8; // Applique l'intensité pour l'alphabétisation
                        pixel_data[pixel_index] = intensity; // Rouge
                        pixel_data[pixel_index + 1] = intensity; // Vert
                        pixel_data[pixel_index + 2] = intensity; // Bleu
                        pixel_data[pixel_index + 3] = 255; // Alpha complet
                    }
                });
                x_offset -= bounding_box.width() as i32; // Ajuste la position en X après chaque glyphe
            }
        }

        // Mets à jour la surface avec les nouveaux pixels
        surface.with_lock_mut(|buffer: &mut [u8]| {
            buffer.copy_from_slice(&pixel_data);
        });

        // Crée une texture à partir de la surface
        let texture_creator = self.canvas.texture_creator();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| format!("Failed to create texture: {}", e))?;

        // Dessine la texture à l'écran à la position spécifiée
        self.canvas.copy(
            &texture,
            None,
            Some(sdl2::rect::Rect::new(
                x,
                y,
                max_width as u32,
                max_height as u32,
            )),
        )?;

        Ok(())
    }
}
