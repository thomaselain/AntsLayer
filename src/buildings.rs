use crate::{coords::Coords, terrain::Terrain, units::RaceType};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BuildingType {
    Hearth,
    Stockpile,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Building {
    pub hp: u32,
    pub coords: Coords,
    pub building_type: BuildingType,
    pub race: RaceType,
}

pub trait FindHome {
    fn find_home(&self, race: RaceType, terrain: &Terrain) -> Option<Coords>;
}

impl FindHome for Vec<Building> {
    fn find_home(&self, race: RaceType, terrain: &Terrain) -> Option<Coords> {
    //    if let Some(tb) = terrain.buildings.iter().find(|b| b.race == race) { // ALL GO TO ANTS HEARTS FOR TESTINGS
        if let Some(tb) = terrain.buildings.iter().find(|b| b.race == RaceType::ANT) {
            return Some(tb.coords);
        } else {
            return None;
        }
    }
}

impl Building {}
