use rand::{self, Rng};

use crate::{
    coords::Coords,
    terrain::{MineralType, Terrain, HEIGHT, WIDTH},
    units::{JobType, RaceType},
};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BuildingType {
    Hearth,
    /// A stockpile is used by Units that have a matching job(MineralType)
    Stockpile(MineralType),
}

/// At terrain generation, a circle is dug around its race's Hearth
pub const HOME_STARTING_SIZE: u32 = 20;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Building {
    /// 0 < hp < 100
    pub hp: u32,
    pub coords: Coords,
    pub building_type: BuildingType,
    pub race: RaceType,
}
impl Building {
    pub fn new(race_type: RaceType, building_type: BuildingType) -> Building {
        let mut rng = rand::thread_rng();
        let offset: i32 = HOME_STARTING_SIZE as i32 / 2;
        let center_coords = match race_type {
            RaceType::ANT => Coords {
                x: WIDTH as i32 / 2,
                y: HEIGHT as i32 / 2,
            },
            RaceType::HUMAN => Coords {
                x: offset,
                y: offset,
            },
            RaceType::ALIEN => Coords {
                x: WIDTH as i32 - offset,
                y: HEIGHT as i32 - offset,
            },
        };
        Building {
            hp: 100,
            coords: match building_type {
                BuildingType::Stockpile(_) => {
                    center_coords
                        + Coords {
                            x: rng.gen_range(-offset..offset),
                            y: rng.gen_range(-offset..offset),
                        }
                }
                BuildingType::Hearth => center_coords,
            },
            building_type: building_type,
            race: race_type,
        }
    }
}


pub trait FindBuilding {
    fn find_building(&self, race_type: RaceType, job: JobType, terrain: &Terrain)
        -> Option<Coords>;
}

/// impl Vec<Building> to use on terrain.buildings
impl FindBuilding for Vec<Building> {
    /// Return Some(coords) if a matching Building was found
    /// Return none otherwise
    fn find_building(
        &self,
        race_type: RaceType,
        job: JobType,
        terrain: &Terrain,
    ) -> Option<Coords> {
        //   if let Some(tb) = terrain.buildings.iter().find(|b| b.race == RaceType::ANT) {// ALL GO TO ANTS HEARTS FOR TESTINGS
        if let Some(tb) = terrain.buildings.iter().find(|b| {
            b.race == race_type
                && match job {
                    JobType::MINER(mineral) => BuildingType::Stockpile(mineral),
                    _ => BuildingType::Hearth,
                } == b.building_type
        }) {
            return Some(tb.coords);
        } else {
            return None;
        }
    }
}
