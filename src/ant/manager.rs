use std::time::{ Duration, Instant };

use crate::{ ant::{ Action, Direction }, chunk::{ tile::TileFlag, ChunkManager, SEA_LEVEL } };

use super::Ant;

pub struct Manager {
    pub ants: Vec<Ant>,
}

impl Manager {
    pub fn new() -> Self {
        Self {
            ants: Self::generate_colony((0, 0, SEA_LEVEL as i32), 0),
        }
    }
    pub fn add(&mut self, ant: Ant) {
        self.ants.insert(0, ant);
    }

    pub fn generate_colony((x, y, z): (i32, i32, i32), n: usize) -> Vec<Ant> {
        let mut ants = vec![];

        for _i in 0..n {
            ants.push(Ant::new((x, y, (SEA_LEVEL as i32) + 15), super::Type::Explorer));
        }

        ants
    }
}

impl Manager {
    pub fn tick(&mut self, chunk_mngr: &ChunkManager, last_tick: Instant) {
        for a in self.ants.as_mut_slice() {
            // Gravity check !
            if let Some(tile) = chunk_mngr.tile_at(Direction::Down.add_to(&a.pos)) {
                // Is is traversable ?
                if tile.properties.contains(TileFlag::TRAVERSABLE) && a.pos.2 > 0 {
                    a.pos = Direction::Down.add_to(&a.pos);
                    break;
                }
            }
            let mut action_attempts = 0;
            'walking: loop {
                action_attempts += 1;

                if Instant::now().duration_since(a.last_action) > Duration::from_millis(1000) {
                    if let Some(action) = a.think() {
                        match action {
                            Action::Walk(direction) => {
                                let dest_block = direction.add_to(&a.pos);
                                if let Some(tile) = chunk_mngr.tile_at(dest_block) {
                                    if tile.properties.contains(TileFlag::TRAVERSABLE) {
                                        a.walk(direction);
                                        break 'walking;
                                    } else {
                                        let dest_block = Direction::Up.add_to(
                                            &direction.add_to(&a.pos)
                                        );
                                        a.pos = dest_block;
                                        break 'walking;
                                    }
                                }
                            }
                        }
                    }
                } else if action_attempts > 4 {
                    break 'walking;
                }
            }
        }
    }
}
