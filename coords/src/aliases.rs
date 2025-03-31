use crate::Coords;

pub type TilePos = crate::Coords<i32>;
pub type ChunkPos = (i32,i32);

impl Default for TilePos {
    fn default() -> Self {
        Self::new(0, 0, 0)
    }
}

impl Into<ChunkPos> for TilePos{
    fn into(self) -> ChunkPos {
        (self.x(), self.y())
    }
}
impl Into<TilePos> for ChunkPos{
    fn into(self) -> TilePos {
        Coords::new(self.0, self.1,0)
    }
}