use super::{ biomes::Params, manager::LoadedChunk, tile::Tile, Chunk, CHUNK_HEIGHT };

pub const SEA_LEVEL: usize = (CHUNK_HEIGHT as f64 * 0.7) as usize;

impl Chunk {
    pub fn from_biome(pos: (i32, i32), b: &Params) -> LoadedChunk {
        let mut chunk = Chunk::new(pos);        

        chunk.c.generate(pos, b.clone(), b.noise.fbm.clone());
        chunk
    }
}

impl Params {
    pub fn tile_at(&self, (x, y, z): (i32, i32, i32), v: f64) -> Tile {
        let above_sea_level = z > (SEA_LEVEL as i32);

        let v = if z > SEA_LEVEL as i32 {
            v *0.3* z as f64
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
                self.name.as_str(),
                //Noise value (from 0.0 to 1.0)
                v,
            )
        {
            // -----------------
            // ----- Ocean -----
            // -----------------
            // Should be air above sea level, regardless of noise values
            (true, "Ocean", _) => Tile::air(),

            (false, "Ocean", 0.0..0.01) => Tile::marble(),
            (false, "Ocean", 0.01..0.9) => Tile::dirt(),
            (false, "Ocean", _) => Tile::water(),
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
