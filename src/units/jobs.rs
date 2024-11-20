use crate::map::terrain::TileType;

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
    /// if self.job isn't some MINER(MineralType), will return None
    /// else returns Some(MineralType)
    pub fn get_miner_target(self) -> TileType {
        match self {
            JobType::MINER(tile_type) => tile_type,
            _ => panic!(),
        }
    }
}
