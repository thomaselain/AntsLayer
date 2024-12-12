/// This module provides conversion traits and methods for coordinates.
use crate::Coords;

/// Trait to convert a type to `i32`.
pub trait Toi32 {
    fn to_i32(&self) -> i32;
}

impl Toi32 for f32 {
    fn to_i32(&self) -> i32 {
        *self as i32
    }
}

/// Trait to convert a type to `f32`.
pub trait Tof32 {
    fn to_f32(&self) -> f32;
}

impl Tof32 for i32 {
    fn to_f32(&self) -> f32 {
        *self as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Coords;


    #[test]
    fn test_coords_conversion() {
        let coords_f32 = Coords:: new(3.0, 4.0);
        let coords_i32 = Coords:: new(3, 4);

        assert_eq!(coords_f32.x_i32(), 3); // Convert x from f32 to i32
        assert_eq!(coords_f32.y_i32(), 4); // Convert y from f32 to i32

        assert_eq!(coords_i32.x_f32(), 3.0); // Convert x from i32 to f32
        assert_eq!(coords_i32.y_f32(), 4.0); // Convert y from i32 to f32
    }
}
