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

impl Default for BiomeConfig {
    fn default() -> Self {
        Self {
            name: Default::default(),
            base_height: Default::default(),
            height_variation: Default::default(),
            thresholds: Default::default(),
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

    pub fn tile_type_from_noise(noise_value: f64, biome: &BiomeConfig) -> TileType {
        // eprintln!("{.2}", noise_value);
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
                    "Sand" => TileType::Wall,
                    "Floor" => TileType::Floor,
                    "Empty" => TileType::Empty,
                    s => todo!("Cutom tile : {:?}", s), // Valeur personnalisée par défaut
                };
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
    pub fn default_biome(self) -> BiomeConfig {
        self.get_biome(DEFAULT_BIOME_NAME)
    }
    pub fn get_biome(self, name: &str) -> BiomeConfig {
        for biome in self.biomes {
            if biome.name == name {
                return biome;
            }
        }
        eprintln!("Biome \"{}\" not found, set to default", name);
        todo!("Default biome");
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
