use serde::Deserialize;
use tile::FluidType;


#[derive(Deserialize, Debug, Clone)]
pub struct Threshold {
    pub min: f64,            // Valeur minimale du bruit pour ce type de tuile
    pub max: f64,            // Valeur maximale du bruit
    pub tile_type: String, // Type de tuile associé
    pub fluid_type: Option<FluidType>, // Type de tuile associé
}

#[derive(Deserialize, Debug, Clone)]
pub struct AdditionalParams {
    pub humidity: Option<f64>,       // Paramètre d'humidité
    pub temperature: Option<f64>,    // Paramètre de température
    pub roughness: Option<f64>,      // Rugosité du terrain
}
