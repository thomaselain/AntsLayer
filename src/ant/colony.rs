use std::time::{ Duration, Instant };

use crate::{
    ant::{
        explorer::Explorer,
        apply_gravity,
        queen::{ Egg, Queen },
        Action,
        ColonyMember,
    },
    chunk::{ ChunkManager, SEA_LEVEL },
};

pub struct Colony {
    pub name: &'static str,
    pub queen: Queen,
    pub ants: Vec<Box<dyn ColonyMember>>,
}
impl Colony {
    const TEST_NAME: &'static str = "Bel-o-Kan";
    const AI_NAME: &'static str = "Com-p-uter";

    pub const PLAYER: usize = 0;
    pub const AI: usize = 1;

    pub fn tick(&mut self, chunk_mngr: &ChunkManager) {
        //////////  QUEEN  /////////

        if Instant::now().duration_since(self.queen.last_action()) > Duration::from_millis(1000) {
            if let Some(action) = self.queen.think() {
                match action {
                    Action::Walk(_) => {
                        panic!("Why would the queen go anywhere ?");
                    }
                    Action::Breed(mut newborns) => {
                        self.ants.append(&mut newborns);
                        self.queen.reset_last_action();
                    }
                }
            }
            // Gravity check !
            let new_pos = apply_gravity(&self.queen.pos, chunk_mngr);
            self.queen.set_pos(new_pos);
        }

        //////////  ANTS  /////////
        for ant in self.ants.iter_mut() {
            if Instant::now().duration_since(ant.last_action()) > Duration::from_millis(1000) {
                println!(
                    "ant took {:.2?} to do something",
                    Instant::now().duration_since(ant.last_action())
                );
                if let Some(action) = ant.think() {
                    match action {
                        Action::Walk(direction) => {
                            ant.walk(&chunk_mngr, direction);
                            ant.reset_last_action();
                        }
                        Action::Breed(_) => {
                            panic!("Only the queen must breed ! ");
                        }
                    }
                }
            }
            // Gravity check !
            let new_pos = apply_gravity(&ant.pos(), chunk_mngr);
            ant.set_pos(new_pos);
        }
    }
    pub fn describe(&self) -> String {
        format!(
            "New colony at : {:?}, it is named {:?} and has a population of {:?} ants\nIts queen has {} eggs ready to hatch",
            self.queen.pos,
            self.name,
            self.ants.len(),
            self.queen.eggs.len()
        )
    }
    // Colony::PLAYER
    pub fn bel_o_kan() -> Self {
        let mut bok = Self::new(Self::TEST_NAME, (0, 0, (SEA_LEVEL as i32) + 15));

        let ants = vec![];

        bok.ants = ants;

        bok.queen.eggs.push(Egg { hatch: Explorer::new });
        // bok.queen.eggs.push(Egg { hatch: Explorer::new });
        // bok.queen.eggs.push(Egg { hatch: Explorer::new });
        // bok.queen.eggs.push(Egg { hatch: Worker::new });
        // bok.queen.eggs.push(Egg { hatch: Worker::new });
        // bok.queen.eggs.push(Egg { hatch: Worker::new });

        println!("{}", bok.describe());

        bok
    }

    // Colony::AI
    pub fn computer_colony() -> Self {
        let mut ai = Self::new(Self::AI_NAME, (-5, 0, (SEA_LEVEL as i32) - 15));

        let ants = vec![];

        ai.ants = ants;

        println!("{}", ai.describe());

        ai
    }

    pub fn new(name: &'static str, pos: (i32, i32, i32)) -> Self {
        Colony {
            name,
            queen: Queen { pos, last_action: Instant::now(), eggs: vec![] },
            ants: vec![],
        }
    }
}
