use crate::chunk::CHUNK_WIDTH;

use super::Renderer;

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
