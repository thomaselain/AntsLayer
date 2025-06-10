use std::sync::Arc;
#[allow(unused)]
use std::{ ops::Range, thread::{ self, JoinHandle }, time::Duration };

use crate::{ chunk::{ biomes::NoiseParams, ChunkContent }, renderer::Renderer };

use super::{
    biomes::Params,
    manager::LoadedChunk,
    tile::Tile,
    Chunk,
    ChunkManager as Manager,
    CHUNK_HEIGHT,
    CHUNK_WIDTH,
};

pub const SEA_LEVEL: usize = ((CHUNK_HEIGHT as f64) * 0.6) as usize;
#[allow(unused)]
pub enum MapShape {
    SQUARE,
    RECT,
    // ???
    ROUND,
}

#[allow(unused)]
pub const STARTING_AREA: i32 = if cfg!(test) { 20 } else { 10 };
pub const STARTING_MAP_SHAPE: MapShape = if cfg!(test) { MapShape::SQUARE } else { MapShape::RECT };

pub type WorldNoise = Arc<[NoiseParams; 4]>;

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

            for z in 0..CHUNK_HEIGHT as i32 {
                for x in 0..CHUNK_WIDTH as i32 {
                    for y in 0..CHUNK_WIDTH as i32 {
                        // Get  noise value
                        let (nx, ny) = (
                            (x as f64) + (pos.0 as f64) * (CHUNK_WIDTH as f64),
                            (y as f64) + (pos.1 as f64) * (CHUNK_WIDTH as f64),
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
    pub fn tile_at(&self, (x, y, z): (i32, i32, i32), v: f64, world_noise: &WorldNoise) -> Tile {
        let (x, y, z) = (x as f64, y as f64, z as f64); // x, y = horizontaux, z = hauteur

        let s = world_noise[Manager::SURFACE].scale;
        // let cliff_factor = world_noise[Manager::SURFACE].frequency;

        // let base_height = world_noise[Manager::SURFACE].get((x * s, y * s, 0.0));
        // let cliff_detail = world_noise[Manager::SURFACE].get((x * s, y * s, 10.0)) * cliff_factor;

        // let surface_height = (base_height + cliff_detail) * (CHUNK_HEIGHT as f64);
        // let surface_height = v;

        let surface_noise = world_noise[Manager::SURFACE].get((x * 0.01, y * 0.01, 0.0)); // grandes structures
        let detail_noise = world_noise[Manager::DETAIL].get((x * 0.05, y * 0.05, 0.0)); // petites variations

        let surface_height = (surface_noise + detail_noise) * CHUNK_HEIGHT as f64;

        //////////////////CAVES///////////////////////////
        if z < surface_height {
            if world_noise[Manager::CAVES].get((x * s, y * s, z * s)) > 0.3 {
                return Tile::air();
            }
            //////////////////VEINS///////////////////////////
            let stratification = world_noise[Manager::LAYERS].get((x * s, y * s, z * s));
            match stratification {
                s if s < 0.2 => {
                    return Tile::dirt();
                }
                s if s < 0.5 => {
                    return Tile::clay();
                }
                _ => {
                    return Tile::air();
                }
            }
        }
        // fallback
        let raw = world_noise[Manager::SURFACE].get((x * s, y * s, z * s));
        let normalized = (raw + 1.0) / 2.0; // range [0.0, 1.0]

        let surface_height =
            (SEA_LEVEL as f64) + normalized * ((CHUNK_HEIGHT as f64) - (SEA_LEVEL as f64));

        if z < surface_height - 20.0 {
            return Tile::limestone();
        } else if z < surface_height - 5.0 {
            return Tile::clay();
        } else {
            return Tile::air();
        }
    }
}
