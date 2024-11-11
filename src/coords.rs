use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Coords {
    pub x: i32,
    pub y: i32,
}

/// Coords struct for easier coordinates manipulation
impl Coords {
    pub fn to_tuple(&self) -> (usize, usize) {
        (self.x as usize, self.y as usize)
    }
    pub fn _distance_to(&self, other: &Coords) -> f64 {
        let dx = (self.x - other.x) as f64;
        let dy = (self.y - other.y) as f64;
        (dx.powi(2) + dy.powi(2)).sqrt()
    }

    pub fn distance_in_tiles(&self, other: &Coords) -> i32 {
        (self.x - other.x).abs().max((self.y - other.y).abs())
    }
}

/// Coords += OtherCoords
impl AddAssign for Coords {
    fn add_assign(&mut self, other: Coords) {
        self.x += other.x;
        self.y += other.y;
    }
}

/// Coords -= OtherCoords
impl SubAssign for Coords {
    fn sub_assign(&mut self, other: Coords) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

/// Coords + OtherCoords
impl Add for Coords {
    fn add(self, other: Coords) -> Coords {
        Coords {
            x: other.x + self.x,
            y: other.y + self.y,
        }
    }
    type Output = Coords;
}

/// Coords - OtherCoords
impl Sub for Coords {
    fn sub(self, other: Coords) -> Coords {
        Coords {
            x: other.x - self.x,
            y: other.y - self.y,
        }
    }
    type Output = Coords;
}
