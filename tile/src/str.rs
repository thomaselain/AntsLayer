use crate::{ FluidType, TileType };

impl From<&str> for TileType {
    fn from(value: &str) -> Self {
        match value {
            "Water" => TileType::Fluid(FluidType::Water),
            "Magma" => TileType::Fluid(FluidType::Magma),
            "Rock" => TileType::Rock,
            "Grass" => TileType::Grass,
            "Dirt" => TileType::Dirt,
            "Wall" => TileType::Wall,
            "Sand" => TileType::Sand,
            "Floor" => TileType::Floor,
            "Empty" => TileType::Empty,
            s => todo!("Cutom tile : {:?}", s), // Valeur personnalisée par défaut
            // _ => TileType::Empty, // En cas d'erreur ou type inconnu
        }
    }
}

impl Into<String> for TileType {
    fn into(self) -> String {
        String::from(match self {
            TileType::Fluid(FluidType::Water) => "Water",
            TileType::Fluid(FluidType::Magma) => "Magma",
            TileType::Rock => "Rock",
            TileType::Grass => "Grass",
            TileType::Dirt => "Dirt",
            TileType::Wall => "Wall",
            TileType::Sand => "Sand",
            TileType::Floor => "Floor",
            TileType::Empty => "Empty",
            TileType::Custom(_type) => todo!("{_type}"),
            // _ => TileType::Empty, // En cas d'erreur ou type inconnu
        })
    }
}
