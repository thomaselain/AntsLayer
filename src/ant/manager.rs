use std::time::{ Duration, Instant };

use crate::{ chunk::{ manager::LoadedChunk, SEA_LEVEL }, renderer::Renderer };

use super::Ant;

pub struct Manager {
    pub ants: Vec<Ant>,
}

impl Manager {
    pub fn new() -> Self {
        Self {
            ants: vec![
                // Joe
                Ant::new((0, 0, SEA_LEVEL as i32), super::Type::Explorer)
            ],
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
    pub fn tick(&mut self, loaded_chunks: &Vec<LoadedChunk>, last_tick: Instant) {
        for a in self.ants.as_mut_slice() {
            if Instant::now().duration_since(a.last_action) > Duration::from_secs(1) {
                #[cfg(test)]
                println!("Ant at {:?} is thinking", a.pos);
                a.think();
            }else{
            }
        }
    }
}
