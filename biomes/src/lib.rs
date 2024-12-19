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

impl BiomeConfig {
    pub fn default() -> BiomeConfig {
        BiomeConfig {
            name: "Default_biome".to_string(),
            height_variation: 0.5,
            base_height:0.5,
            thresholds: Vec::new(),
            noise_layers: Vec::new(),
            additional_params: None,
        }
    }
    fn path() -> String {
        "config/biomes_config.toml".to_string()
    }

    pub fn noise_from_seed(seed:u32) -> Perlin {
        Perlin::new(seed)
    }

    pub fn tile_type_from_noise(noise_value: f64, biome: &BiomeConfig) -> TileType {
        for threshold in &biome.thresholds {
            if noise_value >= threshold.min && noise_value < threshold.max {
                if threshold.tile_type == "Fluid" {
                    // Si c'est un fluide, vérifie le type de fluide
                    if let Some(fluid_type) = &threshold.fluid_type {
                        return match fluid_type.as_string().as_str() {
                            "Water" => TileType::Fluid(FluidType::Water),
                            "Magma" => TileType::Fluid(FluidType::Magma),
                            _ => TileType::Empty, // En cas d'erreur ou type inconnu
                        };
                    }
                }
                // Retourne un type de tuile standard
                return match threshold.tile_type.as_str() {
                    "Rock" => TileType::Rock,
                    "Grass" => TileType::Grass,
                    "Dirt" => TileType::Dirt,
                    "Wall" => TileType::Wall,
                    "Floor" => TileType::Floor,
                    "Empty" => TileType::Empty,
                    _ => TileType::Custom(0), // Valeur personnalisée par défaut
                };
            }
        }
        TileType::Empty // Par défaut si aucune correspondance
    }
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub biomes: Vec<BiomeConfig>,
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
