#[cfg(test)]
mod tests;
mod noise;
mod params;

use ::noise::core::perlin;
use noise::NoiseLayer;
use ::noise::{ NoiseFn, Perlin };
use params::{ AdditionalParams, Threshold };
use serde::Deserialize;
use std::fs::File;
use std::io::{ self, Read };
use std::str::FromStr;
use tile::{ FluidType, TileType };

#[derive(Deserialize, Debug, Clone)]
pub struct BiomeConfig {
    pub name: String, // Nom du biome
    pub thresholds: Vec<Threshold>, // Liste dynamique des seuils
    pub additional_params: Option<AdditionalParams>, // Paramètres facultatifs
    pub noise_layers: Vec<NoiseLayer>, // Nouvelles couches de bruit
}

impl Default for BiomeConfig {
    fn default() -> Self {
        Config::default().get_biome(DEFAULT_BIOME_NAME)
    }
}

impl BiomeConfig {
    // fn path() -> String {
    //     "config/biomes_config.toml".to_string()
    // }

    pub fn noise_from_seed(seed: u32) -> Perlin {
        Perlin::new(seed)
    }
    pub fn tile_type_from_noise(self, noise_value: f64) -> TileType {
        // eprintln!("noise value : {:?}", noise_value);
        for threshold in &self.thresholds {
            if noise_value >= threshold.min && noise_value <= threshold.max {
                return TileType::from(threshold.tile_type.as_str());
            }
        }
        TileType::Empty // Par défaut si aucune correspondance
    }
}

#[derive(Clone, Deserialize, Debug)]
pub struct Config {
    pub biomes: Vec<BiomeConfig>,
}
const DEFAULT_BIOME_NAME: &str = "Default";

impl Into<BiomeConfig> for Config {
    fn into(self) -> BiomeConfig {
        self.get_biome(DEFAULT_BIOME_NAME)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn new() -> Self {
        Self::load().expect("Failed to load config")
    }
    pub fn default_biome(&self) -> BiomeConfig {
        let default = BiomeConfig::default();

        for biome in self.biomes.clone() {
            if biome.name == DEFAULT_BIOME_NAME {
                return biome;
            }
        }

        default
    }

    pub fn get_biome(&self, name: &str) -> BiomeConfig {
        for biome in self.clone().biomes {
            if biome.name == name {
                return biome;
            }
        }
        eprintln!("Biome \"{}\" not found, set to default", name);
        BiomeConfig::default()
    }

    fn load_biomes() -> Result<Config, io::Error> {
        let file_content = include_str!("../config/biomes_config.toml");

        // Désérialisation du fichier TOML en struct Config
        let biomes: Result<Config, toml::de::Error> = toml::de::from_str(&file_content);

        match biomes {
            Ok(cfg) => { Ok(cfg) }
            Err(e) => panic!("{}", e),
        }
    }

    fn load() -> Result<Config, io::Error> {
        Self::load_biomes()
    }
}
