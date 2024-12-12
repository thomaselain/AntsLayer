mod tests;

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

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tile {
    pub hp: u8,
    pub coords: (i32, i32),
    pub tile_type: TileType, // Le type de la tuile
    pub material: u16, // Index ou ID du matériau (roche, métal, etc.)
    pub flags: TileFlags, // États dynamiques (traversable, liquide, etc.)
    pub extra_data: Option<u8>, // Données supplémentaires (exemple : objet)
}

impl Tile {
    pub fn new(coords: (i32, i32), tile_type: TileType, material: u16, flags: TileFlags) -> Self {
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
    pub struct TileFlags: u8 {
        const TRAVERSABLE = 0b00000001;
        const DIGGABLE    = 0b00000010;
        const BUILDABLE   = 0b00000100;
        const LIQUID      = 0b00001000;
        const CUSTOM1     = 0b00010000;
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u8)] // Utiliser 1 octet pour économiser de la mémoire
pub enum TileType {
    Empty, // 0
    Wall, // 1
    Floor, // 2
    Liquid, // 3
    Custom(u8), // 4+ (types personnalisés)
}
