#[allow(unused)]
use ::{ sdl2::pixels::Color, sdl2::rect::Rect, std::{ fmt::Debug } };
#[allow(unused)]
use crate::time;
#[allow(unused)]
use crate::{
    chunk::generation::{ MapShape, STARTING_AREA, STARTING_MAP_SHAPE },
    renderer::{ CLOUDS_HEIGHT, FONT_SIZE },
    Game,
};

impl Debug for MapShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SQUARE => write!(f, "SQUARE"),
            Self::RECT => write!(f, "RECT"),
            Self::ROUND => write!(f, "ROUND"),
        }
    }
}

const DEBUG_FONT_SIZE: u32 = FONT_SIZE as u32;
const DEBUG_BACKGROUND_COLOR: Color = Color::RGBA(5, 5, 5, 150);

impl<'ttf> Game<'ttf> {
    pub fn display_info(&mut self) -> Result<(), String> {
        // Black background
        self.renderer.canvas.set_draw_color(DEBUG_BACKGROUND_COLOR);
        self.renderer.canvas.fill_rect(Rect::new(0, 0, 250, DEBUG_FONT_SIZE * 11))?;

        self.display_info_at(
            format!(
                "win:{:?}|||FPS  :{:?}|||TPS  :{:?}|||",
                // FPS CALCULATION
                self.renderer.dims,
                self.fps.lock().unwrap(),
                self.tps.lock().unwrap()
            ),
            0
        )?;
        self.display_info_at(format!("Camera  : {:?}", self.renderer.camera), 1)?;
        self.display_info_at(
            format!("Time       : {:?}", time::formatted_game_time(self.elapsed_secs())),
            2
        )?;
        self.display_info_at(format!("Tile size  : {:?}", self.renderer.tile_size), 3)?;
        self.display_info_at(format!("View dist  : {:?}", self.renderer.view_distance), 4)?;

        self.display_info_at(
            format!("Loaded chunks : {:?}", self.chunk_manager.loaded_chunks.len()),
            5
        )?;
        // self.display_info_at(format!("Visible chunks : {:?}", chunks_in_fov), 6)?;
        self.display_info_at(format!("Clouds height : {:?}", CLOUDS_HEIGHT), 7)?;

        if let Some(joette) = self.ant_manager.ants.first() {
            self.display_info_at(format!("Joette's pos {:?}", joette.pos), 9)?;
        }

        self.display_info_at(
            format!("map is {:?} of size {:?}", STARTING_MAP_SHAPE, STARTING_AREA),
            10
        )?;
        Ok(())
    }
    pub fn display_info_at(&mut self, info: String, index: i32) -> Result<(), String> {
        // Load font
        // Text to display
        let text = info;

        // Turn text into a surface and then a texture
        let surface = self.renderer.font
            .render(&text)
            .blended(Color::WHITE)
            .map_err(|e| e.to_string())?;

        let texture_creator = self.renderer.canvas.texture_creator();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        // write it
        let target = Rect::new(
            10,
            (DEBUG_FONT_SIZE as i32) * index,
            surface.width(),
            surface.height()
        );
        self.renderer.canvas.copy(&texture, None, Some(target))?;
        Ok(())
    }
}
