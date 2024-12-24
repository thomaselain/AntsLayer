pub type TilePos = crate::Coords<i32>;

impl Default for TilePos {
    fn default() -> Self {
        Self::new(0, 0)
    }
}