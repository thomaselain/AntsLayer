use coords::Coords;

use crate::map::{buildings, minerals::MineralType, terrain::TileType, Map};

use super::{ActionQueue, ActionType, Unit};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum JobType {
    MINER(TileType),
    JOBLESS,
    FARMER,
    FIGHTER,
    BUILDER,
}

impl JobType {
    /// Specific for Miners
    /// if self.job isn't some MINER(MineralType), will return Error
    /// else returns Ok(MineralType)
    pub fn get_miner_target(self) -> Result<MineralType, ()> {
        match self {
            JobType::MINER(tile_type) => match tile_type {
                TileType::Mineral(mineral_type) => Ok(mineral_type),
                _ => Err(()),
            },
            _ => Err(()),
        }
    }
}

impl Unit {
    pub fn find_job_action(&self, map: &Map) -> Result<(ActionType, Coords), Coords> {
        match self.job.clone() {
            JobType::MINER(tile_type) => {
                if let Ok(target) = map.find_closest(self.coords, tile_type) {
                    return Ok((ActionType::DIG, target));
                }
            }
            JobType::FARMER => {}
            JobType::FIGHTER => {}
            JobType::BUILDER => {}
            JobType::JOBLESS => {}
        }
        map.clone().go_to_hearth(self.coords)
    }
}

impl Map {
    pub fn go_to_hearth(self, from: Coords) -> Result<(ActionType, Coords), Coords> {
        if let Ok(hearth) = self.find_closest_building(from, buildings::BuildingType::Hearth) {
            Ok((ActionType::MOVE, hearth))
        } else {
            Err(from)
        }
    }
}
