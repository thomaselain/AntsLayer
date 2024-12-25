mod tests;
pub mod from;

use coords::aliases::TilePos;
use serde::{ Serialize, Deserialize };

// Chatgpt le goat
// +-------------------+               +----------------------------+
// |      Tuile        | <------------> | Fonctionnalités de la Tuile|
// +-------------------+               +----------------------------+
// | - Type            |               | - can_walk_on()            |
// | - Ressource       |               | - interact()               |
// | - État dynamique  |               | - update_state()           |
// | - Solide          |               | - get_lighting()           |
// | - Température     |               | - mine_resource()          |
// | - Traversable     |               | - check_collision()        |
// |                   |               | - degrade()                |
// +-------------------+               +----------------------------+

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u16)]
pub enum TileType {
    Empty,
    Wall,
    Rock,
    Sand,
    Dirt,
    Grass,
    Floor,
    Fluid(FluidType),
    Custom(u16), //+ (types personnalisés)
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tile {
    pub hp: u8,
    pub coords: TilePos,
    pub tile_type: TileType, // Le type de la tuile
    pub flags: TileFlags, // États dynamiques (traversable, liquide, etc.)
    pub material: u16, // Index ou ID du matériau (roche, métal, etc.)
    pub extra_data: Option<u8>, // Données supplémentaires (exemple : objet)
}

impl Tile {
    pub fn new(coords: TilePos, tile_type: TileType, material: u16, flags: TileFlags) -> Self {
        Self {
            hp: u8::MAX,
            coords,
            tile_type,
            material,
            flags,
            extra_data: None,
        }
    }

    pub fn set_extra_data(&mut self, data: u8) {
        self.extra_data = Some(data);
    }
}

use bitflags::bitflags;

bitflags! {
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
    pub struct TileFlags: u16 {
        const TRAVERSABLE  = 0b00000001;
        const DIGGABLE     = 0b00000010;
        const BUILDABLE    = 0b00000100;
        const LIQUID       = 0b00001000;
        const TEMPERATURE  = 0b00010000;
        const BROKEN       = 0b00100000;
        const INTERACTIBLE = 0b01000000;
        const HAS_STATE    = 0b10000000;
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u16)]
pub enum FluidType {
    Magma,
    Water,
}

impl FluidType {
    pub fn flow_speed(self) -> u8 {
        match self {
            FluidType::Magma => 1,
            FluidType::Water => 3,
        }
    }
    pub fn as_string(self) -> String {
        match self {
            FluidType::Magma => "Magma".to_string(),
            FluidType::Water => "Water".to_string(),
        }
    }
}
