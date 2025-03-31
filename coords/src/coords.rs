use std::ops::{ Add, AddAssign, Sub, SubAssign };
use crate::convert::{ Tof32, Tof64, Toi32 };

/// `Coords` is a struct that holds two values, `x` and `y`, representing a 2D coordinate in a generic type `T`.
/// Methods are provided to convert `x` and `y` between `i32` and `f32` types.
///
/// # Example:
///
/// ```
/// use crate::coords::Coords;
/// let coords_f32 = Coords::new(1.5,2.5,5.5);
/// assert_eq!(coords_f32.x_i32(), 1); // Converts `x` to `i32`
/// assert_eq!(coords_f32.y_i32(), 2); // Converts `y` to `i32`
/// assert_eq!(coords_f32.z_i32(), 5); // Converts `z` to `i32`
///
/// let coords_i32 = Coords::new(3,4,5);
/// assert_eq!(coords_i32.x_f32(), 3.0); // Converts `x` to `f32`
/// assert_eq!(coords_i32.y_f32(), 4.0); // Converts `y` to `f32`
/// assert_eq!(coords_i32.z_f32(), 5.0); // Converts `z` to `f32`
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Hash)]
pub struct Coords<T> where T: Clone + Serialize {
    x: T,
    y: T,
    z: T,
}

impl<T> Coords<T> where T: Clone + Serialize {
    // Simple accessor for x
    pub fn x(self) -> T {
        self.x
    }

    // Simple accessor for y
    pub fn y(self) -> T {
        self.y
    }
    // Simple accessor for z
    pub fn z(self) -> T {
        self.z
    }

    // Creates a new `Coords` instance with the given `x` and `y` values.
    pub fn new(x: T, y: T, z: T) -> Self {
        Coords { x, y, z }
    }
}

impl<T> Coords<T> where T: Clone + Serialize {
    /// Returns the `x` coordinate as an `i32`
    pub fn x_i32(&self) -> i32 where T: Toi32 {
        self.x.to_i32()
    }

    /// Returns the `y` coordinate as an `i32`
    pub fn y_i32(&self) -> i32 where T: Toi32 {
        self.y.to_i32()
    }
    /// Returns the `z` coordinate as an `i32`
    pub fn z_i32(&self) -> i32 where T: Toi32 {
        self.z.to_i32()
    }

    /// Returns the `x` coordinate as an `f32`
    pub fn x_f32(&self) -> f32 where T: Tof32 {
        self.x.to_f32()
    }

    /// Returns the `y` coordinate as an `f32`
    pub fn y_f32(&self) -> f32 where T: Tof32 {
        self.y.to_f32()
    }
    /// Returns the `z` coordinate as an `f32`
    pub fn z_f32(&self) -> f32 where T: Tof32 {
        self.z.to_f32()
    }

    /// Returns the `x` coordinate as an `f32`
    pub fn x_f64(&self) -> f64 where T: Tof64 {
        self.x.to_f64()
    }

    /// Returns the `y` coordinate as an `f64`
    pub fn y_f64(&self) -> f64 where T: Tof64 {
        self.y.to_f64()
    }
    /// Returns the `z` coordinate as an `f64`
    pub fn z_f64(&self) -> f64 where T: Tof64 {
        self.z.to_f64()
    }
}

/// Implementation of relative movement for `Coords<T>`, where `T` supports
/// addition and subtraction.
impl<T> Coords<T> where T: AddAssign + SubAssign + Clone + Serialize {
    /// Moves the coordinates by the given `dx` and `dy`.
    pub fn move_by(&mut self, dx: T, dy: T, dz: T) {
        self.x += dx;
        self.y += dy;
        self.z += dz;
    }
}

// Operations: Subtraction, Addition, AddAssign, SubAssign
impl<T> Sub for Coords<T> where T: Sub<Output = T> + Clone + Serialize {
    type Output = Coords<T>;

    fn sub(self, other: Coords<T>) -> Coords<T> {
        Coords::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl<T> Add for Coords<T> where T: Add<Output = T> + Clone + Serialize {
    type Output = Coords<T>;

    fn add(self, other: Coords<T>) -> Coords<T> {
        Coords::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl<T> AddAssign for Coords<T> where T: Add<Output = T> + Clone + Serialize + AddAssign {
    fn add_assign(&mut self, other: Coords<T>) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<T> SubAssign for Coords<T> where T: Add<Output = T> + Clone + Serialize + SubAssign {
    fn sub_assign(&mut self, other: Coords<T>) {
        self.x -= other.x;
        self.y -= other.y;
    }
}
