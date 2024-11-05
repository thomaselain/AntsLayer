use crate::{coords::Coords, terrain::Terrain, units::RaceType};

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum BuildingType {
    Hearth,
    Stockpile,
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Building {
    pub hp: u32,
    pub coords: Coords,
    pub building_type: BuildingType,
    pub race: RaceType,
}

pub trait FindHome {
    fn find_home(&self, terrain: &Terrain) -> Option<Coords>;
}

impl FindHome for Vec<(RaceType, Building)> {
    fn find_home(&self, terrain: &Terrain) -> Option<Coords> {
        for (unit_race, building) in self {
            if let Some(tb) = terrain.buildings.iter().find(|(_, b)| b.race == *unit_race) {
               // println!("Home found at ({}, {})", tb.1.coords.x, tb.1.coords.y);
                return Some(tb.1.coords);
            }
        }
        None
    }
}

impl Building {}
