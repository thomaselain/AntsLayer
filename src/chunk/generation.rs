use std::sync::Arc;
#[allow(unused)]
use std::{ ops::Range, thread::{ self, JoinHandle }, time::Duration };

use crate::{
    chunk::{ biomes::Biome, manager::WorldNoise, ChunkContent, SEA_LEVEL },
    renderer::Renderer,
};

use super::{ manager::LoadedChunk, tile::Tile, Chunk, ChunkManager as Manager, HEIGHT, WIDTH };

#[allow(unused)]
pub enum MapShape {
    SQUARE,
    RECT,
    // ???
    ROUND,
}

#[allow(unused)]
pub const STARTING_AREA: i32 = if cfg!(test) { 2 } else { 2 };
pub const STARTING_MAP_SHAPE: MapShape = if cfg!(test) {
    MapShape::SQUARE
} else {
    MapShape::SQUARE
};

impl Manager {
    pub fn look_for_new_chunks(&mut self, renderer: &mut Renderer) {
        let (x_min, x_max, y_min, y_max) = renderer.camera_range_i32();
        let mut new_chunks = vec![];

        for x in x_min..x_max {
            for y in y_min..y_max {
                if !self.loaded_chunks.contains_key(&(x, y)) {
                    new_chunks.push(Chunk::generate((x, y), &self.world_noise));
                }
            }
        }
        //
        for handle in new_chunks {
            let chunk = handle.join().unwrap();
            self.loaded_chunks.insert(chunk.pos, chunk);
        }
    }
    pub fn generate_range(
        &self,
        x_range: Range<i32>,
        y_range: Range<i32>
    ) -> Vec<JoinHandle<LoadedChunk>> {
        let mut m = Vec::new();

        for j in y_range {
            for i in x_range.clone() {
                m.push(Chunk::generate((i, j), &self.world_noise));
            }
        }

        m
    }
}
impl Chunk {
    pub fn new() -> Self {
        Self { content: ChunkContent::new() }
    }
}

impl Chunk {
    pub fn generate(pos: (i32, i32), world_noise: &WorldNoise) -> JoinHandle<LoadedChunk> {
        // Get the thread safe noise reference
        let world_noise = Arc::clone(&world_noise);

        thread::spawn(move || {
            let chunk = LoadedChunk::new(pos);

            for z in 0..HEIGHT as i32 {
                for x in 0..WIDTH as i32 {
                    for y in 0..WIDTH as i32 {
                        let (nx, ny) = (
                            (x as f64) + (pos.0 as f64) * (WIDTH as f64),
                            (y as f64) + (pos.1 as f64) * (WIDTH as f64),
                        );

                        // Set the tile
                        let tile = Self::tile_at((nx as i32, ny as i32, z), &world_noise);
                        chunk.c.lock().unwrap().set((x, y, z), tile);
                    }
                }
            }
            //////////////////////////////////////////////////////
            // #[cfg(test)]
            // thread::sleep(Duration::from_millis(150));
            //////////////////////////////////////////////////////

            chunk
        })
    }
}

impl Chunk {
    const CAVE_DEPTH: f64 = 5.0;

    fn surface(surface_height: f64, (x, y, z): (f64, f64, f64), biome: &Biome) -> Tile {
        if z < surface_height + 5.0 {
            Tile::DIRT
        } else if z < surface_height + 3.0 {
            Tile::CLAY
        } else if z < surface_height + 2.0 {
            Tile::GRANITE
        } else if z <= (SEA_LEVEL as f64) {
            Tile::WATER
        } else {
            Tile::AIR
        }
    }

    fn ores((x, y, z): (f64, f64, f64), world_noise: &WorldNoise) -> Tile {
        // let noise = world_noise[Manager::VEINS].get((x, y, z));

        Tile::ERROR
    }

    fn cave_noise((x, y, z): (f64, f64, f64), world_noise: &WorldNoise, biome: &Biome) -> Tile {
        let scale = world_noise[Manager::SURFACE].scale;

        let cave =
            world_noise[Manager::CAVES].get((x * scale, y * scale, z * scale)) -
            (z / (HEIGHT as f64)).powf(1.5);
        let tunnel = world_noise[Manager::TUNNELS].get((x * scale, y * scale, z * scale));

        if cave > 0.3 {
            Tile::MARBLE
        } else if cave > 0.2 {
            Tile::GRANITE
        } else if cave > 0.0 {
            Tile::DIRT
        } else if tunnel > 0.42 {
            Tile::AIR
        } else if tunnel > 0.3 {
            Tile::DIRT
        } else {
            Tile::ERROR
        }
    }
    pub fn tile_at((x, y, z): (i32, i32, i32), world_noise: &WorldNoise) -> Tile {
        let (x, y, z) = (x as f64, y as f64, z as f64);

        if z == 0.0 {
            return Tile::BEDROCK;
        }
        let biome = Biome::get_biome_params(x, y, world_noise);

        let scale = world_noise[Manager::SURFACE].scale;
        // let surface_noise = world_noise[Manager::SURFACE].get((x, y, 0.0));
        // For trees or surface items

        let varitation_noise = world_noise[Manager::VARIATIONS]
            .get((x * scale, y * scale, 0.0))
            .powf(2.0);
        let detail_noise = world_noise[Manager::DETAIL]
            .get((x * scale * 0.5, y * scale * 0.5, 10.0))
            .powf(2.0);

        let mut surface_height = varitation_noise * detail_noise + biome.get();

        // Normalization 0..CHUNK_HEIGHT
        surface_height = (surface_height + 1.0) * ((HEIGHT as f64) / 2.0);

        let surface = Self::surface(surface_height, (x, y, z), &biome);

        // let ores = Self::ores((x, y, z), world_noise);
        let cave = Self::cave_noise((x, y, z), world_noise, &biome);

        if z <= surface_height || (cave == Tile::AIR && surface != Tile::WATER) {
            cave
        } else if z > surface_height {
            surface
        } else {
            Tile::AIR
        }
    }
}
