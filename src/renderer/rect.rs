use sdl2::{ pixels::Color, rect::Rect };

use crate::chunk::{ CHUNK_WIDTH };

use super::Renderer;

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
    pub fn draw_chunk(&mut self, (x, y): (i32, i32), c: Color) {
        let rect_size = (self.tile_size as u32) * (CHUNK_WIDTH as u32);
        self.canvas.set_draw_color(c);
        self.canvas
            .fill_rect(Rect::new(x, y, rect_size, rect_size))
            .expect(&format!("Failed to draw tile at {:?}", (x, y)));
        self.canvas.set_draw_color(Color::BLACK);
    }
}
