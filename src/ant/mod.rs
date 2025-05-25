mod test;

mod manager;

use std::time::Instant;

/// Name export so it's not confused with Chunk::Manager
pub use manager::Manager as AntManager;

use crate::chunk::tile::Tile;
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

impl Ant {
    pub fn new(pos: (i32, i32, i32), t: Type) -> Self {
        Self { last_action: Instant::now(), pos, t }
    }
    pub fn think(&mut self) {
        self.pos.0 += 1;
     
        self.last_action = Instant::now();
    }
    pub fn act() {}
    pub fn find_tile() -> Option<Tile> {
        None
    }
}
