pub const HOME_STARTING_SIZE: u32 = 20;

use crate::{
    coords::Coords,
    terrain::{Mineral, MineralType, Terrain, TileType, HEIGHT, WIDTH},
    units::{JobType, RaceType},
};
use rand::{self, Rng};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BuildingType {
    Hearth,
    Stockpile(MineralType),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Building {
    pub hp: u32,
    pub coords: Coords,
    pub building_type: BuildingType,
    pub race: RaceType,
}

pub trait FindHome {
    fn find_building(&self, race_type: RaceType, job: JobType, terrain: &Terrain)
        -> Option<Coords>;
}
impl Building {
    pub fn new(race_type: RaceType, building_type: BuildingType) -> Building {
        let mut rng = rand::thread_rng();
        let offset: i32 = HOME_STARTING_SIZE as i32 / 2;
        let center_coords = match race_type {
            RaceType::ANT => Coords {
                x: offset,
                y: offset,
            },
            RaceType::HUMAN => Coords {
                x: WIDTH as i32 / 2,
                y: HEIGHT as i32 / 2,
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
impl FindHome for Vec<Building> {
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

impl Building {}
