#[cfg(test)]
mod tests;

use serde::Deserialize;
use std::fs::File;
use std::io::{ self, Read };

#[derive(Deserialize, Debug, Clone)]
pub struct BiomeConfig {
    pub name: String,
    pub scale: f64,
    pub wall_threshold: f64,
    pub floor_threshold: f64,
    pub liquid_threshold: f64,
}

impl BiomeConfig {
    pub fn default() -> BiomeConfig {
        BiomeConfig {
            name: "Plains".to_string(),
            scale: 1.0,
            wall_threshold: 0.5,
            floor_threshold: 0.0,
            liquid_threshold: -0.5,
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