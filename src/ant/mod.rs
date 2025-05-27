mod test;

mod manager;

use std::time::Instant;

/// Name export so it's not confused with Chunk::Manager
pub use manager::Manager as AntManager;
use rand::{ distributions::{ Standard }, prelude::Distribution, Rng };

use crate::chunk::{manager::LoadedChunk, tile::Tile, CHUNK_WIDTH};
#[allow(unused)]
use crate::renderer::{ self, Renderer };

#[derive(Clone, Copy)]
pub struct Ant {
    pub last_action: Instant,
    pub pos: (i32, i32, i32),
    pub t: Type,
}

#[derive(Clone, Copy)]
pub enum Type {
    Explorer,
    Fetcher,
    Warrior,
}

pub enum Direction {
    Up,
    Down,
    North,
    East,
    South,
    West,
}
impl Direction {
    pub fn add_to(&self, p: (i32, i32, i32)) -> (i32, i32, i32) {
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
        match rng.gen_range(0..5) {
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

impl Ant {
    pub fn new(pos: (i32, i32, i32), t: Type) -> Self {
        Self { last_action: Instant::now(), pos, t }
    }
    pub fn think(&mut self) {
        let direction: Direction = rand::random();

        match direction {
            Direction::Up | Direction::Down => {}
            _ => self.walk(direction),
        }
        self.last_action = Instant::now();
    }
    pub fn walk(&mut self, d: Direction) {
        self.pos = d.add_to(self.pos);
    }
    pub fn act() {}
    pub fn find_tile() -> Option<Tile> {
        None
    }
}
impl Ant {
    // Checks if this xyz is in this chunk (un peu crado)
    pub fn is_in(&self, c:LoadedChunk) -> bool {
        let (x,y, width) = (c.pos.0, c.pos.1, CHUNK_WIDTH as i32);
        let (x_min, x_max) = (
            self.pos.0 / (width),
            (self.pos.0 + width) / (width) - 1,
        );
        let (y_min, y_max) = (
            self.pos.1 / (width),
            (self.pos.1 + width) / (width) - 1,
        );

        if x > x_min && x < x_max && y > y_min && y < y_max {
            true
        } else {
            false
        }
    }
}
