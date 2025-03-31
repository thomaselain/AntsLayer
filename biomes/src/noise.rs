use std::f64::{ MAX, MIN };

use noise::{ NoiseFn, Perlin, Seedable };
use rand::random;
use serde::Deserialize;

use crate::{ BiomeConfig, Config };

#[derive(Deserialize, Debug, Clone)]
pub struct NoiseLayer {
    pub from_seed: Option<bool>,
    pub scale: f64,
    pub weight: f64,
}

impl BiomeConfig {
    pub fn combined_noise(&self, seed: u32, perlin: &noise::Perlin, key: (f64, f64, f64)) -> f64 {
        let mut total_weight = 0.0;
        let mut value = 0.0;
        // let mut value = Perlin::new(seed).get([nx, ny]);

        for layer in &self.noise_layers {
            let layer_value = perlin.get([
                key.0 / layer.scale,
                key.1 / layer.scale,
                key.2 / layer.scale,
            ]);
            value += layer_value * layer.weight;
            total_weight += layer.weight;
        }

        if total_weight > 0.0 {
            value /= total_weight; // Normalisation
        }

        value + Config::noise_at((key.0 as i32, key.1 as i32, key.2 as i32), seed)
    }
}
impl Config {
    /// Cherche un biome en fct d'un nom
    pub fn biome_from_name(self, name: &str) -> BiomeConfig {
        let biome = self.biomes.iter().find(|biome| biome.name == name);

        // eprintln!("\n_____\nLooking for \"{}\"\n_____\n", name);

        let biome = biome.expect("Unknown biome name");

        biome.clone()
    }

    pub fn noise_at(coords: (i32, i32, i32), seed: u32) -> f64 {
        let perlin = Perlin::new(seed);
        perlin.get([0.3 * (coords.0 as f64), 0.3 * (coords.1 as f64), 0.3 * (coords.2 as f64)])
    }

    /// Cherche un biome en fct d'une coord
    /// Renvoie la valeur trouvée aux coords et le biome qui y correspond
    pub fn biome_from_coord(self, key: (i32, i32), seed: u32) -> (f64, BiomeConfig) {
        let height = Config::noise_at((key.0, key.1,0), seed);
        let biome = match height {
            -1.0..-0.5 => self.biome_from_name("Ocean"),
            -0.5..0.0 => self.biome_from_name("Coast"),
            0.0..0.2=> self.biome_from_name("Plain"),
            0.2..0.6 => self.biome_from_name("Hill"),
            0.6..1.0 => self.biome_from_name("Mountain"),
            _ => BiomeConfig::default(),
        };

        // eprintln!("Looking for biome at height : {:.3}", height);
        // println!("biome at ({} , {}) : {:?}", key.0, key.1, biome.name);
        (height, biome)
    }
}
