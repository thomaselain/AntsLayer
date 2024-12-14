#[cfg(test)]
mod tests;

use serde::Deserialize;
use std::fs::File;
use std::io::{ self, Read };

#[derive(Deserialize, Debug, Clone)]
pub struct BiomeConfig {
    pub name: String,
    pub scale: f64,
    pub rock_threshold: f64,
    pub grass_threshold: f64,
    pub dirt_threshold: f64,
    pub water_threshold: f64,
    pub magma_threshold: f64,
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
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub biomes: Vec<BiomeConfig>,
}

impl Config {
    pub fn load() -> Result<Config, io::Error> {
        let mut file = File::open("config/biomes_config.toml")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        // Désérialisation du fichier TOML en struct Config
        toml::de::from_str(&contents).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }
}