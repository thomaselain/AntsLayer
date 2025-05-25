use std::process::{ ExitCode, Termination };

use sdl2::pixels::Color;
use sdl2::rect::Rect;

use crate::Game;

impl Game {
    pub fn display_debug(&mut self) -> Result<(), String> {
        self.display_debug_at(format!("Camera  : {:?}", self.renderer.camera), 1)?;
        self.display_debug_at(format!("Time       : {:.1?}s", self.elapsed_secs()), 2)?;
        self.display_debug_at(format!("Tile size  : {:?}", self.renderer.tile_size), 3)?;

        self.display_debug_at(format!("Joette's pos {:?}", self.ant_manager.ants.first().expect("No ants?").pos), 6)?;
        Ok(())
    }
    pub fn display_debug_at(&mut self, info: String, index: i32) -> Result<(), String> {
        // Load font
        let font = self.ttf_context
            .load_font("assets/Minecraft.ttf", 16)
            .expect("Failed to load Font");

        // Text to display
        let text = info;

        // Turn text into a surface and then a texture
        let surface = font
            .render(&text)
            .blended(Color::WHITE)
            .map_err(|e| e.to_string())?;

        let texture_creator = self.renderer.canvas.texture_creator();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        // write it
        let target = Rect::new(10, 25 * index, surface.width(), surface.height());
        self.renderer.canvas.copy(&texture, None, Some(target))?;
        Ok(())
    }
}
