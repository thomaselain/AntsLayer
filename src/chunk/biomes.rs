use std::{ fmt::Debug };

use noise::{ Fbm, NoiseFn, Perlin };
use sdl2::pixels::Color;

use crate::chunk::manager::AMOUNT_OF_BIOMES;

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum Id {
    Ocean,
    Plain,
    Coast,
    Mountain,
}
impl From<i32> for Id {
    fn from(value: i32) -> Self {
        match value{
            0 => {Id::Ocean},
            1 => {Id::Coast},
            2 => {Id::Plain},
            3 => {Id::Mountain},
            _=>panic!()
        }
    }
}
impl Debug for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ocean => write!(f, "Ocean"),
            Self::Plain => write!(f, "Plain"),
            Self::Coast => write!(f, "Coast"),
            Self::Mountain => write!(f, "Mountain"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Params {
    pub id: Id,
    pub noise: NoiseParams,
    pub terrain: TerrainParams,
}

impl Default for Params {
    fn default() -> Self {
        Self { id: Id::Ocean, noise: NoiseParams::default(), terrain: TerrainParams::default() }
    }
}

#[derive(Debug, Clone)]
pub struct NoiseParams {
    // raw noise
    pub fbm: Fbm<Perlin>,

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
impl NoiseParams {
    pub fn get(&self, (x, y, z): (f64, f64, f64)) -> f64 {
        self.fbm.get([
            //
            x * self.scale,
            //
            y * self.scale,
            //
            z * self.scale,
        ])
    }
}
impl Default for NoiseParams {
    fn default() -> Self {
        Self {
            fbm: Fbm::new(42),
            octaves: 3,
            frequency: 1.0,
            lacunarity: 2.0,
            persistence: 1.1,
            scale: 0.015,
        }
    }
}

// Global noise for Biome choosing
impl NoiseParams {
    pub fn biomes() -> Self {
        Self {
            fbm: Fbm::new(1),
            octaves: 4,
            frequency: 1.0,
            lacunarity: 2.0,
            persistence: 1.5,
            scale: 0.009,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TerrainParams {
    /// Humidity
    pub humidity: f64,

    /// Temperature
    pub temperature: f64,

    /// Elevation
    pub elevation: f64,

    /// Roughness
    /// The harder the biome is, the more stony the terrain will be
    pub roughness: f64,
}

impl Into<f64> for TerrainParams {
    fn into(self) -> f64 {
        (self.elevation + self.humidity * self.roughness * self.temperature) as f64
    }
}
impl Default for TerrainParams {
    fn default() -> Self {
        Self {
            humidity: 6.0,
            temperature: 25.0,
            elevation: 2.0,
            roughness: 2.0,
        }
    }
}
impl Into<Color> for Id {
    fn into(self) -> Color {
        match self {
            Id::Ocean => Color::BLUE,
            Id::Plain => Color::GREEN,
            Id::Coast => Color::CYAN,
            Id::Mountain => Color::GREY,
        }
    }
}

// Hard coded biome parameters
impl Params {
    pub fn all() -> [Self; AMOUNT_OF_BIOMES] {
        [Self::plain(), Self::coast(), Self::ocean(), Self::mountain()]
    }
    pub fn plain() -> Self {
        Self {
            id: Id::Plain,
            terrain: TerrainParams {
                humidity: 6.0,
                temperature: 25.0,
                elevation: 2.0,
                roughness: 2.0,
            },
            noise: NoiseParams {
                fbm: Fbm::new(0),
                octaves: 5,
                frequency: 1.0,
                lacunarity: 2.0,
                persistence: 1.0,
                scale: 0.16,
            },
        }
    }
    pub fn coast() -> Self {
        Self {
            id: Id::Coast,
            terrain: TerrainParams {
                humidity: 8.0,
                temperature: 19.0,
                elevation: 1.0,
                roughness: 1.0,
            },
            noise: NoiseParams {
                fbm: Fbm::new(0),
                octaves: 5,
                frequency: 1.0,
                lacunarity: 2.0,
                persistence: 1.0,
                scale: 0.15,
            },
        }
    }
    pub fn ocean() -> Self {
        Self {
            id: Id::Ocean,
            terrain: TerrainParams {
                humidity: 10.0,
                temperature: 15.0,
                elevation: -15.0,
                roughness: 0.0,
            },
            noise: NoiseParams {
                fbm: Fbm::new(0),
                octaves: 5,
                frequency: 1.0,
                lacunarity: 2.0,
                persistence: 1.0,
                scale: 0.09,
            },
        }
    }
    pub fn mountain() -> Self {
        Self {
            id: Id::Plain,
            terrain: TerrainParams {
                humidity: 8.0,
                temperature: 8.0,
                elevation: 10.0,
                roughness: 10.0,
            },
            noise: NoiseParams {
                fbm: Fbm::new(0),
                octaves: 5,
                frequency: 1.0,
                lacunarity: 2.0,
                persistence: 1.0,
                scale: 0.1,
            },
        }
    }
}
//
