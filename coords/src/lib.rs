pub mod coords;
pub mod convert;
pub mod aliases;
pub mod serialize;

pub use crate::coords::Coords;

/// This module contains the `Coords` struct which represents a 2D coordinate system
/// and various operations that can be performed on it, such as addition, subtraction,
/// and coordinate manipulation.
///
/// ```
/// use crate::coords::Coords;
/// ```

/// # Example
/// ```
/// use crate::Coords;
/// let coord = Coords::new(1, 2);
/// assert_eq!(coord.x(), 1);
/// assert_eq!(coord.y(), 2);
/// ```
///
/// # Operations
/// You can perform addition, subtraction, and movement operations on `Coords`:
///
/// ## Addition
/// ```
/// let a = Coords::new(1, 2);
/// let b = Coords::new(3, 4);
/// let result = a + b;
/// assert_eq!(result, Coords::new(4, 6));
/// ```
///
/// ## Subtraction
/// ```
/// let a = Coords::new(3, 5);
/// let b = Coords::new(1, 2);
/// let result = a - b;
/// assert_eq!(result, Coords::new(2, 3));
/// ```
///
/// ## Movement
/// ```
/// let mut coord = Coords::new(1, 2);
/// coord.move_by(1, 1);
/// assert_eq!(coord.to_tuple(), (2, 3));
/// ```
#[cfg(test)]
mod tests {
    use super::*;

    /// Tests subtraction of two `Coords` with `f64` values.
    ///
    /// # Example
    /// ```
    /// let (a, b) = (Coords::new(2.0, 0.0), Coords::new(2.0, 0.0));
    /// assert_eq!(a - b, Coords::new(0.0, 0.0));
    /// ```
    #[test]
    fn sub_f64() {
        let a = Coords::new(2.0, 2.0, 2.0);
        assert_eq!(a - a, Coords::new(0.0, 0.0, 0.0));
    }

    /// Tests addition of two `Coords` with `f64` values.
    ///
    /// # Example
    /// ```
    /// let (a, b) = (Coords::new(2.0, 0.0), Coords::new(2.0, 0.0));
    /// assert_eq!(a + b, Coords::new(4.0, 0.0));
    /// ```
    #[test]
    fn add_f64() {
        let (a, b) = (Coords::new(2.0, 0.0, 1.0), Coords::new(2.0, 0.0, 1.0));
        assert_eq!(a + b, Coords::new(4.0, 0.0, 1.0));
    }

    /// Tests subtraction of two `Coords` with `i32` values.
    ///
    /// # Example
    /// ```
    /// let (a, b) = (Coords::new(2, 0), Coords::new(2, 0));
    /// assert_eq!(a - b, Coords::new(0, 0));
    /// ```
    #[test]
    fn sub_i32() {
        let a = Coords::new(2, 0, 1);
        assert_eq!(a - a, Coords::new(0, 0, 0));
    }

    /// Tests addition of two `Coords` with `i32` values.
    ///
    /// # Example
    /// ```
    /// let (a, b) = (Coords::new(2, 0), Coords::new(2, 0));
    /// assert_eq!(a + b, Coords::new(4, 0));
    /// ```
    #[test]
    fn add_i32() {
        let (a, b) = (Coords::new(2, 0, 1), Coords::new(2, 0, 2));
        assert_eq!(a + b, Coords::new(4, 0, 3));
    }
}
