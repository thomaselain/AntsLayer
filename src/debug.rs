use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::ttf::Font;

use crate::Game;

impl Game {
    pub fn display_debug(&mut self) -> Result<(), String> {
        // Load font
        let font = self
            .ttf_context
            .load_font("assets/Minecraft.ttf", 16)
            .expect("Failed to load Font");

        // Text to display
        let text = format!("Camera: {:?}", self.renderer.camera);

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
        let target = Rect::new(10, 10, surface.width(), surface.height());
        self.renderer.canvas.copy(&texture, None, Some(target))?;

        Ok(())
    }
}
