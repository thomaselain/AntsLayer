use chunk::{ thread::Status, CHUNK_SIZE };
use chunk_manager::{ ChunkManager, Draw, DrawAll };
use sdl2::{ pixels::Color, rect::Rect, Sdl };
use tile::TileType;

use crate::{ camera::Camera, Map };

pub const TILE_SIZE: i32 = 3;

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
        let (window_width, window_height) = renderer.get_window_size();
        let (offset_x, offset_y) = camera.get_offset(window_width, window_height);

        map.generate_visible_chunks(camera, self);

        for (&(chunk_x, chunk_y), chunk) in &map.chunks {
            let chunk_screen_x = chunk_x * (CHUNK_SIZE as i32) * (TILE_SIZE as i32);
            let chunk_screen_y = chunk_y * (CHUNK_SIZE as i32) * (TILE_SIZE as i32);

            // Check if current_chunk is done generating
            //Draw each tile
            for (x, row) in chunk.tiles.iter().enumerate() {
                for (y, tile) in row.iter().enumerate() {
                    let (screen_x, screen_y) = (
                        chunk_screen_x + (x as i32) * (TILE_SIZE as i32) - offset_x,
                        chunk_screen_y + (y as i32) * (TILE_SIZE as i32) - offset_y,
                    );

                    let color = match tile.tile_type {
                        TileType::Floor => Color::GREEN,
                        TileType::Liquid => Color::BLUE,
                        TileType::Wall => Color::GRAY,
                        _ => Color::WHITE,
                    };

                    renderer.canvas.set_draw_color(color);
                    renderer.canvas
                        .fill_rect(
                            Rect::new(screen_x, screen_y, TILE_SIZE as u32, TILE_SIZE as u32)
                        )
                        .expect("Failed to draw tile");
                }
            }
            renderer.canvas.set_draw_color(Color::BLACK);
        }
    }
}

impl Draw<Renderer, Camera> for ChunkManager {
    fn draw(&self, renderer: &mut Renderer, camera: &Camera) {
        let (window_width, window_height) = renderer.get_window_size();
        let (offset_x, offset_y) = camera.get_offset(window_width, window_height);

        for (&(chunk_x, chunk_y), status) in &self.chunks {
            let chunk_screen_x = chunk_x * (CHUNK_SIZE as i32) * (TILE_SIZE as i32);
            let chunk_screen_y = chunk_y * (CHUNK_SIZE as i32) * (TILE_SIZE as i32);

            // Check if current_chunk is done generating
            match status {
                Status::Ready(ref chunk) => {
                    //Draw each tile
                    for (x, row) in chunk.tiles.iter().enumerate() {
                        for (y, tile) in row.iter().enumerate() {
                            let (screen_x, screen_y) = (
                                chunk_screen_x + (x as i32) * (TILE_SIZE as i32) - offset_x,
                                chunk_screen_y + (y as i32) * (TILE_SIZE as i32) - offset_y,
                            );

                            let color = match tile.tile_type {
                                TileType::Floor => Color::GREEN,
                                TileType::Liquid => Color::BLUE,
                                TileType::Wall => Color::GRAY,
                                _ => Color::WHITE,
                            };

                            renderer.canvas.set_draw_color(color);
                            renderer.canvas
                                .fill_rect(
                                    Rect::new(
                                        screen_x,
                                        screen_y,
                                        TILE_SIZE as u32,
                                        TILE_SIZE as u32
                                    )
                                )
                                .expect("Failed to draw tile");
                        }
                    }
                }
                Status::Pending => {
                    let (screen_x, screen_y) = (
                        chunk_screen_x + offset_x,
                        chunk_screen_y + offset_y,
                    );

                    renderer.canvas.set_draw_color(Color::CYAN);

                    renderer.canvas
                        .fill_rect(
                            Rect::new(screen_x, screen_y, TILE_SIZE as u32, TILE_SIZE as u32)
                        )
                        .expect("Failed to draw tile");

                    // Render loading tile
                }
            }
        }
        renderer.canvas.set_draw_color(Color::BLACK);
    }
}

#[cfg(test)]
mod tests {}
