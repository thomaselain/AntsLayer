use std::time::{ Duration, Instant };

use crate::{ ant::Direction, chunk::{ manager::LoadedChunk, tile::TileFlag, ChunkManager } };

use super::Ant;

pub struct Manager {
    pub ants: Vec<Ant>,
}

impl Manager {
    pub fn new() -> Self {
        Self {
            ants: vec![],
        }
    }
    pub fn add(&mut self, ant: Ant) {
        self.ants.insert(0, ant);
    }
    pub fn find_from_chunk(ants: &Vec<Ant>, chunk: &LoadedChunk) -> Vec<Ant> {
        let mut v = vec![];

        for a in ants {
            if a.is_in(*chunk) {
                v.push(*a);
            }
        }

        v
    }
}

impl Manager {
    pub fn tick(&mut self, chunk_mngr: &ChunkManager, last_tick: Instant) {
        for a in self.ants.as_mut_slice() {
            if Instant::now().duration_since(a.last_action) > Duration::from_secs(1) {
                // Gravity check !
                if let Some(tile) = chunk_mngr.tile_at(Direction::Down.add_to(a.pos)) {
                    // Is is traversable ?
                    if tile.properties.contains(TileFlag::TRAVERSABLE) && a.pos.2 > 0 {
                        a.pos = Direction::Down.add_to(a.pos);
                    }
                }

                #[cfg(test)]
                println!("Ant at {:?} is thinking", a.pos);
                a.think();
            } else {
            }
        }
    }
}
