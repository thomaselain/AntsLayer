use noise::{ NoiseFn, Perlin };
use serde::Deserialize;

use crate::renderer::TILE_SIZE;

use super::{ tile::Tile, CHUNK_WIDTH };

pub struct Biomes {
    pub params: Vec<Params>,
}

impl Biomes {
    pub fn load() -> Biomes {
        Biomes {
            params: Params::all(),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Params {
    pub name: String,
    pub noise: NoiseParams,
    pub terrain: TerrainParams,
}

#[derive(Deserialize, Debug, Clone)]
pub struct NoiseParams {
    /// https://docs.rs/noise/latest/noise/struct.Fbm.html
    /// Total number of frequency octaves to generate the noise with.
    /// The number of octaves control the amount of detail in the noise function. Adding more octaves increases the detail, with the drawback of increasing the calculation time.
    pub octaves: usize,

    /// The number of cycles per unit length that the noise function outputs.
    pub frequency: f64,

    /// A multiplier that determines how quickly the frequency increases for each successive octave in the noise function.
    /// The frequency of each successive octave is equal to the product of the previous octave’s frequency and the lacunarity value.
    /// A lacunarity of 2.0 results in the frequency doubling every octave. For almost all cases, 2.0 is a good value to use.
    pub lacunarity: f64,

    /// A multiplier that determines how quickly the amplitudes diminish for each successive octave in the noise function.
    /// The amplitude of each successive octave is equal to the product of the previous octave’s amplitude and the persistence value. Increasing the persistence produces “rougher” noise.
    pub persistence: f64,

    /// A multiplier on perlin.get() method
    pub scale: f64,
}
impl Default for NoiseParams {
    fn default() -> Self {
        Self {
            octaves: 3,
            frequency: 1.0,
            lacunarity: 1.5,
            persistence: 1.1,
            scale: 0.015,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct TerrainParams {
    /// Humidity
    pub humidity: u8,

    /// Temperature
    pub temperature: u8,

    /// Elevation
    pub elevation: u8,

    /// Roughness
    /// The harder the biome is, the more stony the terrain will be
    pub roughness: u8,
}
impl Into<f64> for TerrainParams {
    fn into(self) -> f64 {
        (self.elevation + self.humidity * self.roughness * self.temperature) as f64
    }
}

// Hard coded biome parameters
impl Params {
    pub fn all() -> Vec<Self> {
        vec![Self::plain(), Self::coast(), Self::ocean()]
    }
    pub fn plain() -> Self {
        Self {
            name: "Plain".into(),
            terrain: TerrainParams {
                humidity: 6,
                temperature: 25,
                elevation: 2,
                roughness: 2,
            },
            noise: NoiseParams::default(),
        }
    }
    pub fn coast() -> Self {
        Self {
            name: "Coast".into(),
            terrain: TerrainParams {
                humidity: 8,
                temperature: 19,
                elevation: 1,
                roughness: 1,
            },
            noise: NoiseParams::default(),
        }
    }
    pub fn ocean() -> Self {
        Self {
            name: "Ocean".into(),
            terrain: TerrainParams {
                humidity: 10,
                temperature: 15,
                elevation: 0,
                roughness: 0,
            },
            noise: NoiseParams::default(),
        }
    }
}
//
