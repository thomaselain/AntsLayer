use std::fmt;

use bitflags::bitflags;
use sdl2::{ pixels::Color, rect::Rect };
use serde::{ Deserialize, Serialize };

use crate::renderer::{ Renderer, TILE_SIZE };

use super::{ CHUNK_HEIGHT, CHUNK_WIDTH };

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

impl Tile {
    pub fn index_to_xyz(index: usize) -> (i32, i32, i32) {
        (
            // X
            (index % CHUNK_WIDTH) as i32,
            // Y
            ((index / CHUNK_WIDTH) % CHUNK_WIDTH) as i32,
            // Z
            ((index / CHUNK_WIDTH.pow(2)) % CHUNK_HEIGHT) as i32,
        )
    }
}
/// TILE STRUCT
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub struct Tile {
    pub hp: u8,
    pub tile_type: TileType,
    // extra_data (for items, units etc...)
    pub extra_data: Option<u8>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u16)]
pub enum TileType {
    Stone(Stone),
    Soil(Soil),
    Gas(Gas),
    Fluid(Fluid),
    Custom(u16),
}
impl TileType {
    pub fn is_transparent(self) -> bool {
        match self {
            TileType::Gas(_) => true,
            _ => false,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u16)]
pub enum Fluid {
    Magma,
    Water,
    SaltWater,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
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
            extra_data: None,
        }
    }
    pub fn sand() -> Tile {
        Tile {
            hp: 100,
            tile_type: TileType::Soil(Soil::Sand),
            extra_data: None,
        }
    }
    pub fn clay() -> Tile {
        Tile {
            hp: 100,
            tile_type: TileType::Soil(Soil::Clay),
            extra_data: None,
        }
    }
    pub fn dirt() -> Tile {
        Tile {
            hp: 100,
            tile_type: TileType::Soil(Soil::Dirt),
            extra_data: None,
        }
    }
    pub fn marble() -> Tile {
        Tile {
            hp: 100,
            tile_type: TileType::Stone(Stone::Marble),
            extra_data: None,
        }
    }
    pub fn limestone() -> Tile {
        Tile {
            hp: 100,
            tile_type: TileType::Stone(Stone::Limestone),
            extra_data: None,
        }
    }
    pub fn granite() -> Tile {
        Tile {
            hp: 100,
            tile_type: TileType::Stone(Stone::Granite),
            extra_data: None,
        }
    }
    pub fn water() -> Tile {
        Tile {
            hp: 100,
            tile_type: TileType::Fluid(Fluid::Water),
            extra_data: None,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u16)]
pub enum Soil {
    Dirt,
    Sand,
    Clay,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
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
                    Stone::Limestone => Color::RGB(230, 230, 230),
                }
            }
            TileType::Soil(soil) => {
                match soil {
                    Soil::Dirt => Color::RGB(111, 78, 55),
                    Soil::Sand => Color::RGB(255, 255, 143),
                    Soil::Clay => Color::RGB(182, 106, 80),
                }
            }
            TileType::Gas(_gas) => Color::RGBA(255, 255, 255, 25),
            TileType::Fluid(_fluid) => Color::BLUE,
            TileType::Custom(_) => Color::RGB(255, 155, 200),
        }
    }

    pub fn draw(self, renderer: &mut Renderer, (x, y): (i32, i32), color: Color) {
        // eprintln!("Rendering Tile at : {:?} with color {:?}", (x,y), self.color());
        renderer.canvas.set_draw_color(color);

        renderer.canvas
            .fill_rect(Rect::new(x, y, TILE_SIZE as u32, TILE_SIZE as u32))
            .expect("Failed to draw tile");
        renderer.canvas.set_draw_color(Color::BLACK);
    }
}

// Stores additional metadata about the Tile
bitflags! {
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
    pub struct TileFlags: u16 {
        const TRAVERSABLE  = 0b000001;
        const DIGGABLE     = 0b000010;
        const BUILDABLE    = 0b000100;
        const TEMPERATURE  = 0b001000;
        const INTERACTIBLE = 0b010000;
        const HAS_STATE    = 0b100000;
    }
}
