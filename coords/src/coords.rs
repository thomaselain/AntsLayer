use std::ops::{ Add, AddAssign, Sub, SubAssign };
use crate::convert::{ Tof32, Toi32 };

/// `Coords` is a struct that holds two values, `x` and `y`, representing a 2D coordinate in a generic type `T`.
/// Methods are provided to convert `x` and `y` between `i32` and `f32` types.
///
/// # Example:
///
/// ```
/// use crate::coords::Coords;
/// let coords_f32 = Coords::new(1.5,2.5);
/// assert_eq!(coords_f32.x_i32(), 1); // Converts `x` to `i32`
/// assert_eq!(coords_f32.y_i32(), 2); // Converts `y` to `i32`
///
/// let coords_i32 = Coords::new(3,4);
/// assert_eq!(coords_i32.x_f32(), 3.0); // Converts `x` to `f32`
/// assert_eq!(coords_i32.y_f32(), 4.0); // Converts `y` to `f32`
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Hash)]
pub struct Coords<T> where T: Clone + Serialize {
    x: T,
    y: T,
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

    // Creates a new `Coords` instance with the given `x` and `y` values.
    pub fn new(x: T, y: T) -> Self {
        Coords { x, y }
    }
}

impl<T> Coords<T> where T: Clone + Serialize {
    /// Returns the `x` coordinate as an `i32` by converting the internal value.
    pub fn x_i32(&self) -> i32 where T: Toi32 {
        self.x.to_i32()
    }

    /// Returns the `y` coordinate as an `i32` by converting the internal value.
    pub fn y_i32(&self) -> i32 where T: Toi32 {
        self.y.to_i32()
    }

    /// Returns the `x` coordinate as an `f32` by converting the internal value.
    pub fn x_f32(&self) -> f32 where T: Tof32 {
        self.x.to_f32()
    }

    /// Returns the `y` coordinate as an `f32` by converting the internal value.
    pub fn y_f32(&self) -> f32 where T: Tof32 {
        self.y.to_f32()
    }
}

/// Implementation of relative movement for `Coords<T>`, where `T` supports
/// addition and subtraction.
impl<T> Coords<T> where T: AddAssign + SubAssign + Clone + Serialize {
    /// Moves the coordinates by the given `dx` and `dy`.
    pub fn move_by(&mut self, dx: T, dy: T) {
        self.x += dx;
        self.y += dy;
    }
}

// Operations: Subtraction, Addition, AddAssign, SubAssign
impl<T> Sub for Coords<T> where T: Sub<Output = T> + Clone + Serialize {
    type Output = Coords<T>;

    fn sub(self, other: Coords<T>) -> Coords<T> {
        Coords::new(self.x - other.x, self.y - other.y)
    }
}

impl<T> Add for Coords<T> where T: Add<Output = T> + Clone + Serialize {
    type Output = Coords<T>;

    fn add(self, other: Coords<T>) -> Coords<T> {
        Coords::new(self.x + other.x, self.y + other.y)
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
