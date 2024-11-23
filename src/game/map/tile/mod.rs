pub(crate) mod buildings;
pub(crate) mod minerals;
pub(crate) mod terrain;

use buildings::{Buildable, BuildingType};
use minerals::{Mineral, MineralType};

use crate::game::units::{Item, RaceType};

use super::{Map, TerrainType};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum TileType {
    Mineral(MineralType),
    Building(Buildable<RaceType>),
    TerrainType(TerrainType),
}

impl TileType {

    /// TODO : better
    pub fn from_str(str: &str) -> Result<TileType, &str> {
        match str {
            "AIR" => Ok(TileType::TerrainType(TerrainType::AIR)),
            "WATER" => Ok(TileType::TerrainType(TerrainType::WATER)),
            "ROCK" => Ok(TileType::Mineral(MineralType::ROCK)),
            "DIRT" => Ok(TileType::Mineral(MineralType::DIRT)),
            "IRON" => Ok(TileType::Mineral(MineralType::IRON)),
            "GOLD" => Ok(TileType::Mineral(MineralType::GOLD)),
            "MOSS" => Ok(TileType::Mineral(MineralType::MOSS)),
            _ => Err(str),
        }
    }

    pub fn to_ascii(self) -> String {
        match self {
            TileType::Mineral(mineral_type) => mineral_type.to_char(),
            _ => " ".to_string(),
        }
    }
    /// Can be harvested
    /// ROCK - for
    /// IRON - for
    /// GOLD - for
    /// MOSS - for breeding
    pub fn is_collectable(self) -> bool {
        match self {
            TileType::Mineral(MineralType::ROCK) => false,
            TileType::Mineral(MineralType::IRON) => false,
            TileType::Mineral(MineralType::GOLD) => false,
            TileType::Mineral(MineralType::MOSS) => true,
            _ => false,
        }
    }
    pub fn into_mineral(self) -> Option<Mineral> {
        match self {
            TileType::Mineral(mineral_type) => Some(Mineral(mineral_type)),
            _ => None,
        }
    }
    pub fn into_item(self) -> Option<Item> {
        if self.is_collectable() {
            Some(Item::Mineral(self))
        } else {
            return None;
        }
    }
}

impl TileType {
    pub fn is_walkable(self) -> bool {
        match self {
            TileType::TerrainType(TerrainType::AIR) => true,
            TileType::TerrainType(TerrainType::WATER) => true,
            TileType::Mineral(MineralType::MOSS) => true,
            TileType::Building(_) => true,
            _ => false,
        }
    }
    pub fn get_mineral_type(self) -> Result<MineralType, Self> {
        match self {
            TileType::Mineral(mineral_type) => Ok(mineral_type),
            _ => Err(self),
        }
    }
    pub fn get_building_type(self) -> Option<BuildingType> {
        match self {
            TileType::Building(buildable) => buildable.to_tile_type().get_building_type(),
            _ => None,
        }
    }
    pub fn get_terrain_type(self) -> Option<TerrainType> {
        match self {
            TileType::TerrainType(terrain_type) => Some(terrain_type),
            _ => None,
        }
    }

    pub fn as_tile(self) -> Tile {
        let mut tile = Tile::new();
        match self {
            TileType::TerrainType(terrain_type) => {
                tile.set_single(terrain_type.to_tile_type());
            }
            TileType::Mineral(mineral_type) => {
                tile.set_single(mineral_type.to_tile_type());
            }
            TileType::Building(buildable) => {
                tile.set_single(TileType::Building { 0: buildable });
            }
        };
        tile
    }
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Tile(
    pub Option<TerrainType>,
    pub Option<Mineral>,
    pub Option<Buildable<RaceType>>,
);

impl Tile {
    pub fn new() -> Tile {
        Tile(None, None, None)
    }
    pub fn eq(self, other: Option<Tile>) -> bool {
        if other.is_some()
            && self.0 == other.unwrap().0
            && self.1 == other.unwrap().1
            && self.2 == other.unwrap().2
        {
            true;
        }
        false
    }
    pub fn any(other: TileType) -> Tile {
        let mut tile = Tile::new();
        tile.set_single(other);
        tile
    }

    pub fn has(self, other: TileType) -> Result<Tile, ()> {
        // Mineral comparaison
        if let Ok(terrain) = self.get_terrain() {
            let tile_type = terrain.to_tile_type();
            if tile_type == other {
                return Ok(tile_type.as_tile());
            }
        }

        // Terrain comparaison
        if let Ok(mineral) = self.get_mineral() {
            let tile_type = mineral.0.to_tile_type();
            if tile_type == other {
                return Ok(tile_type.as_tile());
            }
        }

        // Building comparaison
        if let Ok(building) = self.get_buildable() {
            let tile_type = building.to_tile_type();
            if tile_type == other {
                return Ok(tile_type.as_tile());
            }
        }

        Err(())
    }

    pub fn has_some(self) -> bool {
        if self.0.is_some() || self.1.is_some() || self.2.is_some() {
            true;
        }
        false
    }
    pub fn to_tile_type(&mut self) -> (Option<TileType>, Option<TileType>, Option<TileType>) {
        let mut new_tile = (None, None, None);

        match (self.0, self.1, self.2) {
            (Some(terrain), _, _) => new_tile.0 = Some(TileType::TerrainType(terrain)),
            (_, Some(mineral), _) => new_tile.1 = Some(TileType::Mineral(mineral.0)),
            (_, _, Some(building)) => todo!("Building color"), //new_tile.2 = Some(building.to_tile_type()),
            _ => {
                todo!("Empty tiles conversion")
            }
        }
        new_tile
    }
    pub fn get_terrain(self) -> Result<TerrainType, Tile> {
        match self.0 {
            Some(terrain) => Ok(terrain),
            _ => Err(self),
        }
    }
    pub fn get_mineral(self) -> Result<Mineral, Tile> {
        match self.1 {
            Some(mineral) => Ok(mineral),
            _ => Err(self),
        }
    }
    pub fn get_mineral_type(self) -> Result<MineralType, Tile> {
        match self.get_mineral() {
            a_mineral => Ok(a_mineral.ok().expect("Should be a mineral").0),
            _ => Err(self),
        }
    }
    pub fn get_buildable(self) -> Result<Buildable<RaceType>, Tile> {
        match self.2 {
            Some(buildable) => Ok(buildable),
            _ => Err(self),
        }
    }

    pub fn set_single(&mut self, tile_type: TileType) -> Tile {
        match tile_type {
            TileType::TerrainType(terrain_type) => {
                self.0 = Some(terrain_type);
            }
            TileType::Mineral(mineral_type) => {
                self.1 = Some(Mineral(mineral_type));
            }
            TileType::Building(buildable) => {
                self.2 = Some(buildable);
            }
        }
        *self
    }
    pub fn add(mut self, add: Tile) {
        match self {
            Tile(None, _, _) => self.0 = add.0,
            Tile(_, None, _) => self.1 = add.1,
            Tile(_, _, None) => self.2 = add.2,
            _ => {}
        };
    }

    /// Counts tiles around (x,y) that have the same type as tile_type
    pub fn count_neighbors(&mut self, map: Map, tile: Tile, x: usize, y: usize) -> usize {
        let mut count = 0;

        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                match map.get_tile(nx as usize, ny as usize) {
                    Ok((Tile(_, Some(mineral), _), _)) => {
                        if tile.has(mineral.0.to_tile_type()).is_ok() {
                            count += 1;
                        }
                    }
                    _ => {}
                }
            }
        }
        count
    }
}
