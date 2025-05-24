use noise::{ Fbm, NoiseFn };
use sdl2::Sdl;

use crate::chunk::{
    biomes::NoiseParams,
    index::flatten_index_i32,
    tile::{ Tile },
    Chunk,
    ChunkContent,
    CHUNK_WIDTH,
    SEA_LEVEL,
};

/// When drawing an air tile, the renderer looks for tiles to draw bellow
/// This is maximum of air tiles to display
pub const MAX_AIR_DEPTH: u8 = 5;
// Width of a renderer tile (in pixels)
pub const DEFAULT_TILE_SIZE: usize = 4;
//
pub const CLOUDS_RENDERING: bool = false;
pub const VIEW_DISTANCE: i32 = (CHUNK_WIDTH as i32) * 4;
pub const MAX_VISIBLE_CHUNKS: usize = VIEW_DISTANCE.pow(2) as usize;

// Struct for rendering with noise
pub struct RendererNoise {
    clouds: NoiseParams,
}

impl RendererNoise {
    pub fn get_cloud_value(&self, x: f64, y: f64, z: f64, t: f64) -> f64 {
        let scale = self.clouds.scale / (CHUNK_WIDTH as f64);

        self.clouds.fbm.get([x * scale, y * scale, z * scale, t])
    }
}

pub struct Renderer {
    pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
    pub camera: (i32, i32, i32),
    pub view_distance: i32,

    // Width of a renderer tile (in pixels)
    pub tile_size: usize,

    noise: RendererNoise,
}

impl NoiseParams {
    pub fn clouds() -> Self {
        Self {
            fbm: Fbm::new(69_420),
            octaves: 1,
            frequency: 1.05,
            lacunarity: 2.0,
            persistence: 1.9,
            scale: 0.3,
        }
    }
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

        let mut canvas = window
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())?;
        canvas.set_blend_mode(sdl2::render::BlendMode::Blend);

        Ok(Renderer {
            canvas,
            camera: (0, 0, crate::chunk::SEA_LEVEL as i32),
            noise: RendererNoise { clouds: NoiseParams::clouds() },
            tile_size: DEFAULT_TILE_SIZE,
            view_distance: VIEW_DISTANCE,
        })
    }

    pub fn get_window_size(&self) -> (u32, u32) {
        self.canvas.output_size().expect("Failed to get window size")
    }

    // Offsets
    pub fn get_offset(&self) -> (i32, i32) {
        let (w, h) = self.get_window_size();
        let half_w = (w as i32) / 2;
        let half_h = (h as i32) / 2;

        let offset_x = self.camera.0 * (self.tile_size as i32) + half_w;
        let offset_y = self.camera.1 * (self.tile_size as i32) + half_h;

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
        tile_pos: (i32, i32),
        tile_size: i32
    ) -> (i32, i32) {
        let (x, y) = Self::tile_world_coords(chunk_pos, tile_pos);

        let pixel_x = offset.0 + x * tile_size;
        let pixel_y = offset.1 + y * tile_size;

        (pixel_x, pixel_y)
    }
    pub fn tile_world_coords(chunk_pos: (&i32, &i32), tile_pos: (i32, i32)) -> (i32, i32) {
        let x = chunk_pos.0 * (CHUNK_WIDTH as i32) + tile_pos.0;
        let y = chunk_pos.1 * (CHUNK_WIDTH as i32) + tile_pos.1;

        (x, y)
    }
}

impl Chunk {
    pub fn draw(
        &self,
        renderer: &mut Renderer,
        // Chunk coordinates
        (pos_x, pos_y): (i32, i32),
        timestamp: f64
    ) {
        let (offset_x, offset_y) = renderer.get_offset();
        let camera_z = renderer.get_camera_z();
        // eprintln!("offset : {:?}", (offset_x, offset_y));

        for index in 0..ChunkContent::len() {
            let (x, y, z) = Tile::index_to_xyz(index);

            if z == camera_z {
                let draw_pos = Renderer::tile_screen_coords(
                    (offset_x, offset_y),
                    (&pos_x, &pos_y),
                    (x, y),
                    renderer.tile_size as i32
                );
                let mut depth = 0;
                let mut current_z = z;
                let mut tiles_to_draw = Vec::new();

                ////////////////////////////////////////////////////////////////
                //////////////////     FOG     RENDERING  //////////////////////
                ////////////////////////////////////////////////////////////////
                'find_deepest: loop {
                    let idx = flatten_index_i32((x, y, current_z));
                    let tile = &self.content[idx];

                    tiles_to_draw.push(tile);

                    if
                        // current tile is not transparent
                        !tile.tile_type.is_transparent() ||
                        // Reached bottom
                        current_z == 0 
                        // Dont draw too much
                        || depth >= MAX_AIR_DEPTH
                    {
                        break 'find_deepest;
                    }

                    current_z -= 1;
                    depth += 1;
                }
                ////////////////////////////////////////////////////////////////

                // Draw deepest tile found first
                if let Some(bottom_tile) = tiles_to_draw.pop() {
                    let c = bottom_tile.color();
                    bottom_tile.draw(renderer, draw_pos, c);
                }

                // Draw tiles from bottom to top
                for tile in tiles_to_draw.iter().rev() {
                    let mut c = tile.color();
                    if tile.tile_type.is_transparent() {
                        tile.draw(renderer, draw_pos, c);
                    }

                    // Clouds
                    if CLOUDS_RENDERING {
                        if z > (SEA_LEVEL as i32) + (MAX_AIR_DEPTH as i32) / 2 {
                            let (x, y) = Renderer::tile_world_coords((&pos_x, &pos_y), (x, y));
                            let (x, y, z) = (x as f64, y as f64, z as f64);
                            // Find cloud value
                            c.a = ((c.a as f64) +
                                renderer.noise.get_cloud_value(
                                    x + timestamp * 5.0,
                                    y + timestamp * 2.5,
                                    z,
                                    timestamp / 10.0
                                ) *
                                    255.0) as u8;
                            tile.draw(renderer, draw_pos, c);
                        }
                    }
                }
            }
        }
    }
}
