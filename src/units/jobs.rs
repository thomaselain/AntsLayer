use crate::game::map::tile::minerals::MineralType;

use super::TileType;


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
