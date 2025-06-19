use rand::{distributions::Standard, prelude::Distribution, Rng};

pub enum Direction {
    Up,
    Down,
    North,
    East,
    South,
    West,
}
impl Direction {
    pub fn add_to(&self, p: &(i32, i32, i32)) -> (i32, i32, i32) {
        let (mut x, mut y, mut z) = p;
        match self {
            Direction::West => {
                x += 1;
            }
            Direction::East => {
                x -= 1;
            }
            Direction::North => {
                y += 1;
            }
            Direction::South => {
                y -= 1;
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
