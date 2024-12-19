#[cfg(test)]
mod tests;

use serde::Deserialize;
use std::fs::File;
use std::io::{ self, Read };
use tile::{FluidType, TileType};

#[derive(Deserialize, Debug, Clone)]
pub struct BiomeConfig {
    pub name: String,
    pub scale: f64,
    pub rock_threshold: f64,
    pub grass_threshold: f64,
    pub dirt_threshold: f64,
    pub water_threshold: f64,
    pub magma_threshold: f64,

    pub base_height: f64,        // Par défaut : 0.0
    pub height_variation: f64,  // Par défaut : 1.0
}

impl BiomeConfig {
    pub fn default() -> BiomeConfig {
        BiomeConfig {
            name: "Default_biome".to_string(),
            scale: 1.0,
            rock_threshold: 0.8,
            grass_threshold: 0.4,
            water_threshold: -0.4,
            dirt_threshold: -0.8,
            magma_threshold: -0.9,
            base_height: 0.5,
            height_variation: 0.5,
        }
    }
    fn path() -> String {
        format!("config/biomes_config.toml")
    }

    pub fn tile_type_from_noise(noise_value: f64, biome: &BiomeConfig) -> TileType {
        if noise_value > biome.rock_threshold {
            TileType::Rock
        } else if noise_value > biome.grass_threshold {
            TileType::Grass
        } else if noise_value > biome.dirt_threshold {
            TileType::Dirt
        } else if noise_value > biome.water_threshold {
            TileType::Fluid(FluidType::Water)
        } else if noise_value > biome.magma_threshold {
            TileType::Fluid(FluidType::Magma)
        } else {
            TileType::Empty
        }
    }
    
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub biomes: Vec<BiomeConfig>,
}

impl Config {
    pub fn new() -> Self {
        Self::load().expect("Failed to load config")
    }

    pub fn get_biome(self, name: &str) -> BiomeConfig {
        for biome in self.biomes {
            if biome.name == name {
                return biome;
            }
        }
        eprintln!("Biome \"{}\" not found, set to default", name);
        BiomeConfig::default()
    }
    fn load_biomes() -> Result<Config, io::Error> {
        let mut file = File::open(BiomeConfig::path())?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        // Désérialisation du fichier TOML en struct Config
        toml::de::from_str(&contents).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    fn load() -> Result<Config, io::Error> {
        Self::load_biomes()
    }
}
