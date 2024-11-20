use crate::Coords;

use std::ops::{Add, AddAssign, Sub, SubAssign};

/// A Coords struct for easier coordinates manipulation
impl Coords {
    /// Accessor for x
    pub fn x(&self) -> i32 {
        self.0
    }

    /// Accessor for y
    pub fn y(&self) -> i32 {
        self.1
    }

    /// * Tuple conversion
    /// # Example
    /// ```rust, no_run
    /// # use coords::Coords;
    /// assert_eq!((0, 10), Coords(0,10).to_tuple());
    /// ```
    pub fn to_tuple(&self) -> (usize, usize) {
        (self.x() as usize, self.y() as usize)
    }

    /// # Returns distance to other
    /// ```rust, no_run
    /// # use coords::Coords;
    /// let c_1 = Coords(0,0);
    /// let c_2 = Coords(0,10);
    /// assert_eq!(10.0, c_1.distance_to(&c_2));
    /// ```
    pub fn distance_to(&self, other: &Coords) -> f64 {
        let dx = (self.x() - other.x()) as f64;
        let dy = (self.y() - other.y()) as f64;
        (dx.powi(2) + dy.powi(2)).sqrt()
    }

    /// # Return distance to other but in Manathan coordinates
    /// ```rust
    /// # use coords::Coords;
    /// let c_1 = Coords(0,0);
    /// let c_2 = Coords(0,10);
    /// assert_eq!(10, c_1.distance_in_tiles(&c_2));
    /// ```
    pub fn distance_in_tiles(&self, other: &Coords) -> i32 {
        (self.x() - other.x())
            .abs()
            .max((self.y() - other.y()).abs())
    }
    pub fn swap_coords(&self) -> Coords{
        Coords(self.y(), self.x())
    }
}

/// Coords += OtherCoords
impl AddAssign for Coords {
    fn add_assign(&mut self, other: Coords) {
        self.0 += other.x();
        self.1 += other.y();
    }
}

/// Coords -= OtherCoords
impl SubAssign for Coords {
    fn sub_assign(&mut self, other: Coords) {
        self.0 -= other.x();
        self.1 -= other.y();
    }
}

/// Coords + OtherCoords
impl Add for Coords {
    fn add(self, other: Coords) -> Coords {
        Coords(other.x() + self.0, other.y() + self.1)
    }
    type Output = Coords;
}

/// Coords - OtherCoords
impl Sub for Coords {
    fn sub(self, other: Coords) -> Coords {
        Coords(other.x() - self.0, other.y() - self.1)
    }
    type Output = Coords;
}
