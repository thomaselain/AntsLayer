use std::ops::{AddAssign, SubAssign};

#[derive(Copy, Clone, Debug)]
pub struct Coords {
    pub x: i32,
    pub y: i32,
}

impl AddAssign for Coords {
    fn add_assign(&mut self, other: Coords) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl SubAssign for Coords {
    fn sub_assign(&mut self, other: Coords) {
        self.x -= other.x;
        self.y -= other.y;
    }
}


