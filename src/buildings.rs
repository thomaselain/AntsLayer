use std::any::Any;

use noise::Min;
use rand::{self, distributions::uniform::SampleRange, Rng};

use crate::{
    coords::Coords,
    minerals::MineralType,
    terrain::{Terrain, TileType, HEIGHT, WIDTH},
    units::{JobType, RaceType},
};
/// At terrain generation, a circle is dug around its race's Hearth
pub const HOME_STARTING_SIZE: u32 = 20;

/// (stored_amount, max)
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
pub struct Stockpile<MineralType>(pub MineralType, pub StockpileContent);
impl Stockpile<MineralType> {
    pub fn new(self) -> Stockpile<MineralType> {
        Stockpile(self.0, self.1)
    }
    pub fn get_mineral_type(&self) -> MineralType {
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
        match self {
            BuildingType::Hearth => 0x999944ff,
            BuildingType::Stockpile(s) => 0x064f28ff,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Building(pub BuildingType, pub RaceType);

impl Building {
}

// pub trait FindBuilding {
//     fn find_building(&self, race_type: RaceType, job: JobType, terrain: &Terrain)
//         -> Option<Coords>;
// }

//   /// impl Vec<Building> to use on terrain.buildings
//   impl FindBuilding for Vec<Building> {
//       /// Return Some(coords) if a matching Building was found
//       /// Return none otherwise
//       fn find_building(
//           &self,
//           race_type: RaceType,
//           job: JobType,
//           terrain: &Terrain,
//       ) -> Option<Coords> {
//           //   if let Some(tb) = terrain.buildings.iter().find(|b| b.race == RaceType::ANT) {// ALL GO TO ANTS HEARTS FOR TESTINGS
//           if let Some(tb) = terrain.buildings.iter().find(|b| {
//               b.1 == race_type
//                   && match job {
//                       JobType::MINER(mineral_type) => {
//                           TileType::Building(BuildingType::Stockpile(mineral_type))
//                       }
//                       _ => TileType::Building(BuildingType::Hearth),
//                   } == TileType::Building(b.0)
//           }) {
//               return Some(tb);
//           } else {
//               return None;
//           }
//       }
//   }
//}
