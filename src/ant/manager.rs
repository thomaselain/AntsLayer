use std::time::Instant;

use crate::{ chunk::manager::LoadedChunk, renderer::Renderer };

use super::Ant;

pub struct Manager {
    pub ants: Vec<Ant>,
}

impl Manager {
    pub fn new() -> Self {
        Self { ants: vec![] }
    }
    pub fn add(&mut self, ant: Ant) {
        self.ants.insert(0, ant);
    }
}

impl Manager {
    pub fn tick(
        &mut self,
        renderer: &mut Renderer,
        loaded_chunks: &Vec<LoadedChunk>,
        _time: Instant
    ) {
        for a in self.ants.as_slice() {
            if a.pos.2 != renderer.camera.2 {
                a.render(renderer);
                // continue;
            }
            //  else if let Some(tile) = loaded_chunks.find_tile(a.pos) {
            //     a.render(renderer);
            // }

            a.think();
        }
    }
}
