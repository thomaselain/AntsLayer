use std::i32::{ MAX, MIN };

use noise::{ NoiseFn, Perlin, Seedable };
use rand::random;
use serde::Deserialize;

use crate::{ BiomeConfig, Config };

#[derive(Deserialize, Debug, Clone)]
pub struct NoiseLayer {
    pub scale: f64,
    pub weight: f64,
}

impl BiomeConfig {
    pub fn combined_noise(&self, perlin: &noise::Perlin, nx: f64, ny: f64) -> f64 {
        let mut value = 0.0;
        let mut total_weight = 0.0;

        for layer in &self.noise_layers {
            let layer_value = perlin.get([nx / layer.scale, ny / layer.scale]);
            value += layer_value * layer.weight;
            total_weight += layer.weight;
        }

        if total_weight > 0.0 {
            value /= total_weight; // Normalisation
        }

        value
    }
}
impl Config {
    /// Cherche un biome en fct d'un nom
    pub fn biome_from_name(self, name: &str) -> BiomeConfig {
        let biome = self.biomes.iter().find(|biome| biome.name == name);


        eprintln!("\n_____\nLooking for \"{}\"\n_____\n", name);

        let biome = biome.expect("Unknown biome name");

        biome.clone()
    }
    /// Cherche un biome en fct d'une hauteur
    pub fn biome_from_height(self, height: i32) -> BiomeConfig {
        match height {
            MIN..-5 => BiomeConfig::default(),
            -5..-1 => self.biome_from_name("Ocean"),
            -1..1 => self.biome_from_name("Coast"),
            1..3 => self.biome_from_name("Hill"),
            3..5 => self.biome_from_name("Plain"),
            5 => self.biome_from_name("Mountain"),
            6..=MAX => BiomeConfig::default(),
        }
    }

    /// Cherche un biome en fct d'une coord
    pub fn biome_from_coord(self, key: (f64, f64)) -> BiomeConfig {
        let world_scale = 0.1;
        let perlin = Perlin::default();
        let mut height = perlin.get([world_scale * key.0, world_scale * key.1]);

        height /= 0.1; // Normalisation

        let biome = self.biome_from_height(height as i32);
        println!("biome at ({} , {}) : {:?}", key.0, key.1, biome);
        biome
    }
}

#[test]
fn biomes_perlin() {
    let config = Config::default();
    let value = config.biome_from_coord((0.1, 1.0));

    eprintln!("{:?}", value);
}
