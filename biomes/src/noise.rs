use std::f64::{ MAX, MIN };

use coords::Coords;
use noise::{ NoiseFn, Perlin, Seedable };
use rand::random;
use serde::Deserialize;

use crate::{ BiomeConfig, Config };

const WORLD_SCALE:f64 = 0.1;

#[derive(Deserialize, Debug, Clone)]
pub struct NoiseLayer {
    pub from_seed: Option<bool>,
    pub scale: f64,
    pub weight: f64,
}

impl BiomeConfig {
    pub fn combined_noise(&self, perlin:& Perlin, pos: Coords<i32>) -> f64 {
        let mut total_weight = 0.0;
        let mut value = 0.0;
        
        for layer in &self.noise_layers {
            let layer_value = perlin.get([
                pos.x_f64() / layer.scale,
                pos.y_f64() / layer.scale,
                pos.z_f64() / layer.scale,
            ]);
            value += layer_value * layer.weight;
            total_weight += layer.weight;
        }

        if total_weight > 0.0 {
            value /= total_weight; // Normalisation
        }

        value + Config::noise_at(perlin, pos)
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

    pub fn noise_at(perlin: &Perlin, coords: Coords<i32>) -> f64 {
        perlin.get([
            WORLD_SCALE * coords.x_f64(),
            WORLD_SCALE * coords.y_f64(),
            WORLD_SCALE * coords.z_f64()])
    }

    /// Cherche un biome en fct d'une coord
    /// Renvoie la valeur trouvée aux coords et le biome qui y correspond
    pub fn biome_from_coord(self, key: (i32, i32)) -> (f64, BiomeConfig) {
        let height = 0.5;
        // let height = Config::noise_at(Coords::new(key.0, key.1, 0), seed);
        let biome = match height {
            -1.0..-0.5 => self.biome_from_name("Ocean"),
            -0.5..0.0 => self.biome_from_name("Coast"),
            0.0..0.2 => self.biome_from_name("Plain"),
            0.2..0.6 => self.biome_from_name("Hill"),
            0.6..1.0 => self.biome_from_name("Mountain"),
            _ => BiomeConfig::default(),
        };

        eprintln!("Looking for biome at height : {:.3}", height);
        println!("biome at ({} , {}) : {:?}", key.0, key.1, biome.name);
        (height, biome)
    }
}
