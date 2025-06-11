use std::{ fmt };

use bitflags::bitflags;
use sdl2::{ pixels::Color };
use serde::{ Deserialize, Serialize };

use crate::{ renderer::Renderer };

// Stores additional metadata about the Tile
const GAS_FLAGS: TileFlag = TileFlag::from_bits_retain(0b1000001);
// const FLUID_FLAGS: TileFlag = TileFlag::from_bits_retain(0b1000001);
const FLUID_FLAGS: TileFlag = GAS_FLAGS;

bitflags! {
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
    pub struct TileFlag: u16 {
        const TRAVERSABLE  = 0b0000001;
        const DIGGABLE     = 0b0000010;
        const BUILDABLE    = 0b0000100;
        const TEMPERATURE  = 0b0001000;
        const INTERACTIBLE = 0b0010000;
        const HAS_STATE    = 0b0100000;
        const TRANSPARENT  = 0b1000000;
    }
}

/// Allows ASCII display
impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self.tile_type {
            TileType::Stone(_rock) => "X",
            TileType::Soil(_soil) => "x",
            TileType::Gas(_gas) => "'",
            TileType::Fluid(_fluid) => "~",
            TileType::Custom(_) => "?",
        })?;
        Ok(())
    }
}

/// TILE STRUCT
#[derive(Hash, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct Tile {
    pub hp: u8,
    pub tile_type: TileType,
    pub properties: TileFlag,
}

#[derive(Hash, Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u16)]
pub enum TileType {
    Stone(Stone),
    Soil(Soil),
    Gas(Gas),
    Fluid(Fluid),
    Custom(u16),
}

#[derive(Hash, Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u16)]
pub enum Fluid {
    Magma,
    Water,
    SaltWater,
}
#[derive(Hash, Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u16)]
pub enum Gas {
    Air,
}

/// Hard coded Tiles
impl Tile {
    pub fn air() -> Tile {
        Tile {
            hp: 100,
            tile_type: TileType::Gas(Gas::Air),
            properties: GAS_FLAGS,
        }
    }
    pub fn sand() -> Tile {
        Tile {
            hp: 100,
            tile_type: TileType::Soil(Soil::Sand),
            properties: TileFlag::empty(),
        }
    }
    pub fn clay() -> Tile {
        Tile {
            hp: 100,
            tile_type: TileType::Soil(Soil::Clay),
            properties: TileFlag::empty(),
        }
    }
    pub fn dirt() -> Tile {
        Tile {
            hp: 100,
            tile_type: TileType::Soil(Soil::Dirt),
            properties: TileFlag::empty(),
        }
    }
    pub fn marble() -> Tile {
        Tile {
            hp: 100,
            tile_type: TileType::Stone(Stone::Marble),
            properties: TileFlag::DIGGABLE,
        }
    }
    pub fn limestone() -> Tile {
        Tile {
            hp: 100,
            tile_type: TileType::Stone(Stone::Limestone),
            properties: TileFlag::DIGGABLE,
        }
    }
    pub fn granite() -> Tile {
        Tile {
            hp: 100,
            tile_type: TileType::Stone(Stone::Granite),
            properties: TileFlag::DIGGABLE,
        }
    }
    pub fn water() -> Tile {
        Tile {
            hp: 100,
            tile_type: TileType::Fluid(Fluid::Water),
            properties: FLUID_FLAGS,
        }
    }
}

#[derive(Hash, Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u16)]
pub enum Soil {
    Dirt,
    Sand,
    Clay,
}
#[derive(Hash, Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u16)]
pub enum Stone {
    Granite,
    Marble,
    Limestone,
}

/// TEXT DISPLAY
impl Into<String> for TileType {
    fn into(self) -> String {
        String::from(match self {
            TileType::Fluid(f) => {
                match f {
                    Fluid::Magma => "Magma",
                    Fluid::Water => "Water",
                    Fluid::SaltWater => "SaltWater",
                    // _ => "Fluid",
                }
            }
            TileType::Stone(r) => {
                match r {
                    Stone::Granite => "Granite",
                    Stone::Marble => "Marble",
                    Stone::Limestone => "Limestone",
                    // _ => "Rock",
                }
            }
            TileType::Soil(s) => {
                match s {
                    Soil::Dirt => "Dirt",
                    Soil::Sand => "Sand",
                    Soil::Clay => "Clay",
                    // _ => "Soil",
                }
            }
            TileType::Gas(g) => {
                match g {
                    Gas::Air => "Air",
                    // _ => "Gas",
                }
            }
            t => todo!("Unknown tiletype : {:?}", t),
            // TileType::Custom(_) => todo!("Custom tile type"),
        })
    }
}

impl Tile {
    pub fn color(self) -> Color {
        match self.tile_type {
            TileType::Stone(stone) => {
                match stone {
                    Stone::Granite => Color::RGB(100, 100, 100),
                    Stone::Marble => Color::RGB(150, 150, 150),
                    Stone::Limestone => Color::RGB(175, 175, 175),
                }
            }
            TileType::Soil(soil) => {
                match soil {
                    Soil::Dirt => Color::RGB(111, 78, 55),
                    Soil::Sand => Color::RGB(220, 220, 10),
                    Soil::Clay => Color::RGB(182, 106, 80),
                }
            }
            TileType::Gas(_gas) => Color::RGBA(15, 15, 15, 10),
            TileType::Fluid(fluid) =>
                match fluid {
                    Fluid::Water => Color::RGBA(0, 0, 250, 150),
                    Fluid::SaltWater => Color::RGBA(0, 0, 200, 150),
                    Fluid::Magma => Color::RGBA(255, 0, 0, 200),
                }
            TileType::Custom(_) => Color::RGB(255, 155, 200),
        }
    }

    pub fn draw(self, renderer: &mut Renderer, (x, y): (i32, i32), c: Color) {
        renderer.draw_tile((x, y), c);
    }
}
