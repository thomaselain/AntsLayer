use std::sync::Arc;
#[allow(unused)]
use std::{ ops::Range, thread::{ self, JoinHandle }, time::Duration };

use crate::{ chunk::{ biomes::NoiseParams, ChunkContent, SEA_LEVEL }, renderer::Renderer };

use super::{
    biomes::Params,
    manager::LoadedChunk,
    tile::Tile,
    Chunk,
    ChunkManager as Manager,
    HEIGHT,
    WIDTH,
};

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

pub type WorldNoise = Arc<[NoiseParams; 5]>;

impl Manager {
    pub fn look_for_new_chunks(&mut self, renderer: &mut Renderer) {
        let (x_min, x_max, y_min, y_max) = renderer.camera_range_i32();
        let mut new_chunks = vec![];

        for x in x_min..x_max {
            for y in y_min..y_max {
                if !self.loaded_chunks.contains_key(&(x, y)) {
                    new_chunks.push(self.generate(&(x, y), &self.world_noise));
                }
            }
        }
        //
        for handle in new_chunks {
            let chunk = handle.join().unwrap();
            self.loaded_chunks.insert(chunk.pos, chunk);
        }
    }
    pub fn generate(
        &self,
        (x, y): &(i32, i32),
        world_noise: &WorldNoise
    ) -> JoinHandle<LoadedChunk> {
        let biome = &self.biome_at((*x, *y));

        Chunk::from_biome((*x, *y), biome, world_noise)
    }
    pub fn generate_range(
        &self,
        x_range: Range<i32>,
        y_range: Range<i32>
    ) -> Vec<JoinHandle<LoadedChunk>> {
        let mut m = Vec::new();

        for j in y_range {
            for i in x_range.clone() {
                m.push(self.generate(&(i, j), &self.world_noise));
            }
        }

        m
    }
}
impl Chunk {
    pub fn new() -> Self {
        Self { content: ChunkContent::new() }
    }
    pub fn from_biome(
        pos: (i32, i32),
        b: &Params,
        world_noise: &WorldNoise
    ) -> JoinHandle<LoadedChunk> {
        Chunk::generate(pos, b.clone(), world_noise)
    }
}

impl Chunk {
    pub fn generate(
        pos: (i32, i32),
        b: Params,
        world_noise: &WorldNoise
    ) -> JoinHandle<LoadedChunk> {
        // Get the thread safe noise reference
        let world_noise = Arc::clone(&world_noise);

        thread::spawn(move || {
            let chunk = LoadedChunk::new(b.id, pos);

            for z in 0..HEIGHT as i32 {
                for x in 0..WIDTH as i32 {
                    for y in 0..WIDTH as i32 {
                        // Get  noise value
                        let (nx, ny) = (
                            (x as f64) + (pos.0 as f64) * (WIDTH as f64),
                            (y as f64) + (pos.1 as f64) * (WIDTH as f64),
                        );
                        let v = b.noise.get((
                            //
                            nx,
                            //
                            ny,
                            //
                            z as f64,
                        ));

                        // Set the tile
                        let tile = b.tile_at((nx as i32, ny as i32, z), v, &world_noise);
                        chunk.c.lock().unwrap().set((x, y, z), tile);
                    }
                }
            }
            //////////////////////////////////////////////////////
            // #[cfg(test)]
            // thread::sleep(Duration::from_millis(15));
            //////////////////////////////////////////////////////

            chunk
        })
    }
}

impl Params {
    fn surface(surface_height: f64, (x, y, z): (f64, f64, f64)) -> Option<Tile> {
        if z < surface_height - 5.0 {
            Some(Tile::GRANITE)
        } else if z < surface_height - 4.0 {
            Some(Tile::CLAY)
        } else if z < surface_height - 2.0 {
            Some(Tile::DIRT)
        } else if z < surface_height && z == (SEA_LEVEL as f64) + 1.0 {
            Some(Tile::SAND)
        } else if z < (SEA_LEVEL as f64) {
            Some(Tile::WATER)
        } else {
            // Some(Tile::AIR)
            None
        }
    }

    fn veins((x, y, z): (f64, f64, f64), world_noise: &WorldNoise) -> Option<Tile> {
        // let noise = world_noise[Manager::VEINS].get((x, y, z));

        // match noise {
        //     s if s < 0.0 => { Some(Tile::DIRT) }
        //     s if s < 0.1 => { Some(Tile::CLAY) }
        //     s if s < 0.15 => { Some(Tile::LIMESTONE) }
        //     s if s < 0.25 => { Some(Tile::MARBLE) }
        //     s if s < 0.3 => { Some(Tile::WATER) }
        //     s if s > 0.0 => { None }
        //     _ => { Some(Tile::AIR) }
        // }
        None
    }

    fn cave_noise((x, y, z): (f64, f64, f64), world_noise: &WorldNoise) -> Option<Tile> {
        let noise = world_noise[Manager::CAVES].get((x, y, z));
        if noise < -0.24 {
            return Some(Tile::AIR);
        }
        // Stones
        match noise {
            0.0..0.1 => Some(Tile::GRANITE),
            0.1..0.3 => Some(Tile::DIRT),
            0.3..0.4 => Some(Tile::LIMESTONE),
            0.4..0.45 => Some(Tile::MARBLE),
            0.45..0.5 => Some(Tile::CLAY),
            _ => None,
        }
    }
    pub fn tile_at(&self, (x, y, z): (i32, i32, i32), v: f64, world_noise: &WorldNoise) -> Tile {
        let (x, y, z) = (x as f64, y as f64, z as f64);

        let scale = world_noise[Manager::SURFACE].scale;

        let surface_noise = world_noise[Manager::SURFACE].get((x, y, 0.0));
        let detail_noise = world_noise[Manager::DETAIL].get((x * scale, y * scale, 0.0));

        let surface_height = (surface_noise + detail_noise) * (HEIGHT as f64) + (SEA_LEVEL as f64);
        let surface = Self::surface(surface_height, (x, y, z));

        let ores = Self::veins((x, y, z), world_noise);
        let cave = Self::cave_noise((x, y, z), world_noise);

        match (cave, ores, surface) {
            (None, _, Some(surface)) => { surface }
            (Some(cave), _, Some(surface)) => {
                if z < surface_height - 8.0 { cave } else { surface }
            }
            // (_, Some(ore), _) => { ore }
            (_, _, _) => Tile::AIR,
        }
    }
}
