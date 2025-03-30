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
    pub fn combined_noise(&self, seed: u32, perlin: &noise::Perlin, key: (f64, f64)) -> f64 {
        let mut total_weight = 0.0;
        let mut value = 0.0;
        // let mut value = Perlin::new(seed).get([nx, ny]);

        for layer in &self.noise_layers {
            let layer_value = perlin.get([key.0 / layer.scale, key.1 / layer.scale]);
            value += layer_value * layer.weight;
            total_weight += layer.weight;
        }

        if total_weight > 0.0 {
            value /= total_weight; // Normalisation
        }

        value + Config::height_at((key.0 as i32, key.1 as i32), seed)
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
    /// Cherche un biome en fct d'une hauteur
    pub fn biome_from_height(self, height: f64) -> BiomeConfig {
        match height {
            MIN..0.0 => self.biome_from_name("Ocean"),
            0.0..5.0 => self.biome_from_name("Coast"),
            5.0..10.0 => self.biome_from_name("Plain"),
            10.0..32.0 => self.biome_from_name("Hill"),
            32.0..=MAX => self.biome_from_name("Mountain"),
            _ => panic!("Unknown height"),
        }
    }

    pub fn height_at(key: (i32, i32), seed: u32) -> f64 {
        let perlin = Perlin::new(seed);
        perlin.get([0.1 * (key.0 as f64), 0.1 * (key.1 as f64)]) / 0.1
    }

    /// Cherche un biome en fct d'une coord
    /// Renvoie la hauteur trouvée aux coords et le biome qui y correspond
    pub fn biome_from_coord(self, key: (i32, i32), seed: u32) -> (f64, BiomeConfig) {
        let height = Config::height_at(key, seed);
        let biome = self.biome_from_height(height);

        // eprintln!("Looking for biome at height : {:.3}", height);
        // println!("biome at ({} , {}) : {:?}", key.0, key.1, biome.name);
        (height, biome)
    }
}
