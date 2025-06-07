use noise::Fbm;
use sdl2::pixels::Color;

//
use crate::{
    chunk::{ biomes::NoiseParams },
    renderer::{ Renderer, CLOUDS_HEIGHT },
};

const CLOUD_COLOR: Color = Color::RGBA(200, 175, 200, 255);

impl NoiseParams {
    pub fn clouds() -> Self {
        Self {
            fbm: Fbm::new(69_42),
            octaves: 2,
            frequency: 1.0,
            lacunarity: 2.0,
            persistence: 0.8,
            scale: 0.005,
        }
    }
}

impl Renderer<'_> {
    pub fn draw_cloud(
        &mut self,
        chunk_pos: (i32, i32),
        //
        (x, y): (i32, i32),
        //
        draw_pos: (i32, i32),
        //
        timestamp: f64
    ) {
        let mut cloud = CLOUD_COLOR;
        let (x, y) = Renderer::to_world_coords(chunk_pos, (x, y));

        // Convert into world coords f64
        // Allows use of perlin.get[coords]
        let (x, y, z) = (
            //
            x as f64,
            //
            y as f64,
            //
            CLOUDS_HEIGHT as f64,
        );

        // Find cloud value
        let cloud_value = ((cloud.a as f64) +
            self.noise.get_cloud_value(
                x + timestamp * 10.0,
                y + timestamp * 5.0,
                z,
                timestamp / 100.0
            ) *
                255.0) as u8;
        cloud.a = match cloud_value {
            0..50 => 50,
            50..150 => 100,
            150..200 => 150,
            _ => 0,
        };
        self.fill_rect(draw_pos, cloud);
    }
}
