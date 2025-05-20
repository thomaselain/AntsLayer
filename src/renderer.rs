use std::sync::{ Arc, Mutex };

use sdl2::Sdl;

use crate::chunk::{ tile::Tile, Chunk, ChunkContent, CHUNK_WIDTH };

// Width of a renderer tile (in pixels)
pub const TILE_SIZE: usize = 4;

pub struct Renderer {
    pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
    pub camera: (i32, i32, i32),
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

        Ok(Renderer { canvas, camera: (0, 0, crate::chunk::SEA_LEVEL as i32) })
    }

    pub fn get_window_size(&self) -> (u32, u32) {
        self.canvas.output_size().expect("Failed to get window size")
    }

    // Offsets
    pub fn get_offset(&self) -> (i32, i32) {
        let (w, h) = self.get_window_size();
        let half_w = (w as i32) / 2;
        let half_h = (h as i32) / 2;

        let offset_x = self.camera.0 * (TILE_SIZE as i32) + half_w;
        let offset_y = self.camera.1 * (TILE_SIZE as i32) + half_h;

        (offset_x, offset_y)
    }

    // returns self.camera.z
    pub fn get_camera_z(&self) -> i32 {
        self.camera.2
    }

    // Converts Tile coords into displayable coords (x,y)
    pub fn tile_screen_coords(
        offset: (i32, i32),
        chunk_pos: (&i32, &i32),
        tile_pos: (i32, i32)
    ) -> (i32, i32) {
        let world_x_tiles = chunk_pos.0 * (CHUNK_WIDTH as i32) + tile_pos.0;
        let world_y_tiles = chunk_pos.1 * (CHUNK_WIDTH as i32) + tile_pos.1;

        let pixel_x = offset.0 + world_x_tiles * (TILE_SIZE as i32);
        let pixel_y = offset.1 + world_y_tiles * (TILE_SIZE as i32);

        (pixel_x, pixel_y)
    }
}

impl Chunk {
    pub fn draw(
        &self,
        renderer: &mut Renderer,
        // Chunk coordinates
        (pos_x, pos_y): (&i32, &i32)
    ) {
        let (offset_x, offset_y) = renderer.get_offset();
        let camera_z = renderer.get_camera_z();
        // eprintln!("offset : {:?}", (offset_x, offset_y));

        for index in 0..ChunkContent::len() {
            let (
                // x = i % W
                x,
                // y = (i / W) % W
                y,
                // z = (i / W²) % W

                // Does not need calculation because it is set by camera
                z,
            ) = Tile::index_to_xyz(index);

            // Skip other layers
            if z != camera_z {
                continue;
            }

            // println!("Drawing tile at {:?} (index = {:?})", (x, y, z), index);

            // NEEDS FIX
            // panic!(
            //     "Fix coordinates
            // \nChunk coords :{:?}
            // \nIndex        :{:?}
            // \nTile pos     :{:?}
            // ",
            //     (pos_x, pos_y),
            //     index,
            //     (x,y),
            // );

            let draw_pos = Renderer::tile_screen_coords(
                (offset_x, offset_y),
                (pos_x, pos_y),
                (x, y)
            );

            self.content[index].draw(renderer, draw_pos);
        }
    }
}
