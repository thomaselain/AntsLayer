use rand::seq::SliceRandom;
use rand::{self, Rng};

use crate::coords::Coords;
use crate::terrain::TileType;
use crate::terrain::{self, Terrain};

#[derive(Copy, Clone)]
pub enum RaceType {
    HUMAN,
    ANT,
    ALIEN,
}

#[derive(Copy, Clone)]
pub enum JobType {
    MINER,
    FARMER,
    FIGHTER,
    BUILDER,
}

#[derive(Copy, Clone, Debug)]
pub enum ActionType {
    WANDER,
    WAIT,
    MOVE,
    DIG,
    EAT,
    SLEEP,
    FIGHT,
    BUILD,
}

#[doc = "Unit.speed is thinking speed (in milliseconds) not moving speed"]
#[derive(Clone)]
pub struct Unit {
    pub color: u32,
    pub job: JobType,
    pub race: RaceType,
    pub coords: Coords,
    pub action_coords: Option<Coords>,
    pub action_queue: Vec<ActionType>,
    speed: i32,
    last_action_timer: i32,
}

fn random_direction() -> i32 {
    let choices = [-1, 0, 1];
    *choices.choose(&mut rand::thread_rng()).unwrap()
}

impl Unit {
    pub fn new() -> Unit {
        let coords = Coords {
            x: (terrain::WIDTH / 2) as i32,
            y: (terrain::HEIGHT / 2) as i32,
        };
        let mut rng = rand::thread_rng();
        let race = match rng.gen_range(0..3) {
            1 => RaceType::HUMAN,
            2 => RaceType::ANT,
            3 => RaceType::ALIEN,
            _ => RaceType::ANT,
        };
        let mut rng = rand::thread_rng();
        let job = match rng.gen_range(1..=4) {
            1 => JobType::MINER,
            2 => JobType::BUILDER,
            3 => JobType::FARMER,
            4 => JobType::FIGHTER,
            _ => JobType::MINER,
        };
        let race_type_str = match race {
            RaceType::ALIEN => "ALIEN",
            RaceType::ANT => "ANT",
            RaceType::HUMAN => "HUMAN",
        };

        println!(
            "New Unit (x : {:?} | y : {:?}) --> {:?}",
            coords.x,
            coords.y,
            race_type_str
        );

        Unit {
            color: match race {
                RaceType::ANT => 0xff0000ff,
                RaceType::ALIEN => 0x00ff00ff,
                RaceType::HUMAN => 0x0000ffff,
            },
            race,
            job,
            coords,
            action_coords: None,
            action_queue: vec![],
            last_action_timer: 0,
            speed: match race {
                RaceType::HUMAN => 0,
                RaceType::ANT => 0,
                RaceType::ALIEN => 0,
            },
        }
    }
}

pub trait Actions {
    fn do_action(&mut self, terrain: &Terrain, action: ActionType);
    fn think(&mut self, terrain: &Terrain, delta_time: i32);
    fn r#move(&mut self, terrain: &Terrain, m: Coords);
    fn dig(&self);
    fn build(&self);
}

/// ??? Pas sur de ca
impl Actions for Vec<Unit> {
    fn do_action(&mut self, terrain: &Terrain, action: ActionType) {
        for u in self {
            u.do_action(terrain, action)
        }
    }
    fn think(&mut self, terrain: &Terrain, delta_time: i32) {
        for u in self {
            u.think(terrain, delta_time)
        }
    }
    fn r#move(&mut self, terrain: &Terrain, m: Coords) {
        for u in self {
            u.r#move(terrain, m)
        }
    }
    fn dig(&self) {
        for u in self {
            u.dig()
        }
    }
    fn build(&self) {
        for u in self {
            u.build()
        }
    }
}
impl Actions for Unit {
    fn do_action(&mut self, terrain: &Terrain, action: ActionType) {
        match action {
            ActionType::MOVE => {
                self.r#move(terrain, Coords { x: 1, y: 0 });
            }
            ActionType::WANDER => {
                self.r#move(
                    terrain,
                    Coords {
                        x: random_direction(),
                        y: random_direction(),
                    },
                );
            }
            _ => {}
        }
    }
    /// Decide what to do next
    fn think(&mut self, terrain: &Terrain, delta_time: i32) {
        if self.last_action_timer >= self.speed {
            if let Some(action) = self.action_queue.first() {
                self.do_action(terrain, *action);
                self.last_action_timer = 0;
                self.action_queue.remove(0);
            }
        }
        if self.action_queue.is_empty() {
            self.action_queue.push(ActionType::WANDER);
        }
        self.last_action_timer += delta_time;
    }
    //move
    fn r#move(&mut self, terrain: &Terrain, m: Coords) {
        let target_x = (self.coords.x + m.x) as usize;
        let target_y = (self.coords.y + m.y) as usize;

        if terrain.get_data(target_x, target_y) == Some(TileType::AIR) {
            self.coords.x += m.x as i32;
            self.coords.y += m.y as i32;
        }
    }
    fn dig(&self) {
        todo!("unit.dig")
    }
    fn build(&self) {
        todo!("unit.build")
    }
}
