use std::{ collections::HashMap, fmt::Debug, sync::{mpsc, Arc} };

use noise::{ Fbm, NoiseFn, Perlin };

use crate::chunk::{ manager::WorldNoise, ChunkManager as Manager };

#[derive(Debug, Clone, Copy)]
pub struct Biome {
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
impl Biome {
    pub fn get(self) -> f64 {
        let climate_flatness = self.temperature * 1.05 + self.humidity * 1.04;
        let elevation_boost = self.elevation * 1.1;
        let biome_factor = (elevation_boost - climate_flatness).max(0.0);
        let roughness_effect = self.roughness * 1.3;

        biome_factor * roughness_effect
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
            octaves: 4,
            frequency: 0.5,
            lacunarity: 2.0,
            persistence: 1.1,
            scale: 0.001,
        }
    }
}

// Hard coded biome parameters
impl Biome {
    pub fn get_biome_params(x: f64, y: f64, world_noise: &WorldNoise) -> Self {
        let scale = world_noise[Manager::SURFACE].scale; // grande échelle = variation lente

        let humidity = world_noise[Manager::HUMIDITY].get((x * scale, y * scale, 0.0));
        let temperature = world_noise[Manager::TEMPERATURE].get((x * scale, y * scale, 5.0));
        let elevation = world_noise[Manager::ELEVATION].get((x * scale, y * scale, 15.0));
        let roughness = world_noise[Manager::ROUGHNESS].get((x * scale, y * scale, 20.0));

        let min = -1.0;
        let max = 1.0;
        Self {
            humidity: humidity.clamp(min, max),
            temperature: temperature.clamp(min, max),
            elevation: elevation.clamp(min, max),
            roughness: roughness.clamp(min, max),
        }
    }
}
//
impl Manager {
    pub const SURFACE: usize = 0;
    pub const VARIATIONS: usize = 1;
    pub const DETAIL: usize = 2;
    pub const CAVES: usize = 3;
    pub const TUNNELS: usize = 4;
    pub const VEINS: usize = 5;
    pub const HUMIDITY: usize = 6;
    pub const ELEVATION: usize = 7;
    pub const ROUGHNESS: usize = 8;
    pub const TEMPERATURE: usize = 9;

    pub fn empty() -> Self {
        let (rx, tx) = mpsc::channel();
        Self {
            world_noise: Arc::new([
                // Surface
                NoiseParams {
                    fbm: Fbm::new(1),
                    octaves: 5,
                    frequency: 0.5,
                    lacunarity: 2.0,
                    persistence: 1.5,
                    scale: 0.012,
                },
                // Variations
                NoiseParams {
                    fbm: Fbm::new(1),
                    octaves: 1,
                    frequency: 1.0,
                    lacunarity: 2.0,
                    persistence: 1.0,
                    scale: 0.1,
                },
                // Details
                NoiseParams {
                    fbm: Fbm::new(1),
                    octaves: 4,
                    frequency: 1.0,
                    lacunarity: 2.0,
                    persistence: 1.0,
                    scale: 0.1,
                },
                // Caves
                NoiseParams {
                    fbm: Fbm::new(64),
                    octaves: 4,
                    frequency: 1.4,
                    lacunarity: 2.0,
                    persistence: 1.5,
                    scale:0.9, // IS MULTIPLIED BY SURFACE SCALE
                },
                // Tunnels
                NoiseParams {
                    fbm: Fbm::new(65),
                    octaves: 1,
                    frequency: 1.0,
                    lacunarity: 2.0,
                    persistence: 0.1,
                    scale: 1.0, // IS MULTIPLIED BY SURFACE SCALE
                },
                // Layers
                NoiseParams {
                    fbm: Fbm::new(3),
                    octaves: 4,
                    frequency: 0.02,
                    lacunarity: 2.0,
                    persistence: 0.4,
                    scale: 0.09,
                },
                // HUMIDITY
                NoiseParams::default(),
                // ELEVATION
                NoiseParams {
                    fbm: Fbm::new(333),
                    octaves: 3,
                    frequency: 1.1,
                    lacunarity: 2.0,
                    persistence: 1.1,
                    scale: 0.0001,
                },
                // ROUGHNESS
                NoiseParams::default(),
                // TEMPERATURE
                NoiseParams {
                    fbm: Fbm::new(33),
                    octaves: 3,
                    frequency: 1.2,
                    lacunarity: 2.0,
                    persistence: 0.99,
                    scale: 0.001,
                },
            ]),
            rx,
            tx,
            pending_chunks: vec![],
            loaded_chunks: HashMap::new(),
        }
    }
}
