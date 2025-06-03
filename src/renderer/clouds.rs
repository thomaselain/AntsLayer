use noise::Fbm;

//
use crate::chunk::biomes::NoiseParams;

impl NoiseParams {
    pub fn clouds() -> Self {
        Self {
            fbm: Fbm::new(69_42),
            octaves: 2,
            frequency: 1.0,
            lacunarity: 2.0,
            persistence: 0.8,
            scale: 0.04,
        }
    }
}