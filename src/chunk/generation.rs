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
    CHUNK_HEIGHT,
    CHUNK_WIDTH,
};

#[allow(unused)]
pub enum MapShape {
    SQUARE,
    RECT,
    // ???
    ROUND,
}

#[allow(unused)]
pub const STARTING_AREA: i32 = if cfg!(test) { 25 } else { 10 };
pub const STARTING_MAP_SHAPE: MapShape = if cfg!(test) { MapShape::SQUARE } else { MapShape::RECT };

const CAVE_HEIGHT: f64 = (CHUNK_HEIGHT as f64) * 0.4; // Keep under *0.5
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
    fn surface(surface_height: f64, (x, y, z): (f64, f64, f64)) -> Tile {
        if z < surface_height - 5.0 {
            Tile::granite()
        } else if z < surface_height - 4.0 {
            Tile::clay()
        } else if z < surface_height - 2.0 {
            Tile::dirt()
        } else {
            Tile::air()
        }
    }

    fn stratification(
        surface_height: f64,
        (x, y, z): (f64, f64, f64),
        world_noise: &WorldNoise
    ) -> Option<Tile> {
        let scale = world_noise[Manager::LAYERS].scale;
        let noise = world_noise[Manager::LAYERS].get((x * scale, y * scale, z * scale));

        match noise {
            s if s < 0.0 => {
                Some(Tile::dirt());
            }
            s if s < 0.1 => {
                Some(Tile::clay());
            }
            s if s < 0.15 => {
                Some(Tile::limestone());
            }
            s if s < 0.25 => {
                Some(Tile::marble());
            }
            s if s < 0.3 => {
                Some(Tile::water());
            }
            _ => {
                Some(Tile::air());
            }
        }
        None
    }

    fn cave_noise(
        surface_height: f64,
        (x, y, z): (f64, f64, f64),
        world_noise: &WorldNoise
    ) -> Option<Tile> {
        let scale = world_noise[Manager::CAVES].scale;
        let noise = world_noise[Manager::CAVES].get((x * scale, y * scale, z * scale));
        if z < CAVE_HEIGHT && z < surface_height {
            if noise < -0.24 {
                return Some(Tile::air());
            }
            // Stones
            #[allow(unused)] // Needed for parenthesis (C pas moi c rust)
            if
                let Some(stone) = (match noise {
                    0.0..0.1 => Some(Tile::dirt()),
                    0.1..0.3 => Some(Tile::marble()),
                    0.3..0.4 => Some(Tile::limestone()),
                    0.4..0.45 => Some(Tile::water()),
                    0.45..0.5 => Some(Tile::clay()),
                    _ => None,
                })
            {
                return Some(stone);
            } else {
                return Some(Tile::dirt());
            }
        }
        None
    }
    pub fn tile_at(&self, (x, y, z): (i32, i32, i32), v: f64, world_noise: &WorldNoise) -> Tile {
        let (x, y, z) = (x as f64, y as f64, z as f64);

        let scale = world_noise[Manager::SURFACE].scale;

        let surface_noise = world_noise[Manager::SURFACE].get((x * scale, y * scale, 0.0));
        let detail_noise = world_noise[Manager::DETAIL].get((
            x * scale * 0.5,
            y * scale * 0.5,
            0.0,
        ));

        let surface_height = (surface_noise + detail_noise) * (CHUNK_HEIGHT as f64 - SEA_LEVEL as f64);

        //////////////////LAYERS///////////////////////////
        if let Some(tile) = Self::stratification(surface_height, (x, y, z), world_noise) {
            return tile;
        }

        //////////////////CAVES///////////////////////////
        if let Some(tile) = Self::cave_noise(surface_height, (x, y, z), world_noise) {
            return tile;
        }

        Self::surface(surface_noise * (CHUNK_HEIGHT as f64) + (SEA_LEVEL as f64), (x, y, z))
    }
}
