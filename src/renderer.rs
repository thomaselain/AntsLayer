use noise::{ Fbm, NoiseFn };
use sdl2::{ pixels::Color, rect::Rect, ttf::{ Font, Sdl2TtfContext }, Sdl };

use crate::{
    ant::{ Ant, AntManager },
    chunk::{
        biomes::NoiseParams,
        index::flatten_index_i32,
        manager::LoadedChunk,
        tile::TileFlag,
        Chunk,
        ChunkContent,
        CHUNK_HEIGHT,
        CHUNK_WIDTH,
    },
};

/// When drawing an air tile, the renderer looks for tiles to draw bellow
/// This is maximum of air tiles to display
pub const MAX_RENDERING_DEPTH: u8 = 10;

/// Width of a renderer tile (in pixels)
pub const DEFAULT_TILE_SIZE: usize = 16;
const IS_GRID_ENABLED: bool = true;
const GRID_COLOR: Color = Color::RGBA(0, 0, 0, 25);

/// Clouds rendering
const CLOUD_COLOR: Color = Color::RGBA(250, 250, 250, 225 / (MAX_RENDERING_DEPTH as u8));
pub const CLOUDS_HEIGHT: i32 = (CHUNK_HEIGHT as i32) - (MAX_RENDERING_DEPTH as i32);
pub const CLOUDS_RENDERING: bool = false;
pub const VIEW_DISTANCE: i32 = (CHUNK_WIDTH as i32) * 4;

/// Window starting dimentions
pub const WIN_DEFAULT_W: u32 = 800;
pub const WIN_DEFAULT_H: u32 = 600;

/// Rendering sizes
pub const FONT_SIZE: u16 = 15;
pub const MIN_TILE_SIZE: usize = 1;
pub const MAX_TILE_SIZE: usize = 100;

// Struct for rendering with noise
pub struct RendererNoise {
    clouds: NoiseParams,
}

impl RendererNoise {
    pub fn new() -> Self {
        Self { clouds: NoiseParams::clouds() }
    }
    pub fn get_cloud_value(&self, x: f64, y: f64, z: f64, t: f64) -> f64 {
        let scale = self.clouds.scale;

        self.clouds.fbm.get([x * scale, y * scale, z * scale, t])
    }
}

pub struct Renderer<'ttf> {
    pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
    pub camera: (i32, i32, i32),
    pub view_distance: i32,
    pub dims: (u32, u32),
    pub is_grid_enabled: bool,
    pub font: Font<'ttf, 'static>,
    // Width of a renderer tile (in pixels)
    pub tile_size: usize,

    noise: RendererNoise,
}

impl NoiseParams {
    pub fn clouds() -> Self {
        Self {
            fbm: Fbm::new(69_42),
            octaves: 2,
            frequency: 1.0,
            lacunarity: 2.0,
            persistence: 0.8,
            scale: 0.04,
        }
    }
}

impl<'ttf> Renderer<'ttf> {
    pub fn new(
        sdl: &Sdl,
        ttf_context: &'ttf Sdl2TtfContext,
        title: &str
    ) -> Result<Renderer<'ttf>, String> {
        let video_subsystem = sdl.video()?;

        let window = video_subsystem
            .window(title, WIN_DEFAULT_W, WIN_DEFAULT_H)
            .position_centered()
            .resizable()
            .build()
            .map_err(|e| e.to_string())?;

        let mut canvas = window
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())?;

        canvas.set_blend_mode(sdl2::render::BlendMode::Blend);

        let font = ttf_context.load_font("assets/font/Minecraft.ttf", FONT_SIZE)?;

        Ok(Renderer::<'ttf> {
            font,
            is_grid_enabled: IS_GRID_ENABLED,
            canvas,
            // camera: (
            //     // x
            //     -((CHUNK_WIDTH as i32) * STARTING_AREA) / 2,
            //     // y
            //     -((CHUNK_WIDTH as i32) * STARTING_AREA) / 2,
            //     // z
            //     crate::chunk::SEA_LEVEL as i32 +1,
            // ),
            camera: (
                // x
                0,
                // y
                0,
                // z
                (crate::chunk::SEA_LEVEL as i32) + 1,
            ),
            dims: (WIN_DEFAULT_W, WIN_DEFAULT_H),
            noise: RendererNoise::new(),
            tile_size: DEFAULT_TILE_SIZE,
            view_distance: VIEW_DISTANCE,
            // font: todo!(),
        })
    }
}

// Calculation
impl<'ttf> Renderer<'ttf> {
    pub fn update_window_size(&mut self) {
        self.dims = self.canvas.output_size().expect("Failed to get window size");
    }

    // Offsets
    pub fn get_offset(&self) -> (i32, i32) {
        let (w, h) = self.dims;
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

    pub fn to_world_coords(chunk_pos: (i32, i32), tile_pos: (i32, i32)) -> (i32, i32) {
        let x = chunk_pos.0 * (CHUNK_WIDTH as i32) + tile_pos.0;
        let y = chunk_pos.1 * (CHUNK_WIDTH as i32) + tile_pos.1;

        (x, y)
    }

    pub fn is_chunk_on_screen(&self, chunk_pos: (i32, i32)) -> bool {
        let world_x = chunk_pos.0 * (CHUNK_WIDTH as i32);
        let world_y = chunk_pos.1 * (CHUNK_WIDTH as i32);
        let (screen_x, screen_y) = self.tile_to_screen_coords((world_x, world_y));

        let chunk_px = (CHUNK_WIDTH as i32) * (self.tile_size as i32);

        // 3) Bornes du chunk à l’écran
        let left = screen_x;
        let right = screen_x + chunk_px;
        let top = screen_y;
        let bottom = screen_y + chunk_px;

        // 4) Bornes de la fenêtre
        let win_w = self.dims.0 as i32;
        let win_h = self.dims.1 as i32;

        // 5) Vérifie le chevauchement
        let x_overlap = right > 0 && left < win_w;
        let y_overlap = bottom > 0 && top < win_h;
        x_overlap && y_overlap
    }
}

// Camera
impl<'ttf> Renderer<'ttf> {
    /// Filtre la liste des LoadedChunk pour ne garder que ceux visibles
    pub fn visible_chunks(&self, chunks: &Vec<LoadedChunk>) -> Vec<LoadedChunk> {
        let mut v: Vec<LoadedChunk> = Vec::with_capacity(chunks.len());
        for c in chunks {
            if self.is_chunk_on_screen(c.pos) {
                v.push(*c);
            }
        }
        v
    }

    pub fn zoom_in(&mut self) -> Result<(), ()> {
        self.tile_size += 1;
        Ok(())
    }
    pub fn zoom_out(&mut self) -> Result<(), ()> {
        self.tile_size -= 1;
        Ok(())
    }
}

// SDL
impl<'ttf> Renderer<'ttf> {
    pub fn draw_text(&mut self, text: &str, x: i32, y: i32) {
        let surface = self.font.render(text).blended(Color::WHITE).expect("Failed to render text");

        let texture_creator = self.canvas.texture_creator();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .expect("Failed to create texture");

        let target = Rect::new(x, y, surface.width(), surface.height());
        self.canvas.copy(&texture, None, Some(target)).unwrap();
    }
    pub fn draw_tile(&mut self, (x, y): (i32, i32), c: Color) {
        self.fill_rect((x, y), c);

        if self.is_grid_enabled {
            self.rect((x, y), Color::BLACK);
        }
    }
    pub fn rect(&mut self, (x, y): (i32, i32), c: Color) {
        self.canvas.set_draw_color(c);
        self.canvas
            .draw_rect(Rect::new(x, y, self.tile_size as u32, self.tile_size as u32))
            .expect(&format!("Failed to draw rect at {:?}", (x, y)));
        self.canvas.set_draw_color(Color::BLACK);
    }
    pub fn fill_rect(&mut self, (x, y): (i32, i32), c: Color) {
        self.canvas.set_draw_color(c);
        self.canvas
            .fill_rect(Rect::new(x, y, self.tile_size as u32, self.tile_size as u32))
            .expect(&format!("Failed to draw tile at {:?}", (x, y)));
        self.canvas.set_draw_color(Color::BLACK);
    }
}

/// Ant rendering
///
impl Ant {
    pub fn render(self, renderer: &mut Renderer) {
        let (x, y) = (self.pos.0, self.pos.1);
        let (x, y) = renderer.tile_to_screen_coords((x, y));
        renderer.draw_tile((x, y), Color::RED);
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
        ants: &Vec<Ant>,
        timestamp: f64
    ) {
        let mut tiles_to_draw = Vec::with_capacity((MAX_RENDERING_DEPTH as usize) + 1);

        for index in 0..ChunkContent::len() {
            let (x, y, z) = ChunkContent::index_to_xyz(index);

            if z == renderer.camera.2 {
                let (world_x, world_y) = Renderer::to_world_coords((pos_x, pos_y), (x, y));
                let draw_pos = renderer.tile_to_screen_coords((world_x, world_y));

                ////////////////////////////////////////////////////////////////
                if ants.len() > 0 {
                    // todo!("Chunk at {:?} has {:?} ants", (pos_x, pos_y), ants.len());
                    for a in ants {
                        if z == a.pos.2 {
                            a.render(renderer);
                        }
                    }
                }
                ////////////////////////////////////////////////////////////////

                ////////////////////////////////////////////////////////////////
                //////////////////     FOG     RENDERING  //////////////////////
                ////////////////////////////////////////////////////////////////
                let mut depth = 1;
                let mut current_z = z;
                'find_deepest: loop {
                    let idx = flatten_index_i32((x, y, current_z));
                    let tile = &self.content[idx];

                    tiles_to_draw.push(tile);

                    if
                        // current tile is not transparent
                        !tile.properties.contains(TileFlag::TRANSPARENT) ||
                        // tile.tile_type != TileType::Gas(Gas::Air) ||
                        // Reached bottom
                        current_z == 0 ||
                        // Dont draw too much
                        depth >= MAX_RENDERING_DEPTH
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
                    renderer.fill_rect(draw_pos, c);
                }

                // Draw the fog layer
                // And water depth
                'bottom_to_top: loop {
                    if let Some(tile) = tiles_to_draw.pop() {
                        ////////////////////////////////////////////////////////////////
                        ////////////////////  Ants  Rendering //////////////////////////
                        ////////////////////////////////////////////////////////////////
                        // for a in ants {
                        //     if a.pos == (x, y, z) {
                        //         let (ant_x, ant_y) = (x, y);
                        //         renderer.draw_tile((ant_x, ant_y), Color::RED);
                        //         break 'bottom_to_top;
                        //     }
                        // }
                        ////////////////////////////////////////////////////////////////

                        let mut fog = tile.color();
                        fog.a += tile.color().a;
                        renderer.fill_rect(draw_pos, fog);
                    } else {
                        break 'bottom_to_top;
                    }
                }

                ////////////////////////////////////////////////////////////////
                ////////////////////////Clouds//////////////////////////////////
                ////////////////////////////////////////////////////////////////
                if CLOUDS_RENDERING && z >= CLOUDS_HEIGHT {
                    let mut cloud = CLOUD_COLOR;
                    // Convert into world coords f64
                    // Allows use of perlin.get[coords]
                    let (x, y) = Renderer::to_world_coords((pos_x, pos_y), (x, y));
                    let (x, y, z) = (x as f64, y as f64, CLOUDS_HEIGHT as f64);

                    // Find cloud value
                    let cloud_value = ((cloud.a as f64) +
                        renderer.noise.get_cloud_value(
                            x + timestamp * 1.5,
                            y + timestamp * 1.1,
                            z,
                            timestamp / 69.0
                        ) *
                            255.0) as u8;
                    cloud.a = match cloud_value {
                        0..50 => 150,
                        50..75 => 100,
                        75..79 => 50,
                        140..150 => 75,
                        150..160 => 15,
                        170..180 => 175,
                        _ => 0,
                    };
                    renderer.fill_rect(draw_pos, cloud);
                }

                ////////////////////////////////////////////////////////////////
                if IS_GRID_ENABLED {
                    renderer.rect(draw_pos, GRID_COLOR);
                }
                ////////////////////////////////////////////////////////////////

                tiles_to_draw.clear();
            }
        }
    }
}
