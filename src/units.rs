use pathfinding::prelude::astar;
use rand::seq::SliceRandom;
use rand::{self, Rng};

use crate::buildings::{Building, FindHome};
use crate::coords::{self, Coords};
use crate::terrain::TileType;
use crate::terrain::{self, Terrain};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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
    pub action_queue: Vec<(ActionType, Coords)>,
    action_path: Option<(Vec<(usize, usize)>, i32)>,
    speed: i32,
    last_action_timer: i32,
}
pub fn display_action_queue(unit: Unit) {
    print!("ACTION LIST : ");
    for action in unit.action_queue {
        print!(
            "{}",
            match action.0 {
                ActionType::MOVE => "M",
                ActionType::DIG => "D",
                ActionType::WANDER => "W",
                _ => " ",
            }
        );
        print!(" <-- ");
    }
    println!("");
}

pub fn display_action(action: ActionType, coords: Coords) {
    print!("Going to ");
    print!(
        "{}",
        match action {
            ActionType::DIG => "DIG ",
            ActionType::MOVE => "MOVE ",
            ActionType::WANDER => "WANDER ",
            _ => "do something ",
        }
    );
    println!("at ({:?},{:?}) !", coords.x, coords.y);
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
        let race = match rng.gen_range(0..=3) {
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
            coords.x, coords.y, race_type_str
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
            action_queue: vec![],
            action_path: None,
            last_action_timer: 0,
            speed: match race {
                RaceType::HUMAN => 100,
                RaceType::ANT => 25,
                RaceType::ALIEN => 50,
            },
        }
    }

    fn get_movement_cost(&self, is_diagonal: bool, action: Option<ActionType>) -> i32 {
        match self.race {
            RaceType::ANT => {
                if let Some(ActionType::DIG) = action {
                    return 5;
                }
                if is_diagonal {
                    10
                } else {
                    10
                }
            }
            RaceType::HUMAN => {
                if let Some(ActionType::DIG) = action {
                    return 10;
                }
                if is_diagonal {
                    20
                } else {
                    10
                }
            }
            RaceType::ALIEN => {
                if let Some(ActionType::DIG) = action {
                    return 10;
                }
                if is_diagonal {
                    10
                } else {
                    20
                }
            }
        }
    }

    fn find_path(
        &self,
        start: (usize, usize),
        goal: (usize, usize),
        terrain: Terrain,
        action: Option<ActionType>,
    ) -> Option<(Vec<(usize, usize)>, i32)> {
        let (mut path, cost) = astar(
            &start,
            |&(x, y)| {
                // Définir les voisins cardinaux et diagonaux
                let directions = vec![
                    (x + 1, y, false),
                    (x.wrapping_sub(1), y, false),
                    (x, y + 1, false),
                    (x, y.wrapping_sub(1), false),
                    (x + 1, y + 1, true),
                    (x + 1, y.wrapping_sub(1), true),
                    (x.wrapping_sub(1), y + 1, true),
                    (x.wrapping_sub(1), y.wrapping_sub(1), true),
                ];

                directions
                    .into_iter()
                    .filter_map(|(nx, ny, is_diagonal)| match action {
                        Some(ActionType::MOVE) => {
                            if terrain.is_walkable(nx, ny) {
                                Some((
                                    (nx, ny),
                                    self.get_movement_cost(is_diagonal, Some(ActionType::MOVE)),
                                ))
                            } else {
                                None
                            }
                        }
                        Some(ActionType::DIG) => {
                            if terrain.is_walkable(nx, ny) || terrain.is_diggable(nx, ny) {
                                Some((
                                    (nx, ny),
                                    self.get_movement_cost(is_diagonal, Some(ActionType::DIG)),
                                ))
                            } else {
                                None
                            }
                        }
                        None => None,
                        _ => None,
                    })
                    .collect::<Vec<_>>()
            },
            |&(x, y)| {
                let dx = (x as isize - goal.0 as isize).abs();
                let dy = (y as isize - goal.1 as isize).abs();
                ((dx.pow(2) + dy.pow(2)) as f64).sqrt() as i32
            }, // Heuristique euclidien
            |&pos| pos == goal,
        )?;

        match action {
            Some(ActionType::MOVE) => {
                let mut new_path: Vec<(usize, usize)> = path.clone();
                for (x, y) in path.iter().rev() {
                    if terrain.is_diggable(*x, *y) && !terrain.is_walkable(*x, *y) {
                        new_path.pop();
                        break;
                    }
                }
                return Some((new_path, cost))
            }
            // Chercher la dernière position valide avant un obstacle
            Some(ActionType::DIG) => {
                let mut new_path: Vec<(usize, usize)> = vec![];

                for (x, y) in path.clone().iter().rev() {
                    if terrain.is_diggable(*x, *y) && !terrain.is_walkable(*x, *y) {
                        new_path.push((*x, *y));
                    } 
                }
                return Some((new_path, cost))
            }
            _ => todo!(),
        }
    }
}

pub trait Actions {
    fn do_action(&mut self, terrain: &mut Terrain);
    fn think(&mut self, terrain: &mut Terrain, delta_time: i32);
    fn r#move(&mut self, terrain: &mut Terrain, m: Coords);
    fn dig(&self);
    fn build(&self);
}

/// ??? Pas sur de ca
impl Actions for Vec<Unit> {
    fn do_action(&mut self, terrain: &mut Terrain) {
        for u in self {
            u.do_action(terrain)
        }
    }
    fn think(&mut self, terrain: &mut Terrain, delta_time: i32) {
        for u in self {
            u.think(terrain, delta_time)
        }
    }
    fn r#move(&mut self, terrain: &mut Terrain, m: Coords) {
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
    fn do_action(&mut self, terrain: &mut Terrain) {
        display_action_queue(self.clone());

        let action = self.action_queue.first().unwrap();
        match action {
            (ActionType::WAIT, _) => {
                display_action(action.0, action.1);
                match action {
                    (ActionType::WAIT, _) => {}
                    _ => {
                        self.action_queue.pop();
                        println!("Waiting ...");
                    }
                }
            }
            (ActionType::MOVE, coords) => {
                display_action(action.0, action.1);
                let start = self.coords.to_tuple();
                let goal = coords.to_tuple();
                if self.action_path == None {
                    let path = self.find_path(start, goal, terrain.clone(), Some(ActionType::MOVE));
                    let dig_path =
                        self.find_path(start, goal, terrain.clone(), Some(ActionType::DIG));
                    match (dig_path, path) {
                        (Some(dig_path), None) => {
                            // Reached a wall ! --> Stop !
                            println!("WALLLLLL -------------");
                            self.action_path = Some(dig_path);
                            self.action_queue.remove(0);
                            return;
                        }
                        (_, Some(path)) => {
                            // Found a way ! --> Go !
                            self.action_path = Some(path);
                            //   self.action_queue.remove(0);
                        }
                        (None, None) => {
                            self.action_queue.clear();
                            self.action_queue.push((ActionType::WANDER, self.coords));
                            return;
                        }
                    }
                }
                match self.action_path.clone() {
                    Some(mut path) => {
                        if path.0.len() > 0 {
                            let coords = Coords {
                                x: path.0[0].0 as i32,
                                y: path.0[0].1 as i32,
                            };

                            /*    let mut path = <Option<(Vec<(usize, usize)>, i32)> as Clone>::clone(
                                   &self.action_path.clone()
                               )
                               .unwrap();
                            */
                            path.0.remove(0);
                            self.action_path = Some(path);
                            self.r#move(terrain, coords);
                        } else {
                            self.action_queue.remove(0);
                            self.action_path = None;
                            self.action_queue
                                .insert(0, (ActionType::WANDER, self.coords));
                        }
                    }
                    None => {
                        self.action_queue.clear();
                        self.action_queue.push((ActionType::WANDER, self.coords));
                        println!("No path found to the goal.");
                        // DIG ?
                    }
                }
            }
            (ActionType::DIG, coords) => {
                display_action(action.0, action.1);

                let start = self.coords.to_tuple();
                let goal = coords.to_tuple();

                // Vérifier si la case est creusable avant d'essayer de trouver un chemin
                if !terrain.is_diggable(coords.x as usize, coords.y as usize)
                //   && !terrain.is_walkable(coords.x as usize, coords.y as usize)
                {
                    // Si la case n'est pas creusable, annuler l'action
                    println!("Impossible de creuser ici, action annulée");
                    self.action_queue.pop(); // Retirer l'action `DIG` de la queue
                    return; // Sortir sans ajouter d'autres actions
                }
                // Close enough to dig !!!
                let path = self.find_path(start, goal, terrain.clone(), Some(ActionType::MOVE));

                if self.coords.distance_in_tiles(coords) == 1 {
                    terrain.dig_radius(coords, 0);
                    println!("prout");
                }

                self.action_queue.remove(0);
                let dig_path = self.find_path(start, goal, terrain.clone(), Some(ActionType::DIG));
                match (dig_path, path) {
                    (Some(_), _) => {
                        self.action_queue.push((
                            ActionType::MOVE,
                            Coords {
                                x: goal.0 as i32,
                                y: goal.1 as i32,
                            },
                        ));

                        self.action_queue.push((
                            ActionType::DIG,
                            Coords {
                                x: goal.0 as i32,
                                y: goal.1 as i32,
                            },
                        ));

                        return;
                    }
                    (None, Some(mut path)) => {
                        self.action_queue.insert(
                            0,
                            (
                                ActionType::MOVE,
                                Coords {
                                    x: path.0.select_nth_unstable(0).1 .0 as i32,
                                    y: path.0.select_nth_unstable(0).1 .1 as i32,
                                },
                            ),
                        );
                        self.action_queue.push((
                            ActionType::DIG,
                            Coords {
                                x: goal.0 as i32,
                                y: goal.1 as i32,
                            },
                        ));
                        return;
                    }

                    (None, None) => {
                        self.action_path = None;
                        self.action_queue.clear();
                        self.action_queue
                            .insert(0, (ActionType::WANDER, self.coords));

                        // self.action_queue.remove(0);
                        return;
                    }
                }
            }

            (ActionType::WANDER, _) => {
                display_action(action.0, action.1);
                let home = terrain.buildings.find_home(self.race, terrain);

                match home {
                    Some(coords) => {
                        if self.coords.distance_to(&coords) > 10.0 && self.action_queue.len() == 1 {
                            self.action_path = None;
                            self.action_queue.remove(0);
                            self.action_queue.push((ActionType::MOVE, coords));
                        } else if self.action_queue.len() == 1 {
                            self.r#move(
                                terrain,
                                Coords {
                                    x: self.coords.x + random_direction(),
                                    y: self.coords.y + random_direction(),
                                },
                            );
                        } else {
                            self.action_queue.remove(0);
                        }
                    }
                    None => {
                        panic!("WHERE IS MY HOME ???")
                    }
                }
            }

            (ActionType::BUILD, _) => {}
            (_, _) => {}
        }
    }

    /// Decide what to do next
    fn think(&mut self, terrain: &mut Terrain, delta_time: i32) {
        self.last_action_timer += delta_time;
        if self.last_action_timer >= self.speed {
            if let Some(_) = self.action_queue.first() {
                self.do_action(terrain);
                self.last_action_timer = 0;
            }
        }
    }
    //move
    fn r#move(&mut self, terrain: &mut Terrain, m: Coords) {
        if terrain.get_data(m.x as usize, m.y as usize) == Some(TileType::AIR) {
            self.coords.x = m.x as i32;
            self.coords.y = m.y as i32;
        }
    }
    fn dig(&self) {
        todo!("unit.dig")
    }
    fn build(&self) {
        todo!("unit.build")
    }
}
