use super::TileType;

extern crate noise;
extern crate sdl2;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum TerrainType {
    AIR,
    WATER,
}

impl TerrainType {
    pub fn to_tile_type(self) -> TileType {
        TileType::TerrainType(self)
    }
    pub fn color(self) -> u32 {
        match self {
            TerrainType::AIR => 0x040201ff,
            TerrainType::WATER => 0x111199ff,
        }
    }
}
