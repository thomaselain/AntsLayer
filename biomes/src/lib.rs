#[cfg(test)]
mod tests;
mod noise;
mod params;

use noise::NoiseLayer;
use ::noise::Perlin;
use params::{ AdditionalParams, Threshold };
use serde::Deserialize;
use std::fs::File;
use std::io::{ self, Read };
use std::slice::SliceIndex;
use std::str::FromStr;
use tile::{ FluidType, TileType };

#[derive(Deserialize, Debug, Clone)]
pub struct BiomeConfig {
    pub name: String, // Nom du biome
    pub base_height: f64,
    pub height_variation: f64, // Variation de hauteur
    pub thresholds: Vec<Threshold>, // Liste dynamique des seuils
    pub additional_params: Option<AdditionalParams>, // Paramètres facultatifs
    pub noise_layers: Vec<NoiseLayer>, // Nouvelles couches de bruit
}

impl Default for BiomeConfig {
    fn default() -> Self {
        Self {
            name: String::from_str(DEFAULT_BIOME_NAME).unwrap(),
            base_height: 0.0,
            height_variation: 0.0,
            thresholds: vec![
                Threshold {
                    min: -1.0,
                    max: -0.5,
                    tile_type: TileType::Fluid(FluidType::Water),
                    fluid_type: Some(FluidType::Water),
                },
                Threshold {
                    min: -0.5,
                    max: 0.0,
                    tile_type: TileType::Sand,
                    fluid_type: None,
                },
                Threshold {
                    min: 0.0,
                    max: 0.5,
                    tile_type: TileType::Dirt,
                    fluid_type: None,
                }, Threshold {
                    min: 0.5,
                    max: 0.75,
                    tile_type: TileType::Grass,
                    fluid_type: None,
                },Threshold {
                    min: 0.75,
                    max: 1.0,
                    tile_type: TileType::Rock,
                    fluid_type: None,
                }
            ],
            additional_params: Default::default(),
            noise_layers: Default::default(),
        }
    }
}

impl BiomeConfig {
    fn path() -> String {
        "config/biomes_config.toml".to_string()
    }

    pub fn noise_from_seed(seed: u32) -> Perlin {
        Perlin::new(seed)
    }

    pub fn tile_type_from_noise(self, noise_value: f64) -> TileType {
        // eprintln!("{.2}", noise_value);
        for threshold in &self.thresholds {
            if noise_value >= threshold.min && noise_value < threshold.max {
                return threshold.tile_type;
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
