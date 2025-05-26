use std::time::{ Duration, Instant };

use crate::{ ant::Direction, chunk::{ ChunkManager, SEA_LEVEL }, renderer::Renderer };

use super::Ant;

pub struct Manager {
    pub ants: Vec<Ant>,
}

impl Manager {
    pub fn new() -> Self {
        Self {
            ants:vec![],
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
