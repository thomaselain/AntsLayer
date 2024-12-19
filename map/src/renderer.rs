use chunk::{ Chunk, CHUNK_SIZE };
use chunk_manager::{ ChunkManager, Draw, DrawAll };
use sdl2::{ pixels::Color, rect::Rect, Sdl };
use tile::{ FluidType, TileType };

use crate::{ camera::Camera, Map };

pub const TILE_SIZE: i32 = 10;

pub fn tile_screen_coords(
    x_chunk: i32,
    y_chunk: i32,
    x: usize,
    y: usize,
    offset_x: i32,
    offset_y: i32
) -> (i32, i32) {
    let rect_x = (x_chunk * (CHUNK_SIZE as i32) + (x as i32)) * TILE_SIZE - offset_x;
    let rect_y = (y_chunk * (CHUNK_SIZE as i32) + (y as i32)) * TILE_SIZE - offset_y;
    (rect_x, rect_y)
}

pub struct Renderer {
    pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
}

impl Renderer {
    pub fn new(sdl: &Sdl, title: &str, width: u32, height: u32) -> Result<Self, String> {
        let video_subsystem = sdl.video()?;
        let window = video_subsystem
            .window(title, width, height)
            .position_centered()
            .resizable()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())?;

        Ok(Renderer { canvas })
    }
    pub fn get_window_size(&self) -> (u32, u32) {
        self.canvas.output_size().expect("Failed to get window size")
    }
}

impl DrawAll<Map, Renderer, Camera> for ChunkManager {
    fn draw_all(&mut self, map: &mut Map, renderer: &mut Renderer, camera: &Camera) {
        // map.generate_visible_chunks(camera, self);
        for (_, chunk) in &map.chunks {
            chunk.draw(renderer, camera);
        }
        renderer.canvas.set_draw_color(Color::BLACK);
    }
}

impl Draw<Renderer, Camera> for ChunkManager {
    fn draw(&self, renderer: &mut Renderer, camera: &Camera) {
        for (_, status) in self.loaded_chunks.clone() {
            if let Ok(chunk) = status.clone().get_chunk() {
                chunk.draw(renderer, camera);
            }
        }
        renderer.canvas.set_draw_color(Color::BLACK);
    }
}

impl Draw<Renderer, Camera> for Chunk {
    fn draw(&self, renderer: &mut Renderer, camera: &Camera) {
        let (window_width, window_height) = renderer.get_window_size();
        let (offset_x, offset_y) = camera.get_offset(window_width, window_height);

        for (x, row) in self.tiles.clone().iter().enumerate() {
            for (y, tile) in row.iter().enumerate() {
                let (screen_x, screen_y) = tile_screen_coords(
                    self.x,
                    self.y,
                    x,
                    y,
                    offset_x,
                    offset_y
                );

                let color = match tile.tile_type {
                    TileType::Floor => Color::WHITE,
                    TileType::Fluid(liquid) =>
                        match liquid {
                            FluidType::Magma => Color::BLUE,
                            FluidType::Water => Color::RED,
                        }
                    TileType::Wall => Color::GRAY,
                    TileType::Empty => Color::BLACK,
                    TileType::Rock => Color::GREY,
                    TileType::Dirt => Color::RGB(50, 200, 25),
                    TileType::Grass => Color::GREEN,
                    TileType::Custom(_) => todo!(),
                    // _ => Color::WHITE,
                };

                renderer.canvas.set_draw_color(color);
                renderer.canvas
                    .fill_rect(Rect::new(screen_x, screen_y, TILE_SIZE as u32, TILE_SIZE as u32))
                    .expect("Failed to draw tile");
            }
        }
    }
}

#[cfg(test)]
mod tests {}
