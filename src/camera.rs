use crate::{
    coords::Coords, terrain, window::{self, WIDTH}
};

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub position: Coords,
    pub zoom: f32,
    pub screen_width: u32,
    pub screen_height: u32,
}

impl Camera {
    pub fn new(screen_width: u32, screen_height: u32) -> Self {
        Camera {
            position: Coords { x: 0, y: 0 },
            zoom: terrain::WIDTH as f32 / window::WIDTH as f32,
            screen_width,
            screen_height,
        }
    }
    pub fn zoom_in(&mut self) {
        self.zoom = self.zoom.clamp(0.0, WIDTH as f32) * 0.95;
    }
    pub fn zoom_out(&mut self) {
        self.zoom = self.zoom.clamp(0.0, WIDTH as f32) / 0.95;
    }
}
