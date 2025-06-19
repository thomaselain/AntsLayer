use std::{ any::Any, ops::Deref, time::{ Duration, Instant } };

use crate::{
    ant::{ colony::Colony, worker::Worker, Action, ColonyMember, Direction },
    chunk::{ tile::TileFlag, ChunkManager, SEA_LEVEL },
};

pub struct Manager {
    pub colonies: [Colony; 2], // One colony for each player (0 is player and other are AI for now)
}

impl Manager {
    pub fn new() -> Self {
        Self {
            colonies: [Colony::Bel_o_Kan(), Colony::computer_colony()],
        }
    }
}

impl Manager {
    pub fn tick(&mut self, chunk_mngr: &ChunkManager, last_tick: Instant) {
        todo!("Ajouter colony.tick()");

        // self.colonies[Colony::PLAYER].tick();
        // self.colonies[Colony::AI].tick();

        // for c in self.colonies.as_mut_slice() {
        //     for a in c.ants {
        //         // Gravity check !
        //         if let Some(tile) = chunk_mngr.tile_at(Direction::Down.add_to(&a.pos)) {
        //             // Is is traversable ?
        //             if tile.properties.contains(TileFlag::TRAVERSABLE) && a.pos.2 > 0 {
        //                 a.pos = Direction::Down.add_to(&a.pos);
        //                 break;
        //             }
        //         }
        //         let mut action_attempts = 0;
        //         'walking: loop {
        //             action_attempts += 1;

        //             if Instant::now().duration_since(a.last_action) > Duration::from_millis(1000) {
        //                 if let Some(action) = a.think() {
        //                     match action {
        //                         Action::Walk(direction) => {
        //                             let dest_block = direction.add_to(&a.pos);
        //                             if let Some(tile) = chunk_mngr.tile_at(dest_block) {
        //                                 if tile.properties.contains(TileFlag::TRAVERSABLE) {
        //                                     a.walk(direction);
        //                                     break 'walking;
        //                                 } else {
        //                                     let dest_block = Direction::Up.add_to(
        //                                         &direction.add_to(&a.pos)
        //                                     );
        //                                     a.pos = dest_block;
        //                                     break 'walking;
        //                                 }
        //                             }
        //                         }
        //                     }
        //                 }
        //             } else if action_attempts > 4 {
        //                 break 'walking;
        //             }
        //         }
        //     }
        // }
    }
}
