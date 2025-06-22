use crate::{ ant::colony::Colony, chunk::ChunkManager, renderer::Renderer };

pub struct Manager {
    pub colonies: [Colony; 2], // One colony for each player (0 is player and other are AI for now)
}

impl Manager {
    pub fn new() -> Self {
        Self {
            colonies: [Colony::bel_o_kan(), Colony::computer_colony()],
        }
    }
}

impl Manager {
    pub fn tick(&mut self, chunk_mngr: &ChunkManager) {
        self.colonies[Colony::PLAYER].tick(chunk_mngr);
        self.colonies[Colony::AI].tick(chunk_mngr);
    }
    pub fn render(&self, renderer: &mut Renderer, timestamp:f64) {
        renderer.draw_ants(&self.colonies[Colony::PLAYER], timestamp);
        renderer.draw_ants(&self.colonies[Colony::AI], timestamp);
    }
}
