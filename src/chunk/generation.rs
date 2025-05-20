use noise::{ Fbm, Perlin };

use super::{ biomes::Params, tile::Tile, Chunk, CHUNK_HEIGHT };

const TEST_SEED: u32 = 12;
pub const SEA_LEVEL: usize = CHUNK_HEIGHT / 2;

impl Chunk {
    pub fn from_biome((x, y): (i32, i32), b: &Params) -> Chunk {
        let mut chunk = Chunk::new();
        let mut p = Fbm::<Perlin>::new(TEST_SEED);
        p.octaves = b.noise.octaves;
        p.frequency = b.noise.frequency;
        p.lacunarity = b.noise.lacunarity;
        p.persistence = b.noise.persistence;

        chunk.generate((x, y), b.clone(), p.clone());
        chunk 
    }
}

impl Params {
    pub fn tile_at(&self, (x, y, z): (i32, i32, i32), mut v: f64) -> Tile {
        let above_sea_level = z >= SEA_LEVEL as i32;

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
            // Should be air above sea level, regardless of noise values
            (true, "Ocean", _) => { Tile::air() }

            (false, "Ocean", 0.0..0.01) => { Tile::marble() }
            (false, "Ocean", 0.01..0.02) => { Tile::dirt() }
            (false, "Ocean", _) => { Tile::water() }

            (_, _, 0.0..0.1) => { Tile::dirt() }
            (_, _, 0.2..0.3) => { Tile::marble() }

            (_,_,_) => { Tile::air() }
        }
    }
}