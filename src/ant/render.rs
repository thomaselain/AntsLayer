use crate::{
    ant::{ colony::Colony, ColonyMember },
    chunk::tile::{ Tile, TileFlag },
    renderer::Renderer,
};

impl Renderer<'_> {
    pub fn draw_ants(&mut self, colony: &Colony, timestamp: f64) {
        colony.queen.clone().render(self);

        for ant in colony.ants.iter() {
            ant.render(self);
        }
    }
}
