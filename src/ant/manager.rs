use std::time::{ Duration, Instant };

use crate::{
    ant::Direction,
    chunk::{ manager::LoadedChunk, tile::TileType, ChunkManager, SEA_LEVEL },
    renderer::Renderer,
};

use super::Ant;

pub struct Manager {
    pub ants: Vec<Ant>,
}

impl Manager {
    pub fn new() -> Self {
        let mut ants = vec![Ant::new((0, 0, SEA_LEVEL as i32), super::Type::Explorer)];

        for i in 10..20 {
            ants.push(Ant::new((i, i, SEA_LEVEL as i32), super::Type::Fetcher));
        }

        Self {
            ants,
        }
    }
    pub fn add(&mut self, ant: Ant) {
        self.ants.insert(0, ant);
    }
    pub fn render(&mut self, renderer: &mut Renderer) {
        for a in self.ants.as_mut_slice() {
            // Only display ants in curent camera height
            if a.pos.2 == renderer.camera.2 {
                a.render(renderer);
            } else {
            }
        }
    }
}

impl Manager {
    pub fn tick(&mut self, chunk_mngr: &ChunkManager, last_tick: Instant) {
        for a in self.ants.as_mut_slice() {
            if Instant::now().duration_since(a.last_action) > Duration::from_secs(1) {
                // Gravity check !
                if let Some(tile) = chunk_mngr.tile_at(Direction::Down.add_to(a.pos)) {
                }

                #[cfg(test)]
                println!("Ant at {:?} is thinking", a.pos);
                a.think();
            } else {
            }
        }
    }
}
