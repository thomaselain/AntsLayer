use coords::Coords;

use crate::units::{RaceType, Unit, HOME_STARTING_SIZE};

use super::{minerals::MineralType, Map, Tile, TileType};

impl Map {
    pub fn build_starting_zone(
        &mut self,
        race_type: RaceType,
    ) -> Result<Buildable<RaceType>, Coords> {
        self.dig_radius(&race_type.starting_coords(), HOME_STARTING_SIZE)?;

        let new_tile = self.set_tile(
            race_type.starting_coords(),
            Some(Tile(
                Some(crate::map::terrain::TerrainType::AIR),
                None,
                Some(crate::map::buildings::Buildable::Hearth(Hearth {
                    race_type,
                    hp: 100,
                })),
            )),
        )?;

        let stockpile: Building<Buildable<RaceType>> = Building {
            buildable: Buildable::Stockpile(Stockpile {
                mineral_type: MineralType::MOSS,
                content: Content(0, 0),
            }),
            coords: race_type.starting_coords() + Coords(0, 5),
            race_type,
        };

        self.build(stockpile)?;
        let stockpile: Building<Buildable<RaceType>> = Building {
            buildable: Buildable::Stockpile(Stockpile {
                mineral_type: MineralType::ROCK,
                content: Content(0, 0),
            }),
            coords: race_type.starting_coords() + Coords(5, 0),
            race_type,
        };
        self.build(stockpile)?;
        let stockpile: Building<Buildable<RaceType>> = Building {
            buildable: Buildable::Stockpile(Stockpile {
                mineral_type: MineralType::DIRT,
                content: Content(0, 0),
            }),
            coords: race_type.starting_coords() + Coords(0, -5),
            race_type,
        };
        self.build(stockpile)?;
        let stockpile: Building<Buildable<RaceType>> = Building {
            buildable: Buildable::Stockpile(Stockpile {
                mineral_type: MineralType::GOLD,
                content: Content(0, 0),
            }),
            coords: race_type.starting_coords() + Coords(-5, 0),
            race_type,
        };
        self.build(stockpile)?;

        Ok(new_tile.get_buildable().ok().expect("...?"))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Buildable<R>
where
    R: Copy,
    R: Eq,
{
    Hearth(Hearth<R>),
    Stockpile(Stockpile<MineralType>),
}

impl Buildable<RaceType> {
    pub fn to_tile_type(self) -> TileType {
        TileType::Building(self)
    }
    pub fn color(self) -> u32 {
        match self {
            Buildable::Hearth(_) => 0xccaa44ff,
            Buildable::Stockpile(_) => 0x064f28ff,
        }
    }
    pub fn building_type(self) -> BuildingType {
        match self {
            Buildable::Hearth(_) => BuildingType::Hearth,
            Buildable::Stockpile(s) => BuildingType::Stockpile(s.mineral_type),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Building<Buildable> {
    pub buildable: Buildable,
    pub coords: Coords,
    pub race_type: RaceType,
}
impl Building<Buildable<RaceType>> {
    pub fn stockpile(&self) -> Stockpile<MineralType> {
        match self.buildable {
            Buildable::Stockpile(s) => s,
            _ => panic!("No stockpile in this Building"),
        }
    }
    pub fn to_tile_type(self) -> TileType {
        TileType::Building(self.buildable)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Stockpile<M>
where
    M: Copy,
{
    pub mineral_type: M,
    pub content: Content,
}
impl Stockpile<MineralType> {
    fn eq(self, other: Stockpile<MineralType>) -> bool {
        self.mineral_type == other.mineral_type
    }
}

/// (stored_amount, max)
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct StockpileContent(u32, u32);

impl StockpileContent {
    pub fn add(mut self, amount: u32) {
        self.0 += amount;
    }
    pub fn take(mut self, amount: u32) {
        self.0 -= amount;
    }
    pub fn stored_amount(&self) -> u32 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BuildingType {
    Hearth,
    /// A stockpile is used by Units that have a matching job(MineralType)
    Stockpile(MineralType),
}
impl BuildingType {
    pub fn color(self) -> u32 {
        0xff0ff00;
        match self {
            BuildingType::Hearth => 0xccaa44ff,
            BuildingType::Stockpile(mineral_type) => match mineral_type {
                MineralType::MOSS => 0x064f28ff,
                MineralType::DIRT => 0x000030ff,
                MineralType::ROCK => 0x0a0a0aff,
                MineralType::GOLD => 0x505030ff,
                _ => 0xff00ffff,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
/// (stored_amount, max)
pub struct Content(pub u32, pub u32);

impl Content {
    pub fn new() -> Content {
        Content(0, 0)
    }
    pub fn add(mut self, amount: u32) {
        self.0 += amount;
    }
    pub fn take(mut self, amount: u32) {
        self.0 -= amount;
    }
    pub fn stored_amount(&self) -> u32 {
        self.0
    }
    pub fn max_amount(&self) -> u32 {
        self.1
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Hearth<R>
where
    R: Copy,
{
    pub race_type: R,
    pub hp: u32,
}

impl Hearth<RaceType> {
    fn eq(self, other: Hearth<RaceType>) -> bool {
        self.race_type == other.race_type
    }
}

impl Unit {
    pub fn find_closest_building(self, map: Map, building_type: BuildingType) -> Result<Building<Buildable<RaceType>>, BuildingType> {
        Err(building_type)
    }
}
