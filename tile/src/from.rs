use crate::{FluidType, TileType};

impl From<&str> for TileType {
    fn from(value: &str) -> Self {
        match value {
            "Water" => TileType::Fluid(FluidType::Water),
            "Magma" => TileType::Fluid(FluidType::Magma),
            "Rock" => TileType::Rock,
            "Grass" => TileType::Grass,
            "Dirt" => TileType::Dirt,
            "Wall" => TileType::Wall,
            "Sand" => TileType::Wall,
            "Floor" => TileType::Floor,
            "Empty" => TileType::Empty,
            s => todo!("Cutom tile : {:?}", s), // Valeur personnalisée par défaut
            _ => TileType::Empty, // En cas d'erreur ou type inconnu
        }
    }
}
