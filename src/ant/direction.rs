use std::{ fmt::Debug, ops::Add };

use rand::{ distributions::Standard, prelude::Distribution, Rng };

pub enum Direction {
    Up,
    Down,
    North,
    East,
    South,
    West,
}
impl Debug for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Up => write!(f, "Up"),
            Self::Down => write!(f, "Down"),
            Self::North => write!(f, "North"),
            Self::East => write!(f, "East"),
            Self::South => write!(f, "South"),
            Self::West => write!(f, "West"),
        }
    }
}
impl Into<(i32, i32, i32)> for Direction {
    fn into(self) -> (i32, i32, i32) {
        match self {
            Direction::West => (-1, 0, 0),
            Direction::East => (1, 0, 0),
            Direction::North => (0, -1, 0),
            Direction::South => (0, 1, 0),
            Direction::Up => (0, 0, 1),
            Direction::Down => (0, 0, -1),
        }
    }
}
impl Add for Direction {
    type Output = (i32, i32, i32);

    fn add(self, rhs: Self) -> Self::Output {
        let p: (i32, i32, i32) = self.into();
        let other: (i32, i32, i32) = rhs.into();

        (p.0 + other.0, p.1 + other.1, p.2 + other.2)
    }
}

impl Direction {
    pub fn add_to(&self, p: &(i32, i32, i32)) -> (i32, i32, i32) {
        let (mut x, mut y, mut z) = p;
        match self {
            Direction::West => {
                x -= 1;
            }
            Direction::East => {
                x += 1;
            }
            Direction::North => {
                y -= 1;
            }
            Direction::South => {
                y += 1;
            }
            Direction::Up => {
                z += 1;
            }
            Direction::Down => {
                z -= 1;
            }
        }
        (x, y, z)
    }
}

// Random implementation
// let direction: Direction = rand::random();
impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0..=5) {
            0 => { Direction::Up }
            1 => { Direction::Down }
            2 => { Direction::North }
            3 => { Direction::East }
            4 => { Direction::South }
            5 => { Direction::West }
            _ => panic!("What direction ? ???"),
        }
    }
}
