use std::time::Instant;

use crate::{ ant::{ queen::Queen, ColonyMember }, chunk::SEA_LEVEL };

pub struct Colony {
    pub name: &'static str,
    pub queen: Queen,
    pub ants: Vec<Box<dyn ColonyMember>>,
}
impl Colony {
    const TEST_NAME: &'static str = "Bel-o-Kan";

    pub const PLAYER:usize = 0;
    pub const AI:usize = 1;

    pub fn describe(&self) -> String {
        format!(
            "New colony at : {:?}, it is named {:?} and has a population of {:?} ants",
            self.queen.pos,
            self.name,
            self.ants.len()
        )
    }
    pub fn Bel_o_Kan() -> Self {
        let mut bok = Self::new(Self::TEST_NAME,(5, 0, (SEA_LEVEL as i32) - 15));

        let ants = vec![];

        bok.ants = ants;

        println!("{}", bok.describe());

        bok
    }

    pub fn computer_colony() -> Self {
        let mut ai = Self::new("Com-put-er", (-5, 0, (SEA_LEVEL as i32) - 15));

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
