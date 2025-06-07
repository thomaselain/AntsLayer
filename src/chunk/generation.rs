#[allow(unused)]
use std::{ ops::Range, thread::{ self, JoinHandle }, time::Duration };

use crate::{ chunk::ChunkContent, renderer::Renderer };

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

impl Manager {
    pub fn look_for_new_chunks(&mut self, renderer: &mut Renderer) {
        let (x_min, x_max, y_min, y_max) = renderer.camera_range_i32();
        let mut new_chunks = vec![];
        for x in x_min..x_max {
            for y in y_min..y_max {
                if !self.loaded_chunks.contains_key(&(x, y)) {
                    new_chunks.push(self.generate(&(x, y)));
                }
            }
        }
        //
        for handle in new_chunks {
            let chunk = handle.join().unwrap();
            self.loaded_chunks.insert(chunk.pos, chunk);
        }
    }
    pub fn generate(&self, (x, y): &(i32, i32)) -> JoinHandle<LoadedChunk> {
        let biome = &self.biome_at((*x, *y));

        Chunk::from_biome((*x, *y), biome)
    }
    pub fn generate_range(
        &self,
        x_range: Range<i32>,
        y_range: Range<i32>
    ) -> Vec<JoinHandle<LoadedChunk>> {
        let mut m = Vec::new();

        for j in y_range {
            for i in x_range.clone() {
                m.push(self.generate(&(i, j)));
            }
        }

        m
    }
}
impl Chunk {
    pub fn new() -> Self {
        Self { content: ChunkContent::new() }
    }
    pub fn from_biome(pos: (i32, i32), b: &Params) -> JoinHandle<LoadedChunk> {
        Chunk::generate(pos, b.clone())
    }
}

impl Chunk {
    pub fn generate(pos: (i32, i32), b: Params) -> JoinHandle<LoadedChunk> {
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
                            nx as f64,
                            //
                            ny as f64,
                            //
                            z as f64,
                        ));

                        // Set the tile
                        let tile = b.tile_at((x, y, z), v);
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
    pub fn tile_at(&self, (x, y, z): (i32, i32, i32), v: f64) -> Tile {
        let above_sea_level = z > (SEA_LEVEL as i32);

        let v = if z > (SEA_LEVEL as i32) {
            // v * 0.3 * (z as f64)
            0.0
        } else {
            // v - 0.2
            v
        };

        // let v = if v <= 0.1 { 0.0 } else { v };

        // Check biome conditions
        match
            (
                //Boolean that tells if z > SEA_LEVEL
                above_sea_level,
                //Biome's name
                self.id,
                //Noise value (from 0.0 to 1.0)
                v,
            )
        {
            // Should be air above sea level, regardless of noise values
            (true, _, _) => Tile::air(),
            // -----------------
            // ----- Ocean -----
            // -----------------

            // (false, Id::Ocean, 0.0..0.01) => Tile::marble(),
            // (false, Id::Ocean, 0.01..0.9) => Tile::dirt(),
            // (false, Id::Ocean, _) => Tile::water(),
            // ------------------

            // -----------------
            // ---- DEFAULT ----
            // -----------------
            (_, _, 0.0) => { Tile::granite() }
            (_, _, 0.0..0.3) => Tile::marble(),
            (_, _, 0.3..0.4) => Tile::limestone(),

            (_, _, 0.4..0.7) => Tile::dirt(),
            (_, _, 0.7..0.8) => Tile::sand(),
            (_, _, 0.8..0.9) => Tile::clay(),

            (_, _, _) => Tile::air(),
            // ------------------
        }
    }
}
