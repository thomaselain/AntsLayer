use noise::{ NoiseFn };
use sdl2::{ pixels::Color, ttf::{ Font, Sdl2TtfContext }, Sdl };

use crate::{
    ant::{ Ant },
    chunk::{ biomes::NoiseParams, CHUNK_HEIGHT, CHUNK_WIDTH },
};

/// SDL methods for drawing squares
/// for tiles rendering
///
/// let (x,y) = (0,0);
/// let c = Color::Red;
/// Renderer::draw_tile((x, y), c);
mod rect;

//
mod clouds;
mod chunk;
mod maths;
mod camera;

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

/// Ant rendering
///
impl Ant {
    pub fn render(self, renderer: &mut Renderer) {
        let (x, y) = (self.pos.0, self.pos.1);
        let (x, y) = renderer.tile_to_screen_coords((x, y));
        renderer.draw_tile((x, y), Color::RED);
    }
}
