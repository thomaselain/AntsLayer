use chunk::{ Chunk, CHUNK_SIZE };
use chunk_manager::{ ChunkManager, Draw, DrawAll };
use coords::aliases::TilePos;
use sdl2::{ pixels::Color, rect::Rect, Sdl };
use tile::{ FluidType, TileType };

use crate::{ camera::Camera, Map };

pub const TILE_SIZE: i32 = 2;

pub fn tile_screen_coords(
    chunk_key: TilePos,
    x: usize,
    y: usize,
    offset_x: i32,
    offset_y: i32
) -> TilePos {
    TilePos::new(
        (chunk_key.x() * (CHUNK_SIZE as i32) + (x as i32)) * TILE_SIZE - offset_x,
        (chunk_key.y() * (CHUNK_SIZE as i32) + (y as i32)) * TILE_SIZE - offset_y
    )
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
        for key in map.chunks.keys() {
            let status = self.loaded_chunks.get(&key);

            if let Some(status) = status {
                match status.clone().get_chunk() {
                    Ok(chunk) => chunk.draw(renderer, camera),
                    Err(e) => {
                        eprintln!("{:?}", e);
                    }
                }
            }
        }
        renderer.canvas.set_draw_color(Color::BLACK);
    }
}

impl Draw<Renderer, Camera> for ChunkManager {
    fn draw(&self, renderer: &mut Renderer, camera: &Camera) {
        for key in self.visible_chunks.clone() {
            let status = self.loaded_chunks.get(&key);

            if let Some(status) = status {
                match status.clone().get_chunk() {
                    Ok(chunk) => {
                        chunk.draw(renderer, camera);
                    }
                    Err(e) => {
                        eprintln!("{:?}", e);
                    }
                }
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
                let screen_coords = tile_screen_coords(self.key, x, y, offset_x, offset_y);

                let color = match tile.tile_type {
                    TileType::Fluid(liquid) =>
                        match liquid {
                            FluidType::Magma => Color::RED,
                            FluidType::Water => Color::BLUE,
                        }
                    TileType::Empty => Color::RGB(10, 10, 10),
                    TileType::Sand => Color::RGB(255, 255, 100),
                    TileType::Wall => Color::GRAY,
                    TileType::Rock => Color::GREY,
                    TileType::Dirt => Color::RGB(50, 220, 50),
                    TileType::Grass => Color::RGB(75, 255, 75),
                    TileType::Custom(_) => Color::CYAN,
                    _ => Color::MAGENTA,
                };

                renderer.canvas.set_draw_color(color);
                renderer.canvas
                    .fill_rect(
                        Rect::new(
                            screen_coords.x(),
                            screen_coords.y(),
                            TILE_SIZE as u32,
                            TILE_SIZE as u32
                        )
                    )
                    .expect("Failed to draw tile");
            }
        }
    }
}

#[cfg(test)]
mod tests {}
