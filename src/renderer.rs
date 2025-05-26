use noise::{ Fbm, NoiseFn };
use sdl2::{ pixels::Color, rect::Rect, Sdl };

use crate::{
    ant::Ant,
    chunk::{
        biomes::NoiseParams, index::flatten_index_i32, tile::Tile, Chunk, ChunkContent, CHUNK_HEIGHT, CHUNK_WIDTH, SEA_LEVEL
    },
};

/// When drawing an air tile, the renderer looks for tiles to draw bellow
/// This is maximum of air tiles to display
pub const MAX_AIR_DEPTH: u8 = 8;
// Width of a renderer tile (in pixels)
pub const DEFAULT_TILE_SIZE: usize = 4;
//
pub const CLOUDS_HEIGHT: i32 = CHUNK_HEIGHT as i32 - MAX_AIR_DEPTH as i32;
pub const CLOUDS_RENDERING: bool = true;
pub const VIEW_DISTANCE: i32 = (CHUNK_WIDTH as i32) * 4;
// pub const MAX_VISIBLE_CHUNKS: usize = VIEW_DISTANCE.pow(2) as usize;

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
            scale: 0.1,
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
    // Converts Tile coords into displayable coords (x,y)
    pub fn tile_to_screen_coords(&self, (x, y): (i32, i32)) -> (i32, i32) {
        let offset = self.get_offset();

        let pixel_x = offset.0 + x * (self.tile_size as i32);
        let pixel_y = offset.1 + y * (self.tile_size as i32);

        (pixel_x, pixel_y)
    }

    pub fn draw_tile(&mut self, pos: (i32, i32), c: Color) {
        let (x, y) = self.tile_to_screen_coords(pos);

        self.canvas.set_draw_color(c);
        self.canvas
            .fill_rect(Rect::new(x, y, self.tile_size as u32, self.tile_size as u32))
            .expect(&format!("Failed to draw tile at {:?}", pos));
        self.canvas.set_draw_color(Color::BLACK);
    }

    pub fn to_world_coords(chunk_pos: (i32, i32), tile_pos: (i32, i32)) -> (i32, i32) {
        let x = chunk_pos.0 * (CHUNK_WIDTH as i32) + tile_pos.0;
        let y = chunk_pos.1 * (CHUNK_WIDTH as i32) + tile_pos.1;

        (x, y)
    }
    pub fn is_on_screen(&self, draw_pos: (i32, i32)) -> bool {
        let win = self.get_window_size();

        let range_x = 0..win.0 as i32;
        let range_y = 0..win.1 as i32;
        if range_x.contains(&draw_pos.0) && range_y.contains(&draw_pos.1) {
            true
        } else {
            false
        }
    }
}

/// Ant rendering
///
impl Ant {
    pub fn render(self, renderer: &mut Renderer) {
        renderer.draw_tile((self.pos.0, self.pos.1), Color::RED);
    }
}

/// Chunk rendering
///
impl Chunk {
    pub fn render(
        &self,
        renderer: &mut Renderer,
        // Chunk coordinates
        (pos_x, pos_y): (i32, i32),
        timestamp: f64
    ) {
        for index in 0..ChunkContent::len() {
            let (x, y, z) = Tile::index_to_xyz(index);

            if z == renderer.camera.2 {
                let (world_x, world_y) = Renderer::to_world_coords((pos_x, pos_y), (x, y));
                let draw_pos = renderer.tile_to_screen_coords((world_x, world_y));

                if !renderer.is_on_screen(draw_pos) {
                    continue;
                }

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
                        current_z == 0 ||
                        // Dont draw too much
                        depth >= MAX_AIR_DEPTH
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

                // Draw the next transparent tiles
                if let Some(next_tile) = tiles_to_draw.pop() {
                    if !next_tile.tile_type.is_transparent() {
                        break;
                    }
                    // fog rendering
                    let mut c = next_tile.color();
                    c.a = c.a * (1 + (tiles_to_draw.len() as u8)) / 2;
                    next_tile.draw(renderer, draw_pos, c);

                    ////////////////////////////////////////////////////////////////
                    ////////////////////////Clouds//////////////////////////////////
                    ////////////////////////////////////////////////////////////////
                    if CLOUDS_RENDERING && z >= CLOUDS_HEIGHT {
                        'clouds_rendering: loop {
                            // Convert into world coords f64
                            // Allows use of perlin.get[coords]
                            let (x, y) = Renderer::to_world_coords((pos_x, pos_y), (x, y));
                            let (x, y, z) = (x as f64, y as f64, CLOUDS_HEIGHT as f64);

                            // Find cloud value
                            c.a = ((c.a as f64) +
                                renderer.noise.get_cloud_value(
                                    x + timestamp * 7.5,
                                    y + timestamp * 3.5,
                                    z,
                                    timestamp / 25.0
                                ) *
                                    255.0) as u8;
                            next_tile.draw(renderer, draw_pos, c);
                            break 'clouds_rendering;
                        }
                    }
                }
            }
        }
    }
}
