use std::str::FromStr;

use serde::Deserialize;
use tile::{ FluidType, TileType };

#[derive(Deserialize, Debug, Clone)]
pub struct Threshold {
    pub min: f64, // Valeur minimale du bruit pour ce type de tuile
    pub max: f64, // Valeur maximale du bruit
    pub tile_type: TileType, // Type de tuile associé
    pub fluid_type: Option<FluidType>, // Type de tuile associé
}
impl Default for Threshold {
    fn default() -> Self {
        Self { min: -1.0, max: 1.0, tile_type: TileType::Empty, fluid_type: None }
    }
}

impl Threshold {
    pub fn new(min: f64, max: f64, tile_type: &str, fluid_type: Option<FluidType>) -> Self {
        Self { min, max, tile_type: tile_type.into(), fluid_type }
    }
}
#[derive(Deserialize, Debug, Clone)]
pub struct AdditionalParams {
    pub humidity: Option<f64>, // Paramètre d'humidité
    pub temperature: Option<f64>, // Paramètre de température
    pub roughness: Option<f64>, // Rugosité du terrain
}
